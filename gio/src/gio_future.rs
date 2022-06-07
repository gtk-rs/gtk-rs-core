// Take a look at the license at the top of the repository in the LICENSE file.

use futures_channel::oneshot;
use futures_core::{
    task::{Context, Poll},
    FusedFuture,
};
use std::future::Future;
use std::pin::{self, Pin};

use crate::prelude::*;
use crate::Cancellable;

use glib::thread_guard::ThreadGuard;

pub struct GioFuture<F, O, T, E> {
    obj: O,
    schedule_operation: Option<F>,
    cancellable: Option<Cancellable>,
    receiver: Option<oneshot::Receiver<Result<T, E>>>,
}

pub struct GioFutureResult<T, E> {
    sender: ThreadGuard<oneshot::Sender<Result<T, E>>>,
}

unsafe impl<T, E> Send for GioFutureResult<T, E> {}

impl<T, E> GioFutureResult<T, E> {
    pub fn resolve(self, res: Result<T, E>) {
        let _ = self.sender.into_inner().send(res);
    }
}

impl<F, O, T: 'static, E: 'static> GioFuture<F, O, T, E>
where
    O: Clone + 'static,
    F: FnOnce(&O, &Cancellable, GioFutureResult<T, E>) + 'static,
{
    pub fn new(obj: &O, schedule_operation: F) -> GioFuture<F, O, T, E> {
        GioFuture {
            obj: obj.clone(),
            schedule_operation: Some(schedule_operation),
            cancellable: Some(Cancellable::new()),
            receiver: None,
        }
    }
}

impl<F, O, T, E> Future for GioFuture<F, O, T, E>
where
    O: Clone + 'static,
    F: FnOnce(&O, &Cancellable, GioFutureResult<T, E>) + 'static,
{
    type Output = Result<T, E>;

    fn poll(mut self: pin::Pin<&mut Self>, ctx: &mut Context) -> Poll<Result<T, E>> {
        let GioFuture {
            ref obj,
            ref mut schedule_operation,
            ref mut cancellable,
            ref mut receiver,
            ..
        } = *self;

        if let Some(schedule_operation) = schedule_operation.take() {
            let main_context = glib::MainContext::ref_thread_default();
            assert!(
                main_context.is_owner(),
                "Spawning futures only allowed if the thread is owning the MainContext"
            );

            // Channel for sending back the GIO async operation
            // result to our future here.
            //
            // In theory, we could directly continue polling the
            // corresponding task from the GIO async operation
            // callback, however this would break at the very
            // least the g_main_current_source() API.
            let (send, recv) = oneshot::channel();

            schedule_operation(
                obj,
                cancellable.as_ref().unwrap(),
                GioFutureResult {
                    sender: ThreadGuard::new(send),
                },
            );

            *receiver = Some(recv);
        }

        // At this point we must have a receiver
        let res = {
            let receiver = receiver.as_mut().unwrap();
            Pin::new(receiver).poll(ctx)
        };

        match res {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Err(_)) => panic!("Async operation sender was unexpectedly closed"),
            Poll::Ready(Ok(v)) => {
                // Get rid of the reference to the cancellable and receiver
                let _ = cancellable.take();
                let _ = receiver.take();
                Poll::Ready(v)
            }
        }
    }
}

impl<F, O, T, E> FusedFuture for GioFuture<F, O, T, E>
where
    O: Clone + 'static,
    F: FnOnce(&O, &Cancellable, GioFutureResult<T, E>) + 'static,
{
    fn is_terminated(&self) -> bool {
        self.schedule_operation.is_none()
            && self
                .receiver
                .as_ref()
                .map_or(true, |receiver| receiver.is_terminated())
    }
}

impl<F, O, T, E> Drop for GioFuture<F, O, T, E> {
    fn drop(&mut self) {
        if let Some(cancellable) = self.cancellable.take() {
            cancellable.cancel();
        }
        let _ = self.receiver.take();
    }
}

impl<F, O, T, E> Unpin for GioFuture<F, O, T, E> {}
