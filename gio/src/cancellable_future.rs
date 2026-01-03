// Take a look at the license at the top of the repository in the LICENSE file.

use std::{
    fmt::{Debug, Display},
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use pin_project_lite::pin_project;

use crate::{prelude::*, Cancellable, CancelledHandlerId, IOErrorEnum};

// rustdoc-stripper-ignore-next
/// Indicator that the [`CancellableFuture`] was cancelled.
pub struct Cancelled;

pin_project! {
    // rustdoc-stripper-ignore-next
    /// A future which can be cancelled via [`Cancellable`].
    ///
    /// It can be used to cancel one or multiple Gio futures that support
    /// internal cancellation via [`Cancellable`] by using this future to
    /// execute cancellable promises that are created (implicitly or explicitly)
    /// via [`GioFuture`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use futures::FutureExt;
    /// # use gio::prelude::*;
    /// # use gio::CancellableFuture;
    /// let l = glib::MainLoop::new(None, false);
    /// let c = gio::Cancellable::new();
    ///
    /// l.context().spawn_local(CancellableFuture::new(async { 42 }, c.clone()).map(|_| ()));
    /// c.cancel();
    ///
    /// ```
    ///
    /// As said the [`CancellableFuture`] can be used to handle Gio futures,
    /// relying on an actual [`Cancellable`] instance or with a new one.
    ///
    /// ```no_run
    /// # use futures::FutureExt;
    /// # use gio::prelude::*;
    /// # use gio::CancellableFuture;
    /// # async {
    /// CancellableFuture::new(
    ///   async {
    ///     let file = gio::File::for_path("/dev/null");
    ///     let file_info = file
    ///       .query_info_future(
    ///         gio::FILE_ATTRIBUTE_STANDARD_NAME,
    ///         gio::FileQueryInfoFlags::NONE,
    ///         glib::Priority::default(),
    ///       )
    ///       .await?;
    ///
    ///     // Sub-cancellable chains are also working as expected.
    ///     // The new cancellable will have its own scope, but will also be
    ///     // cancelled when the parent cancellable is cancelled.
    ///     let io_stream = CancellableFuture::new(
    ///       file.open_readwrite_future(glib::Priority::default()),
    ///       gio::Cancellable::new(),
    ///     )
    ///     .await??;
    ///     // [...]
    ///     Ok::<bool, glib::Error>(true)
    ///   },
    ///   gio::Cancellable::new(),
    /// )
    /// .await
    /// # };
    /// ```
    pub struct CancellableFuture<F> {
        #[pin]
        future: F,

        cancellable: Cancellable,
        connection_id: Option<CancelledHandlerId>,

        waker: std::sync::Arc<std::sync::Mutex<Option<std::task::Waker>>>,
    }

    impl<F> PinnedDrop for CancellableFuture<F> {
        fn drop(this: Pin<&mut Self>) {
            let this = this.project();
            if let Some(connection_id) = this.connection_id.take() {
                this.cancellable.disconnect_cancelled(connection_id);
            }
        }
    }
}

impl<F> CancellableFuture<F> {
    // rustdoc-stripper-ignore-next
    /// Creates a new `CancellableFuture` using a [`Cancellable`].
    ///
    /// When [`cancel`](CancellableExt::cancel) is called, the future will complete
    /// immediately without making any further progress. In such a case, an error
    /// will be returned by this future (i.e., [`Cancelled`]).
    pub fn new(future: F, cancellable: Cancellable) -> Self {
        let waker = std::sync::Arc::new(std::sync::Mutex::new(None::<std::task::Waker>));
        let connection_id = cancellable.connect_cancelled(glib::clone!(
            #[strong]
            waker,
            move |_| {
                if let Some(waker) = waker.lock().unwrap().take() {
                    waker.wake();
                }
            }
        ));

        Self {
            future,
            cancellable,
            connection_id,
            waker,
        }
    }

    // rustdoc-stripper-ignore-next
    /// Checks whether the future has been cancelled.
    ///
    /// This is a shortcut for `self.cancellable().is_cancelled()`
    ///
    /// Note that all this method indicates is whether [`cancel`](CancellableExt::cancel)
    /// was called. This means that it will return true even if:
    ///   * `cancel` was called after the future had completed.
    ///   * `cancel` was called while the future was being polled.
    #[inline]
    pub fn is_cancelled(&self) -> bool {
        self.cancellable.is_cancelled()
    }

    // rustdoc-stripper-ignore-next
    /// Returns the inner [`Cancellable`] associated during creation.
    #[inline]
    pub fn cancellable(&self) -> &Cancellable {
        &self.cancellable
    }
}

impl<F> Future for CancellableFuture<F>
where
    F: Future,
{
    type Output = Result<<F as Future>::Output, Cancelled>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.cancellable.is_cancelled() {
            // XXX: Whenever we want to break the API, we should return here only
            // self.cancellable.set_error_if_cancelled() value.
            return std::task::Poll::Ready(Err(Cancelled));
        }

        let mut waker = self.waker.lock().unwrap();
        if waker.is_none() {
            *waker = Some(cx.waker().clone());
        }
        drop(waker);

        let this = self.as_mut().project();
        match this.future.poll(cx) {
            Poll::Ready(out) => Poll::Ready(Ok(out)),
            Poll::Pending => Poll::Pending,
        }
    }
}

