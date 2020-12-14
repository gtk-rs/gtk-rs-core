// Take a look at the license at the top of the repository in the LICENSE file.

//! `IMPL` Low level signal support.

use crate::object::ObjectType;
use crate::translate::{from_glib, FromGlib, ToGlib, ToGlibPtr};
use ffi::{gboolean, gpointer};
use gobject_ffi::{self, GCallback};
use libc::{c_char, c_ulong, c_void};
use std::mem;
use std::num::NonZeroU64;

/// The id of a signal that is returned by `connect`.
#[derive(Debug, Eq, PartialEq)]
pub struct SignalHandlerId(NonZeroU64);

impl ToGlib for SignalHandlerId {
    type GlibType = c_ulong;

    #[inline]
    fn to_glib(&self) -> c_ulong {
        self.0.get() as c_ulong
    }
}

impl FromGlib<c_ulong> for SignalHandlerId {
    #[inline]
    unsafe fn from_glib(val: c_ulong) -> SignalHandlerId {
        assert_ne!(val, 0);
        SignalHandlerId(NonZeroU64::new_unchecked(val as u64))
    }
}

/// Whether to propagate the signal to the default handler.
///
/// Don't inhibit default handlers without a reason, they're usually helpful.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Inhibit(pub bool);

#[doc(hidden)]
impl ToGlib for Inhibit {
    type GlibType = gboolean;

    #[inline]
    fn to_glib(&self) -> gboolean {
        self.0.to_glib()
    }
}

pub unsafe fn connect_raw<F>(
    receiver: *mut gobject_ffi::GObject,
    signal_name: *const c_char,
    trampoline: GCallback,
    closure: *mut F,
) -> SignalHandlerId {
    unsafe extern "C" fn destroy_closure<F>(ptr: *mut c_void, _: *mut gobject_ffi::GClosure) {
        // destroy
        Box::<F>::from_raw(ptr as *mut _);
    }
    assert_eq!(mem::size_of::<*mut F>(), mem::size_of::<gpointer>());
    assert!(trampoline.is_some());
    let handle = gobject_ffi::g_signal_connect_data(
        receiver,
        signal_name,
        trampoline,
        closure as *mut _,
        Some(destroy_closure::<F>),
        0,
    );
    assert!(handle > 0);
    from_glib(handle)
}

pub fn signal_handler_block<T: ObjectType>(instance: &T, handler_id: &SignalHandlerId) {
    unsafe {
        gobject_ffi::g_signal_handler_block(
            instance.as_object_ref().to_glib_none().0,
            handler_id.to_glib(),
        );
    }
}

pub fn signal_handler_unblock<T: ObjectType>(instance: &T, handler_id: &SignalHandlerId) {
    unsafe {
        gobject_ffi::g_signal_handler_unblock(
            instance.as_object_ref().to_glib_none().0,
            handler_id.to_glib(),
        );
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn signal_handler_disconnect<T: ObjectType>(instance: &T, handler_id: SignalHandlerId) {
    unsafe {
        gobject_ffi::g_signal_handler_disconnect(
            instance.as_object_ref().to_glib_none().0,
            handler_id.to_glib(),
        );
    }
}

pub fn signal_stop_emission_by_name<T: ObjectType>(instance: &T, signal_name: &str) {
    unsafe {
        gobject_ffi::g_signal_stop_emission_by_name(
            instance.as_object_ref().to_glib_none().0,
            signal_name.to_glib_none().0,
        );
    }
}
