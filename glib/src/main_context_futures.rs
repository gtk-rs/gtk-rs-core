// Take a look at the license at the top of the repository in the LICENSE file.

use crate::translate::{from_glib_borrow, from_glib_full, mut_override, Borrowed, ToGlib};
use crate::ThreadGuard;
use futures_core::future::Future;
use futures_core::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
use futures_task::{FutureObj, LocalFutureObj, LocalSpawn, Spawn, SpawnError};
use futures_util::future::FutureExt;
use std::mem;
use std::pin;
use std::ptr;

use crate::MainContext;
use crate::MainLoop;
use crate::Priority;
use crate::Source;

// Wrapper around Send Futures and non-Send Futures that will panic
// if the non-Send Future is polled/dropped from a different thread
// than where this was created.
enum FutureWrapper {
    Send(FutureObj<'static, ()>),
    NonSend(ThreadGuard<LocalFutureObj<'static, ()>>),
}

impl Future for FutureWrapper {
    type Output = ();

    fn poll(self: pin::Pin<&mut Self>, ctx: &mut Context) -> Poll<()> {
        match self.get_mut() {
            FutureWrapper::Send(fut) => fut.poll_unpin(ctx),
            FutureWrapper::NonSend(fut) => fut.get_mut().poll_unpin(ctx),
        }
    }
}

// The TaskSource and WakerSource are split up as the TaskSource
// must only be finalized on the thread that owns the main context
// but the WakerSource is passed around to arbitrary threads for
// being able to wake up the TaskSource.
//
// The WakerSource is set up as a child source of the TaskSource, i.e.
// whenever it is ready also the TaskSource is ready.
#[repr(C)]
struct TaskSource {
    source: ffi::GSource,
    future: FutureWrapper,
    waker: Waker,
}

#[repr(C)]
struct WakerSource {
    source: ffi::GSource,
}

impl TaskSource {
    unsafe extern "C" fn dispatch(
        source: *mut ffi::GSource,
        callback: ffi::GSourceFunc,
        _user_data: ffi::gpointer,
    ) -> ffi::gboolean {
        let source = &mut *(source as *mut TaskSource);
        assert!(callback.is_none());

        // Poll the TaskSource and ensure we're never called again if the
        // contained Future resolved now.
        if let Poll::Ready(()) = source.poll() {
            ffi::G_SOURCE_REMOVE
        } else {
            ffi::G_SOURCE_CONTINUE
        }
    }

    unsafe extern "C" fn finalize(source: *mut ffi::GSource) {
        let source = source as *mut TaskSource;

        // This will panic if the future was a local future and is dropped from
        // a different thread than where it was created.
        ptr::drop_in_place(&mut (*source).future);

        // Drop the waker to unref the underlying GSource
        ptr::drop_in_place(&mut (*source).waker);
    }
}

impl WakerSource {
    unsafe fn clone_raw(waker: *const ()) -> RawWaker {
        static VTABLE: RawWakerVTable = RawWakerVTable::new(
            WakerSource::clone_raw,
            WakerSource::wake_raw,
            WakerSource::wake_by_ref_raw,
            WakerSource::drop_raw,
        );

        let waker = waker as *const ffi::GSource;
        ffi::g_source_ref(mut_override(waker));
        RawWaker::new(waker as *const (), &VTABLE)
    }

    unsafe fn wake_raw(waker: *const ()) {
        Self::wake_by_ref_raw(waker);
        Self::drop_raw(waker);
    }

    unsafe fn wake_by_ref_raw(waker: *const ()) {
        let waker = waker as *const ffi::GSource;
        ffi::g_source_set_ready_time(mut_override(waker), 0);
    }

    unsafe fn drop_raw(waker: *const ()) {
        let waker = waker as *const ffi::GSource;
        ffi::g_source_unref(mut_override(waker));
    }

    unsafe extern "C" fn dispatch(
        source: *mut ffi::GSource,
        _callback: ffi::GSourceFunc,
        _user_data: ffi::gpointer,
    ) -> ffi::gboolean {
        // Set ready-time to -1 so that we're not called again before
        // being woken up another time.
        ffi::g_source_set_ready_time(mut_override(source), -1);
        ffi::G_SOURCE_CONTINUE
    }
}

unsafe impl Send for TaskSource {}
unsafe impl Sync for TaskSource {}

unsafe impl Send for WakerSource {}
unsafe impl Sync for WakerSource {}

impl TaskSource {
    #[allow(clippy::new_ret_no_self)]
    fn new(priority: Priority, future: FutureWrapper) -> Source {
        unsafe {
            static TASK_SOURCE_FUNCS: ffi::GSourceFuncs = ffi::GSourceFuncs {
                check: None,
                prepare: None,
                dispatch: Some(TaskSource::dispatch),
                finalize: Some(TaskSource::finalize),
                closure_callback: None,
                closure_marshal: None,
            };

            static WAKER_SOURCE_FUNCS: ffi::GSourceFuncs = ffi::GSourceFuncs {
                check: None,
                prepare: None,
                dispatch: Some(WakerSource::dispatch),
                finalize: None,
                closure_callback: None,
                closure_marshal: None,
            };

            let source = ffi::g_source_new(
                mut_override(&TASK_SOURCE_FUNCS),
                mem::size_of::<TaskSource>() as u32,
            );

            let waker_source = ffi::g_source_new(
                mut_override(&WAKER_SOURCE_FUNCS),
                mem::size_of::<WakerSource>() as u32,
            );

            ffi::g_source_set_priority(source, priority.to_glib());
            ffi::g_source_add_child_source(source, waker_source);

            {
                let source = &mut *(source as *mut TaskSource);
                ptr::write(&mut source.future, future);

                // This creates a new reference to the waker source.
                let waker = Waker::from_raw(WakerSource::clone_raw(waker_source as *const ()));
                ptr::write(&mut source.waker, waker);
            }

            // Set ready time to 0 so that the source is immediately dispatched
            // for doing the initial polling. This will then either resolve the
            // future or register the waker wherever necessary.
            ffi::g_source_set_ready_time(waker_source, 0);

            // Unref the waker source, a strong reference to it is stored inside
            // the task source directly and inside the task source as child source.
            ffi::g_source_unref(waker_source);

            from_glib_full(source)
        }
    }

    fn poll(&mut self) -> Poll<()> {
        let source = &self.source as *const _;
        let executor: Borrowed<MainContext> =
            unsafe { from_glib_borrow(ffi::g_source_get_context(mut_override(source))) };

        assert!(
            executor.is_owner(),
            "Polling futures only allowed if the thread is owning the MainContext"
        );

        executor.with_thread_default(|| {
            let _enter = futures_executor::enter().unwrap();
            let mut context = Context::from_waker(&self.waker);

            // This will panic if the future was a local future and is called from
            // a different thread than where it was created.
            self.future.poll_unpin(&mut context)
        })
    }
}

impl MainContext {
    /// Spawn a new infallible `Future` on the main context.
    ///
    /// This can be called from any thread and will execute the future from the thread
    /// where main context is running, e.g. via a `MainLoop`.
    pub fn spawn<F: Future<Output = ()> + Send + 'static>(&self, f: F) {
        self.spawn_with_priority(crate::PRIORITY_DEFAULT, f);
    }

    /// Spawn a new infallible `Future` on the main context.
    ///
    /// The given `Future` does not have to be `Send`.
    ///
    /// This can be called only from the thread where the main context is running, e.g.
    /// from any other `Future` that is executed on this main context, or after calling
    /// `push_thread_default` or `acquire` on the main context.
    pub fn spawn_local<F: Future<Output = ()> + 'static>(&self, f: F) {
        self.spawn_local_with_priority(crate::PRIORITY_DEFAULT, f);
    }

