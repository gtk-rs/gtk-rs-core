// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ffi, AsyncResult, Cancellable, Initable, NetworkConnectivity, SocketConnectable};
use glib::{
    object::ObjectType as _,
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, pin::Pin};

glib::wrapper! {
    #[doc(alias = "GNetworkMonitor")]
    pub struct NetworkMonitor(Interface<ffi::GNetworkMonitor, ffi::GNetworkMonitorInterface>) @requires Initable;

    match fn {
        type_ => || ffi::g_network_monitor_get_type(),
    }
}

impl NetworkMonitor {
    pub const NONE: Option<&'static NetworkMonitor> = None;

    #[doc(alias = "g_network_monitor_get_default")]
    #[doc(alias = "get_default")]
    #[allow(clippy::should_implement_trait)]
    pub fn default() -> NetworkMonitor {
        unsafe { from_glib_none(ffi::g_network_monitor_get_default()) }
    }
}

pub trait NetworkMonitorExt: IsA<NetworkMonitor> + 'static {
    #[doc(alias = "g_network_monitor_can_reach")]
    fn can_reach(
        &self,
        connectable: &impl IsA<SocketConnectable>,
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::g_network_monitor_can_reach(
                self.as_ref().to_glib_none().0,
                connectable.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_network_monitor_can_reach_async")]
    fn can_reach_async<P: FnOnce(Result<(), glib::Error>) + 'static>(
        &self,
        connectable: &impl IsA<SocketConnectable>,
        cancellable: Option<&impl IsA<Cancellable>>,
        callback: P,
    ) {
        let main_context = glib::MainContext::ref_thread_default();
        let is_main_context_owner = main_context.is_owner();
        let has_acquired_main_context = (!is_main_context_owner)
            .then(|| main_context.acquire().ok())
            .flatten();
        assert!(
            is_main_context_owner || has_acquired_main_context.is_some(),
            "Async operations only allowed if the thread is owning the MainContext"
        );

        let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
            Box_::new(glib::thread_guard::ThreadGuard::new(callback));
        unsafe extern "C" fn can_reach_async_trampoline<
            P: FnOnce(Result<(), glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = std::ptr::null_mut();
            let _ =
                ffi::g_network_monitor_can_reach_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = can_reach_async_trampoline::<P>;
        unsafe {
            ffi::g_network_monitor_can_reach_async(
                self.as_ref().to_glib_none().0,
                connectable.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn can_reach_future(
        &self,
        connectable: &(impl IsA<SocketConnectable> + Clone + 'static),
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        let connectable = connectable.clone();
        Box_::pin(crate::GioFuture::new(
            self,
            move |obj, cancellable, send| {
                obj.can_reach_async(&connectable, Some(cancellable), move |res| {
                    send.resolve(res);
                });
            },
        ))
    }

    #[doc(alias = "g_network_monitor_get_connectivity")]
    #[doc(alias = "get_connectivity")]
    fn connectivity(&self) -> NetworkConnectivity {
        unsafe {
            from_glib(ffi::g_network_monitor_get_connectivity(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_network_monitor_get_network_available")]
    #[doc(alias = "get_network_available")]
    #[doc(alias = "network-available")]
    fn is_network_available(&self) -> bool {
        unsafe {
            from_glib(ffi::g_network_monitor_get_network_available(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_network_monitor_get_network_metered")]
    #[doc(alias = "get_network_metered")]
    #[doc(alias = "network-metered")]
    fn is_network_metered(&self) -> bool {
        unsafe {
            from_glib(ffi::g_network_monitor_get_network_metered(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "network-changed")]
    fn connect_network_changed<F: Fn(&Self, bool) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn network_changed_trampoline<
            P: IsA<NetworkMonitor>,
            F: Fn(&P, bool) + 'static,
        >(
            this: *mut ffi::GNetworkMonitor,
            network_available: glib::ffi::gboolean,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                NetworkMonitor::from_glib_borrow(this).unsafe_cast_ref(),
                from_glib(network_available),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"network-changed".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    network_changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "connectivity")]
    fn connect_connectivity_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_connectivity_trampoline<
            P: IsA<NetworkMonitor>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GNetworkMonitor,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(NetworkMonitor::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::connectivity".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_connectivity_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "network-available")]
    fn connect_network_available_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_network_available_trampoline<
            P: IsA<NetworkMonitor>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GNetworkMonitor,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(NetworkMonitor::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::network-available".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_network_available_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "network-metered")]
    fn connect_network_metered_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_network_metered_trampoline<
            P: IsA<NetworkMonitor>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GNetworkMonitor,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(NetworkMonitor::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::network-metered".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_network_metered_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<NetworkMonitor>> NetworkMonitorExt for O {}
