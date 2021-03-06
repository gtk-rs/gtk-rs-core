// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::IsA;
use glib::translate::*;
use std::fmt;
use std::ptr;

glib::wrapper! {
    #[doc(alias = "GCancellable")]
    pub struct Cancellable(Object<ffi::GCancellable, ffi::GCancellableClass>);

    match fn {
        type_ => || ffi::g_cancellable_get_type(),
    }
}

impl Cancellable {
    pub const NONE: Option<&'static Cancellable> = None;

    #[doc(alias = "g_cancellable_new")]
    pub fn new() -> Cancellable {
        unsafe { from_glib_full(ffi::g_cancellable_new()) }
    }

    #[doc(alias = "g_cancellable_get_current")]
    #[doc(alias = "get_current")]
    pub fn current() -> Option<Cancellable> {
        unsafe { from_glib_none(ffi::g_cancellable_get_current()) }
    }
}

impl Default for Cancellable {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for Cancellable {}
unsafe impl Sync for Cancellable {}

pub trait CancellableExt: 'static {
    #[doc(alias = "g_cancellable_cancel")]
    fn cancel(&self);

    #[doc(alias = "g_cancellable_get_fd")]
    #[doc(alias = "get_fd")]
    fn fd(&self) -> i32;

    #[doc(alias = "g_cancellable_is_cancelled")]
    fn is_cancelled(&self) -> bool;

    //#[doc(alias = "g_cancellable_make_pollfd")]
    //fn make_pollfd(&self, pollfd: /*Ignored*/&mut glib::PollFD) -> bool;

    #[doc(alias = "g_cancellable_pop_current")]
    fn pop_current(&self);

    #[doc(alias = "g_cancellable_push_current")]
    fn push_current(&self);

    #[doc(alias = "g_cancellable_release_fd")]
    fn release_fd(&self);

    #[doc(alias = "g_cancellable_set_error_if_cancelled")]
    fn set_error_if_cancelled(&self) -> Result<(), glib::Error>;
}

impl<O: IsA<Cancellable>> CancellableExt for O {
    fn cancel(&self) {
        unsafe {
            ffi::g_cancellable_cancel(self.as_ref().to_glib_none().0);
        }
    }

    fn fd(&self) -> i32 {
        unsafe { ffi::g_cancellable_get_fd(self.as_ref().to_glib_none().0) }
    }

    fn is_cancelled(&self) -> bool {
        unsafe {
            from_glib(ffi::g_cancellable_is_cancelled(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    //fn make_pollfd(&self, pollfd: /*Ignored*/&mut glib::PollFD) -> bool {
    //    unsafe { TODO: call ffi:g_cancellable_make_pollfd() }
    //}

    fn pop_current(&self) {
        unsafe {
            ffi::g_cancellable_pop_current(self.as_ref().to_glib_none().0);
        }
    }

    fn push_current(&self) {
        unsafe {
            ffi::g_cancellable_push_current(self.as_ref().to_glib_none().0);
        }
    }

    fn release_fd(&self) {
        unsafe {
            ffi::g_cancellable_release_fd(self.as_ref().to_glib_none().0);
        }
    }

    fn set_error_if_cancelled(&self) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::g_cancellable_set_error_if_cancelled(
                self.as_ref().to_glib_none().0,
                &mut error,
            );
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}

impl fmt::Display for Cancellable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Cancellable")
    }
}
