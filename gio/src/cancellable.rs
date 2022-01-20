// Take a look at the license at the top of the repository in the LICENSE file.

use crate::prelude::*;
use crate::Cancellable;
use glib::{translate::*, IsA};
use std::num::NonZeroU64;

// rustdoc-stripper-ignore-next
/// The id of a cancelled handler that is returned by `CancellableExtManual::connect`. This type is
/// analogous to [`glib::SignalHandlerId`].
#[derive(Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct CancelledHandlerId(NonZeroU64);

impl CancelledHandlerId {
    // rustdoc-stripper-ignore-next
    /// Returns the internal signal handler ID.
    pub unsafe fn as_raw(&self) -> libc::c_ulong {
        self.0.get() as libc::c_ulong
    }
}

impl TryFromGlib<libc::c_ulong> for CancelledHandlerId {
    type Error = GlibNoneError;
    #[inline]
    unsafe fn try_from_glib(val: libc::c_ulong) -> Result<Self, GlibNoneError> {
        NonZeroU64::new(val as u64).map(Self).ok_or(GlibNoneError)
    }
}

pub trait CancellableExtManual {
    // rustdoc-stripper-ignore-next
    /// Convenience function to connect to the `signal::Cancellable::cancelled` signal. Also
    /// handles the race condition that may happen if the cancellable is cancelled right before
    /// connecting. If the operation is cancelled from another thread, `callback` will be called
    /// in the thread that cancelled the operation, not the thread that is running the operation.
    /// This may be the main thread, so the callback should not do something that can block.
    ///
    /// `callback` is called at most once, either directly at the time of the connect if `self` is
    /// already cancelled, or when `self` is cancelled in some thread.
    ///
    /// Since GLib 2.40, the lock protecting `self` is not held when `callback` is invoked. This
    /// lifts a restriction in place for earlier GLib versions which now makes it easier to write
    /// cleanup code that unconditionally invokes e.g.
    /// [`CancellableExt::cancel()`][crate::prelude::CancellableExt::cancel()].
    ///
    /// # Returns
    ///
    /// The id of the signal handler or `None` if `self` has already been cancelled.
    #[doc(alias = "g_cancellable_connect")]
    fn connect_cancelled<F: FnOnce(&Self) + Send + 'static>(
        &self,
        callback: F,
    ) -> Option<CancelledHandlerId>;
    // rustdoc-stripper-ignore-next
    /// Disconnects a handler from a cancellable instance. Additionally, in the event that a signal
    /// handler is currently running, this call will block until the handler has finished. Calling
    /// this function from a callback registered with [`Self::connect_cancelled`] will therefore
    /// result in a deadlock.
    ///
    /// This avoids a race condition where a thread cancels at the same time as the cancellable
    /// operation is finished and the signal handler is removed.
    #[doc(alias = "g_cancellable_disconnect")]
    fn disconnect_cancelled(&self, id: CancelledHandlerId);
}

impl<O: IsA<Cancellable>> CancellableExtManual for O {
    #[doc(alias = "g_cancellable_connect")]
    fn connect_cancelled<F: FnOnce(&Self) + Send + 'static>(
        &self,
        callback: F,
    ) -> Option<CancelledHandlerId> {
        unsafe extern "C" fn connect_trampoline<P: IsA<Cancellable>, F: FnOnce(&P)>(
            this: *mut ffi::GCancellable,
            callback: glib::ffi::gpointer,
        ) {
            let callback: &mut Option<F> = &mut *(callback as *mut Option<F>);
            let callback = callback
                .take()
                .expect("Cancellable::cancel() closure called multiple times");
            callback(Cancellable::from_glib_borrow(this).unsafe_cast_ref())
        }

        unsafe extern "C" fn destroy_closure<F>(ptr: glib::ffi::gpointer) {
            Box::<Option<F>>::from_raw(ptr as *mut _);
        }

        let callback: Box<Option<F>> = Box::new(Some(callback));
        unsafe {
            from_glib(ffi::g_cancellable_connect(
                self.as_ptr() as *mut _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    connect_trampoline::<Self, F> as *const (),
                )),
                Box::into_raw(callback) as *mut _,
                Some(destroy_closure::<F>),
            ))
        }
    }
    #[doc(alias = "g_cancellable_disconnect")]
    fn disconnect_cancelled(&self, id: CancelledHandlerId) {
        unsafe { ffi::g_cancellable_disconnect(self.as_ptr() as *mut _, id.as_raw()) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cancellable_callback() {
        let c = Cancellable::new();
        let id = c.connect_cancelled(|_| {});
        c.cancel(); // if it doesn't crash at this point, then we're good to go!
        c.disconnect_cancelled(id.unwrap());
    }
}
