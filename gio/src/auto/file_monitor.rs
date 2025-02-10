// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ffi, File, FileMonitorEvent};
use glib::{
    object::ObjectType as _,
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GFileMonitor")]
    pub struct FileMonitor(Object<ffi::GFileMonitor, ffi::GFileMonitorClass>);

    match fn {
        type_ => || ffi::g_file_monitor_get_type(),
    }
}

impl FileMonitor {
    pub const NONE: Option<&'static FileMonitor> = None;
}

pub trait FileMonitorExt: IsA<FileMonitor> + 'static {
    #[doc(alias = "g_file_monitor_cancel")]
    fn cancel(&self) -> bool {
        unsafe { from_glib(ffi::g_file_monitor_cancel(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "g_file_monitor_emit_event")]
    fn emit_event(
        &self,
        child: &impl IsA<File>,
        other_file: &impl IsA<File>,
        event_type: FileMonitorEvent,
    ) {
        unsafe {
            ffi::g_file_monitor_emit_event(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
                other_file.as_ref().to_glib_none().0,
                event_type.into_glib(),
            );
        }
    }

    #[doc(alias = "g_file_monitor_is_cancelled")]
    #[doc(alias = "cancelled")]
    fn is_cancelled(&self) -> bool {
        unsafe {
            from_glib(ffi::g_file_monitor_is_cancelled(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_file_monitor_set_rate_limit")]
    #[doc(alias = "rate-limit")]
    fn set_rate_limit(&self, limit_msecs: i32) {
        unsafe {
            ffi::g_file_monitor_set_rate_limit(self.as_ref().to_glib_none().0, limit_msecs);
        }
    }

    #[doc(alias = "rate-limit")]
    fn rate_limit(&self) -> i32 {
        ObjectExt::property(self.as_ref(), "rate-limit")
    }

    #[doc(alias = "changed")]
    fn connect_changed<F: Fn(&Self, &File, Option<&File>, FileMonitorEvent) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn changed_trampoline<
            P: IsA<FileMonitor>,
            F: Fn(&P, &File, Option<&File>, FileMonitorEvent) + 'static,
        >(
            this: *mut ffi::GFileMonitor,
            file: *mut ffi::GFile,
            other_file: *mut ffi::GFile,
            event_type: ffi::GFileMonitorEvent,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                FileMonitor::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(file),
                Option::<File>::from_glib_borrow(other_file)
                    .as_ref()
                    .as_ref(),
                from_glib(event_type),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"changed".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "cancelled")]
    fn connect_cancelled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_cancelled_trampoline<
            P: IsA<FileMonitor>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GFileMonitor,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(FileMonitor::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::cancelled".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_cancelled_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "rate-limit")]
    fn connect_rate_limit_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_rate_limit_trampoline<
            P: IsA<FileMonitor>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GFileMonitor,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(FileMonitor::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::rate-limit".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_rate_limit_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<FileMonitor>> FileMonitorExt for O {}
