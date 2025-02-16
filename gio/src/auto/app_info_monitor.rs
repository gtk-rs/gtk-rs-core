// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ffi;
use glib::{
    object::ObjectType as _,
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GAppInfoMonitor")]
    pub struct AppInfoMonitor(Object<ffi::GAppInfoMonitor>);

    match fn {
        type_ => || ffi::g_app_info_monitor_get_type(),
    }
}

impl AppInfoMonitor {
    #[doc(alias = "g_app_info_monitor_get")]
    pub fn get() -> AppInfoMonitor {
        unsafe { from_glib_full(ffi::g_app_info_monitor_get()) }
    }

    #[doc(alias = "changed")]
    pub fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn changed_trampoline<F: Fn(&AppInfoMonitor) + 'static>(
            this: *mut ffi::GAppInfoMonitor,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"changed\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    changed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