    /// Spawn a new infallible `Future` on the main context, with a non-default priority.
    ///
    /// This can be called from any thread and will execute the future from the thread
    /// where main context is running, e.g. via a `MainLoop`.
    pub fn spawn_with_priority<F: Future<Output = ()> + Send + 'static>(
        &self,
        priority: Priority,
        f: F,
    ) {
        let f = FutureObj::new(Box::new(f));
        let source = TaskSource::new(priority, FutureWrapper::Send(f));
        source.attach(Some(&*self));
    }

    /// Spawn a new infallible `Future` on the main context, with a non-default priority.
    ///
    /// The given `Future` does not have to be `Send`.
    ///
    /// This can be called only from the thread where the main context is running, e.g.
    /// from any other `Future` that is executed on this main context, or after calling
    /// `push_thread_default` or `acquire` on the main context.
    pub fn spawn_local_with_priority<F: Future<Output = ()> + 'static>(
        &self,
        priority: Priority,
        f: F,
    ) {
        assert!(
            self.is_owner(),
            "Spawning local futures only allowed on the thread owning the MainContext"
        );
        let f = LocalFutureObj::new(Box::new(f));
        let source = TaskSource::new(priority, FutureWrapper::NonSend(ThreadGuard::new(f)));
        source.attach(Some(&*self));
    }

    /// Runs a new, infallible `Future` on the main context and block until it finished, returning
    /// the result of the `Future`.
    ///
    /// The given `Future` does not have to be `Send` or `'static`.
    ///
    /// This must only be called if no `MainLoop` or anything else is running on this specific main
    /// context.
    #[allow(clippy::transmute_ptr_to_ptr)]
    pub fn block_on<F: Future>(&self, f: F) -> F::Output {
        let mut res = None;
        let l = MainLoop::new(Some(&*self), false);
        let l_clone = l.clone();

        unsafe {
            let f = f.then(|r| {
                res = Some(r);
                l_clone.quit();
                futures_util::future::ready(())
            });

            // Super-unsafe: We transmute here to get rid of the 'static lifetime
            let f = LocalFutureObj::new(Box::new(f));
            let f: LocalFutureObj<'static, ()> = mem::transmute(f);

            let source = TaskSource::new(
                crate::PRIORITY_DEFAULT,
                FutureWrapper::NonSend(ThreadGuard::new(f)),
            );
            source.attach(Some(&*self));
        }

        l.run();

        res.unwrap()
    }
}