impl From<Cancelled> for glib::Error {
    fn from(_: Cancelled) -> Self {
        glib::Error::new(IOErrorEnum::Cancelled, "Operation was cancelled")
    }
}

impl std::error::Error for Cancelled {}

impl Debug for Cancelled {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Operation was cancelled")
    }
}

impl Display for Cancelled {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

#[cfg(test)]
mod tests {
    use std::future::Future;
    use std::time::Duration;

    use super::{Cancellable, CancellableFuture, Cancelled};
    use crate::{prelude::*, spawn_blocking};

    async fn cancel_after_timeout(duration: Duration, cancellable: Cancellable) {
        glib::timeout_future_with_priority(glib::Priority::default(), duration).await;
        cancellable.cancel();
    }

    async fn cancel_after_sleep_in_thread(duration: Duration, cancellable: Cancellable) {
        spawn_blocking(move || {
            std::thread::sleep(duration);
            cancellable.cancel();
        })
        .await
        .unwrap()
    }

    fn async_begin<P: FnOnce(Result<(), glib::Error>) + Send + 'static>(
        duration: Duration,
        must_be_cancelled_on_begin: bool,
        must_be_cancelled_after_sleep: bool,
        cancellable: &Cancellable,
        callback: P,
    ) {
        // We do not use std::thread here since we want to simulate what C code normally does.
        // Also not using spawn_blocking() directly, since we want to have the full control
        // for the test case.
        let callback = Box::new(callback);
        let task = unsafe {
            crate::Task::<bool>::new(None::<&glib::Binding>, Some(cancellable), move |t, _| {
                let cancellable = t.cancellable().unwrap();
                let ret = t.propagate();
                println!(
                    "Task callback, returning {:?} - cancelled {}",
                    ret,
                    cancellable.is_cancelled()
                );
                assert_eq!(cancellable.is_cancelled(), must_be_cancelled_after_sleep);
                match ret {
                    Err(e) => callback(Err(e)),
                    Ok(_) => callback(Ok(())),
                };
            })
        };

        task.run_in_thread(move |task, _: Option<&glib::Binding>, cancellable| {
            let cancellable = cancellable.unwrap();
            let func = || {
                println!(
                    "Task thread started, cancelled {} - want {}",
                    cancellable.is_cancelled(),
                    must_be_cancelled_on_begin
                );
                assert_eq!(cancellable.is_cancelled(), must_be_cancelled_on_begin);
                std::thread::sleep(duration);
                assert_eq!(cancellable.is_cancelled(), must_be_cancelled_after_sleep);
                println!(
                    "Task thread done, cancelled {} - want {}",
                    cancellable.is_cancelled(),
                    must_be_cancelled_after_sleep
                )
            };

            if let Err(e) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(func)) {
                std::panic::resume_unwind(e);
            }

            unsafe {
                task.return_result(match cancellable.set_error_if_cancelled() {
                    Err(e) => Err(e),
                    Ok(_) => Ok(true),
                });
            }
        });
    }

    fn async_future(
        duration: Duration,
        must_be_cancelled_on_begin: bool,
        must_be_cancelled_after_sleep: bool,
    ) -> std::pin::Pin<Box<dyn Future<Output = Result<(), glib::Error>> + 'static>> {
        Box::pin(crate::GioFuture::new(&(), move |_, cancellable, send| {
            async_begin(
                duration,
                must_be_cancelled_on_begin,
                must_be_cancelled_after_sleep,
                cancellable,
                move |res| send.resolve(res),
            );
        }))
    }

    #[test]
    fn cancellable_future_ok() {
        let ctx = glib::MainContext::new();
        let c = Cancellable::new();

        let future = {
            ctx.spawn_local(async {
                let cancellable_future = CancellableFuture::new(async { 42 }, c);
                assert!(!cancellable_future.is_cancelled());

                let result = cancellable_future.await;
                assert!(matches!(result, Ok(42)));
            })
        };

        ctx.block_on(future).unwrap()
    }

    #[test]
    fn cancellable_future_cancel() {
        let ctx = glib::MainContext::new();
        let c = Cancellable::new();

        let future = {
            let c = c.clone();
            ctx.spawn_local(async move {
                let cancellable_future = CancellableFuture::new(std::future::pending::<()>(), c);

                let result = cancellable_future.await;
                assert!(matches!(result, Err(Cancelled)));
            })
        };

        std::thread::spawn(move || c.cancel()).join().unwrap();

        ctx.block_on(future).unwrap();
    }

    #[test]
    fn cancellable_future_delayed_cancel_local() {
        let ctx = glib::MainContext::new();
        let c = Cancellable::new();

        let (r1, r2) = ctx
            .block_on(ctx.spawn_local({
                futures_util::future::join(
                    CancellableFuture::new(std::future::pending::<()>(), c.clone()),
                    cancel_after_timeout(Duration::from_millis(300), c.clone()),
                )
            }))
            .expect("futures must be executed");

        assert!(matches!(r1, Err(Cancelled)));
        assert!(matches!(r2, ()));
    }

    #[test]
    fn cancellable_future_delayed_cancel_from_other_thread() {
        let ctx = glib::MainContext::new();
        let c = Cancellable::new();

        let (r1, r2) = ctx
            .block_on(ctx.spawn_local({
                futures_util::future::join(
                    CancellableFuture::new(std::future::pending::<()>(), c.clone()),
                    cancel_after_sleep_in_thread(Duration::from_millis(300), c.clone()),
                )
            }))
            .expect("futures must be executed");

        assert!(matches!(r1, Err(Cancelled)));
        assert!(matches!(r2, ()));
    }

    #[test]
    fn cancellable_future_immediate_cancel_with_gio_future() {
        let ctx = glib::MainContext::new();
        let c = Cancellable::new();

        async fn async_chain() -> Result<(), glib::Error> {
            async_future(Duration::from_millis(250), true, true).await?;
            async_future(Duration::from_secs(9999999), true, true).await
        }

        c.cancel();

        let result = ctx
            .block_on(ctx.spawn_local({
                CancellableFuture::new(
                    futures_util::future::join5(
                        async_chain(),
                        async_chain(),
                        async_chain(),
                        async_chain(),
                        CancellableFuture::new(async_chain(), Cancellable::new()),
                    ),
                    c.clone(),
                )
            }))
            .expect("futures must be executed");

        assert!(matches!(result, Err(Cancelled)));
    }

    #[test]
    fn cancellable_future_delayed_cancel_with_gio_future() {
        let ctx = glib::MainContext::new();
        let c = Cancellable::new();

        async fn async_chain() -> Result<(), glib::Error> {
            async_future(Duration::from_millis(250), false, true).await?;
            async_future(Duration::from_secs(9999999), true, true).await
        }

        let (result, _, _) = ctx
            .block_on(ctx.spawn_local({
                futures_util::future::join3(
                    CancellableFuture::new(
                        futures_util::future::join5(
                            async_chain(),
                            async_chain(),
                            async_chain(),
                            async_chain(),
                            CancellableFuture::new(async_chain(), Cancellable::new()),
                        ),
                        c.clone(),
                    ),
                    cancel_after_sleep_in_thread(Duration::from_millis(100), c.clone()),
                    // Let's wait a bit more to ensure that more events are processed
                    // by the loop. Not required, but it simulates a more real
                    // scenario.
                    glib::timeout_future(Duration::from_millis(350)),
                )
            }))
            .expect("futures must be executed");

        assert!(matches!(result, Err(Cancelled)));
    }

    #[test]
    fn cancellable_future_late_cancel_with_gio_future() {
        let ctx = glib::MainContext::new();
        let c = Cancellable::new();

        async fn async_chain() -> Result<(), glib::Error> {
            async_future(Duration::from_millis(100), false, false).await?;
            async_future(Duration::from_millis(100), false, false).await
        }

        let results = ctx
            .block_on(ctx.spawn_local(async move {
                let ret = CancellableFuture::new(
                    futures_util::future::join5(
                        async_chain(),
                        async_chain(),
                        async_chain(),
                        async_chain(),
                        CancellableFuture::new(async_chain(), Cancellable::new()),
                    ),
                    c.clone(),
                )
                .await;

                c.cancel();
                ret
            }))
            .expect("futures must be executed");

        assert!(results.is_ok());

        let r1 = results.unwrap();
        assert!(matches!(r1.0, Ok(())));
        assert!(matches!(r1.1, Ok(())));
        assert!(matches!(r1.2, Ok(())));
        assert!(matches!(r1.3, Ok(())));
        assert!(matches!(r1.4.unwrap(), Ok(())));
    }
}
