// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ffi, AsyncResult, Cancellable};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, pin::Pin};

glib::wrapper! {
    #[doc(alias = "GPermission")]
    pub struct Permission(Object<ffi::GPermission, ffi::GPermissionClass>);

    match fn {
        type_ => || ffi::g_permission_get_type(),
    }
}

impl Permission {
    pub const NONE: Option<&'static Permission> = None;
}

pub trait PermissionExt: IsA<Permission> + 'static {
    #[doc(alias = "g_permission_acquire")]
    fn acquire<'a, P: IsA<Cancellable>>(
        &self,
        cancellable: impl Into<Option<&'a P>>,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::g_permission_acquire(
                self.as_ref().to_glib_none().0,
                cancellable
                    .into()
                    .as_ref()
                    .map(|p| p.as_ref())
                    .to_glib_none()
                    .0,
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

    #[doc(alias = "g_permission_acquire_async")]
    fn acquire_async<'a, P: IsA<Cancellable>, Q: FnOnce(Result<(), glib::Error>) + 'static>(
        &self,
        cancellable: impl Into<Option<&'a P>>,
        callback: Q,
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

        let user_data: Box_<glib::thread_guard::ThreadGuard<Q>> =
            Box_::new(glib::thread_guard::ThreadGuard::new(callback));
        unsafe extern "C" fn acquire_async_trampoline<
            Q: FnOnce(Result<(), glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = std::ptr::null_mut();
            let _ = ffi::g_permission_acquire_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<Q>> =
                Box_::from_raw(user_data as *mut _);
            let callback: Q = callback.into_inner();
            callback(result);
        }
        let callback = acquire_async_trampoline::<Q>;
        unsafe {
            ffi::g_permission_acquire_async(
                self.as_ref().to_glib_none().0,
                cancellable
                    .into()
                    .as_ref()
                    .map(|p| p.as_ref())
                    .to_glib_none()
                    .0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn acquire_future(
        &self,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        Box_::pin(crate::GioFuture::new(
            self,
            move |obj, cancellable, send| {
                obj.acquire_async(Some(cancellable), move |res| {
                    send.resolve(res);
                });
            },
        ))
    }

    #[doc(alias = "g_permission_get_allowed")]
    #[doc(alias = "get_allowed")]
    #[doc(alias = "allowed")]
    fn is_allowed(&self) -> bool {
        unsafe {
            from_glib(ffi::g_permission_get_allowed(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_permission_get_can_acquire")]
    #[doc(alias = "get_can_acquire")]
    #[doc(alias = "can-acquire")]
    fn can_acquire(&self) -> bool {
        unsafe {
            from_glib(ffi::g_permission_get_can_acquire(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_permission_get_can_release")]
    #[doc(alias = "get_can_release")]
    #[doc(alias = "can-release")]
    fn can_release(&self) -> bool {
        unsafe {
            from_glib(ffi::g_permission_get_can_release(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_permission_impl_update")]
    fn impl_update(&self, allowed: bool, can_acquire: bool, can_release: bool) {
        unsafe {
            ffi::g_permission_impl_update(
                self.as_ref().to_glib_none().0,
                allowed.into_glib(),
                can_acquire.into_glib(),
                can_release.into_glib(),
            );
        }
    }

    #[doc(alias = "g_permission_release")]
    fn release<'a, P: IsA<Cancellable>>(
        &self,
        cancellable: impl Into<Option<&'a P>>,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::g_permission_release(
                self.as_ref().to_glib_none().0,
                cancellable
                    .into()
                    .as_ref()
                    .map(|p| p.as_ref())
                    .to_glib_none()
                    .0,
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

    #[doc(alias = "g_permission_release_async")]
    fn release_async<'a, P: IsA<Cancellable>, Q: FnOnce(Result<(), glib::Error>) + 'static>(
        &self,
        cancellable: impl Into<Option<&'a P>>,
        callback: Q,
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

        let user_data: Box_<glib::thread_guard::ThreadGuard<Q>> =
            Box_::new(glib::thread_guard::ThreadGuard::new(callback));
        unsafe extern "C" fn release_async_trampoline<
            Q: FnOnce(Result<(), glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = std::ptr::null_mut();
            let _ = ffi::g_permission_release_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<Q>> =
                Box_::from_raw(user_data as *mut _);
            let callback: Q = callback.into_inner();
            callback(result);
        }
        let callback = release_async_trampoline::<Q>;
        unsafe {
            ffi::g_permission_release_async(
                self.as_ref().to_glib_none().0,
                cancellable
                    .into()
                    .as_ref()
                    .map(|p| p.as_ref())
                    .to_glib_none()
                    .0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn release_future(
        &self,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        Box_::pin(crate::GioFuture::new(
            self,
            move |obj, cancellable, send| {
                obj.release_async(Some(cancellable), move |res| {
                    send.resolve(res);
                });
            },
        ))
    }

    #[doc(alias = "allowed")]
    fn connect_allowed_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_allowed_trampoline<P: IsA<Permission>, F: Fn(&P) + 'static>(
            this: *mut ffi::GPermission,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Permission::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::allowed\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_allowed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "can-acquire")]
    fn connect_can_acquire_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_can_acquire_trampoline<
            P: IsA<Permission>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GPermission,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Permission::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::can-acquire\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_can_acquire_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "can-release")]
    fn connect_can_release_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_can_release_trampoline<
            P: IsA<Permission>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GPermission,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Permission::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::can-release\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_can_release_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<Permission>> PermissionExt for O {}