impl Spawn for MainContext {
    fn spawn_obj(&self, f: FutureObj<'static, ()>) -> Result<(), SpawnError> {
        let source = TaskSource::new(crate::PRIORITY_DEFAULT, FutureWrapper::Send(f));
        source.attach(Some(&*self));
        Ok(())
    }
}

impl LocalSpawn for MainContext {
    fn spawn_local_obj(&self, f: LocalFutureObj<'static, ()>) -> Result<(), SpawnError> {
        let source = TaskSource::new(
            crate::PRIORITY_DEFAULT,
            FutureWrapper::NonSend(ThreadGuard::new(f)),
        );
        source.attach(Some(&*self));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures_channel::oneshot;
    use futures_util::future::TryFutureExt;
    use std::sync::mpsc;
    use std::thread;

    #[test]
    fn test_spawn() {
        let c = MainContext::new();
        let l = crate::MainLoop::new(Some(&c), false);

        let (sender, receiver) = mpsc::channel();
        let (o_sender, o_receiver) = oneshot::channel();

        let l_clone = l.clone();
        c.spawn(
            o_receiver
                .and_then(move |()| {
                    sender.send(()).unwrap();
                    l_clone.quit();

                    futures_util::future::ok(())
                })
                .then(|res| {
                    assert!(res.is_ok());
                    futures_util::future::ready(())
                }),
        );

        thread::spawn(move || {
            l.run();
        });

        o_sender.send(()).unwrap();

        receiver.recv().unwrap();
    }

    #[test]
    fn test_spawn_local() {
        let c = MainContext::new();
        let l = crate::MainLoop::new(Some(&c), false);

        c.push_thread_default();
        let l_clone = l.clone();
        c.spawn_local(futures_util::future::lazy(move |_ctx| {
            l_clone.quit();
        }));

        l.run();

        c.pop_thread_default();
    }

    #[test]
    fn test_block_on() {
        let c = MainContext::new();

        let mut v = None;
        {
            let v = &mut v;

            let future = futures_util::future::lazy(|_ctx| {
                *v = Some(123);
                Ok::<i32, ()>(123)
            });

            let res = c.block_on(future);
            assert_eq!(res, Ok(123));
        }

        assert_eq!(v, Some(123));
    }
}
