// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{
    AsyncResult, Cancellable, DriveStartFlags, DriveStartStopType, Icon, MountOperation,
    MountUnmountFlags, Volume,
};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute, pin::Pin, ptr};

glib::wrapper! {
    #[doc(alias = "GDrive")]
    pub struct Drive(Interface<ffi::GDrive, ffi::GDriveIface>);

    match fn {
        type_ => || ffi::g_drive_get_type(),
    }
}

impl Drive {
    pub const NONE: Option<&'static Drive> = None;
}

pub trait DriveExt: 'static {
    #[doc(alias = "g_drive_can_eject")]
    fn can_eject(&self) -> bool;

    #[doc(alias = "g_drive_can_poll_for_media")]
    fn can_poll_for_media(&self) -> bool;

    #[doc(alias = "g_drive_can_start")]
    fn can_start(&self) -> bool;

    #[doc(alias = "g_drive_can_start_degraded")]
    fn can_start_degraded(&self) -> bool;

    #[doc(alias = "g_drive_can_stop")]
    fn can_stop(&self) -> bool;

    #[doc(alias = "g_drive_eject_with_operation")]
    fn eject_with_operation<P: FnOnce(Result<(), glib::Error>) + 'static>(
        &self,
        flags: MountUnmountFlags,
        mount_operation: Option<&impl IsA<MountOperation>>,
        cancellable: Option<&impl IsA<Cancellable>>,
        callback: P,
    );

    fn eject_with_operation_future(
        &self,
        flags: MountUnmountFlags,
        mount_operation: Option<&(impl IsA<MountOperation> + Clone + 'static)>,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>>;

    #[doc(alias = "g_drive_enumerate_identifiers")]
    fn enumerate_identifiers(&self) -> glib::StrV;

    #[doc(alias = "g_drive_get_icon")]
    #[doc(alias = "get_icon")]
    fn icon(&self) -> Icon;

    #[doc(alias = "g_drive_get_identifier")]
    #[doc(alias = "get_identifier")]
    fn identifier(&self, kind: &str) -> Option<glib::GString>;

    #[doc(alias = "g_drive_get_name")]
    #[doc(alias = "get_name")]
    fn name(&self) -> glib::GString;

    #[doc(alias = "g_drive_get_sort_key")]
    #[doc(alias = "get_sort_key")]
    fn sort_key(&self) -> Option<glib::GString>;

    #[doc(alias = "g_drive_get_start_stop_type")]
    #[doc(alias = "get_start_stop_type")]
    fn start_stop_type(&self) -> DriveStartStopType;

    #[doc(alias = "g_drive_get_symbolic_icon")]
    #[doc(alias = "get_symbolic_icon")]
    fn symbolic_icon(&self) -> Icon;

    #[doc(alias = "g_drive_get_volumes")]
    #[doc(alias = "get_volumes")]
    fn volumes(&self) -> glib::List<Volume>;

    #[doc(alias = "g_drive_has_media")]
    fn has_media(&self) -> bool;

    #[doc(alias = "g_drive_has_volumes")]
    fn has_volumes(&self) -> bool;

    #[doc(alias = "g_drive_is_media_check_automatic")]
    fn is_media_check_automatic(&self) -> bool;

    #[doc(alias = "g_drive_is_media_removable")]
    fn is_media_removable(&self) -> bool;

    #[doc(alias = "g_drive_is_removable")]
    fn is_removable(&self) -> bool;

    #[doc(alias = "g_drive_poll_for_media")]
    fn poll_for_media<P: FnOnce(Result<(), glib::Error>) + 'static>(
        &self,
        cancellable: Option<&impl IsA<Cancellable>>,
        callback: P,
    );

    fn poll_for_media_future(
        &self,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>>;

    #[doc(alias = "g_drive_start")]
    fn start<P: FnOnce(Result<(), glib::Error>) + 'static>(
        &self,
        flags: DriveStartFlags,
        mount_operation: Option<&impl IsA<MountOperation>>,
        cancellable: Option<&impl IsA<Cancellable>>,
        callback: P,
    );

    fn start_future(
        &self,
        flags: DriveStartFlags,
        mount_operation: Option<&(impl IsA<MountOperation> + Clone + 'static)>,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>>;

    #[doc(alias = "g_drive_stop")]
    fn stop<P: FnOnce(Result<(), glib::Error>) + 'static>(
        &self,
        flags: MountUnmountFlags,
        mount_operation: Option<&impl IsA<MountOperation>>,
        cancellable: Option<&impl IsA<Cancellable>>,
        callback: P,
    );

    fn stop_future(
        &self,
        flags: MountUnmountFlags,
        mount_operation: Option<&(impl IsA<MountOperation> + Clone + 'static)>,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>>;

    #[doc(alias = "changed")]
    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "disconnected")]
    fn connect_disconnected<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "eject-button")]
    fn connect_eject_button<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "stop-button")]
    fn connect_stop_button<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Drive>> DriveExt for O {
    fn can_eject(&self) -> bool {
        unsafe { from_glib(ffi::g_drive_can_eject(self.as_ref().to_glib_none().0)) }
    }

    fn can_poll_for_media(&self) -> bool {
        unsafe {
            from_glib(ffi::g_drive_can_poll_for_media(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn can_start(&self) -> bool {
        unsafe { from_glib(ffi::g_drive_can_start(self.as_ref().to_glib_none().0)) }
    }

    fn can_start_degraded(&self) -> bool {
        unsafe {
            from_glib(ffi::g_drive_can_start_degraded(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn can_stop(&self) -> bool {
        unsafe { from_glib(ffi::g_drive_can_stop(self.as_ref().to_glib_none().0)) }
    }

    fn eject_with_operation<P: FnOnce(Result<(), glib::Error>) + 'static>(
        &self,
        flags: MountUnmountFlags,
        mount_operation: Option<&impl IsA<MountOperation>>,
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
        unsafe extern "C" fn eject_with_operation_trampoline<
            P: FnOnce(Result<(), glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let _ =
                ffi::g_drive_eject_with_operation_finish(_source_object as *mut _, res, &mut error);
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
        let callback = eject_with_operation_trampoline::<P>;
        unsafe {
            ffi::g_drive_eject_with_operation(
                self.as_ref().to_glib_none().0,
                flags.into_glib(),
                mount_operation.map(|p| p.as_ref()).to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn eject_with_operation_future(
        &self,
        flags: MountUnmountFlags,
        mount_operation: Option<&(impl IsA<MountOperation> + Clone + 'static)>,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        let mount_operation = mount_operation.map(ToOwned::to_owned);
        Box_::pin(crate::GioFuture::new(
            self,
            move |obj, cancellable, send| {
                obj.eject_with_operation(
                    flags,
                    mount_operation.as_ref().map(::std::borrow::Borrow::borrow),
                    Some(cancellable),
                    move |res| {
                        send.resolve(res);
                    },
                );
            },
        ))
    }

    fn enumerate_identifiers(&self) -> glib::StrV {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::g_drive_enumerate_identifiers(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn icon(&self) -> Icon {
        unsafe { from_glib_full(ffi::g_drive_get_icon(self.as_ref().to_glib_none().0)) }
    }

    fn identifier(&self, kind: &str) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::g_drive_get_identifier(
                self.as_ref().to_glib_none().0,
                kind.to_glib_none().0,
            ))
        }
    }

    fn name(&self) -> glib::GString {
        unsafe { from_glib_full(ffi::g_drive_get_name(self.as_ref().to_glib_none().0)) }
    }

    fn sort_key(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::g_drive_get_sort_key(self.as_ref().to_glib_none().0)) }
    }

    fn start_stop_type(&self) -> DriveStartStopType {
        unsafe {
            from_glib(ffi::g_drive_get_start_stop_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn symbolic_icon(&self) -> Icon {
        unsafe {
            from_glib_full(ffi::g_drive_get_symbolic_icon(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn volumes(&self) -> glib::List<Volume> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::g_drive_get_volumes(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn has_media(&self) -> bool {
        unsafe { from_glib(ffi::g_drive_has_media(self.as_ref().to_glib_none().0)) }
    }

    fn has_volumes(&self) -> bool {
        unsafe { from_glib(ffi::g_drive_has_volumes(self.as_ref().to_glib_none().0)) }
    }

    fn is_media_check_automatic(&self) -> bool {
        unsafe {
            from_glib(ffi::g_drive_is_media_check_automatic(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_media_removable(&self) -> bool {
        unsafe {
            from_glib(ffi::g_drive_is_media_removable(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_removable(&self) -> bool {
        unsafe { from_glib(ffi::g_drive_is_removable(self.as_ref().to_glib_none().0)) }
    }

    fn poll_for_media<P: FnOnce(Result<(), glib::Error>) + 'static>(
        &self,
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
        unsafe extern "C" fn poll_for_media_trampoline<
            P: FnOnce(Result<(), glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let _ = ffi::g_drive_poll_for_media_finish(_source_object as *mut _, res, &mut error);
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
        let callback = poll_for_media_trampoline::<P>;
        unsafe {
            ffi::g_drive_poll_for_media(
                self.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn poll_for_media_future(
        &self,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        Box_::pin(crate::GioFuture::new(
            self,
            move |obj, cancellable, send| {
                obj.poll_for_media(Some(cancellable), move |res| {
                    send.resolve(res);
                });
            },
        ))
    }

    fn start<P: FnOnce(Result<(), glib::Error>) + 'static>(
        &self,
        flags: DriveStartFlags,
        mount_operation: Option<&impl IsA<MountOperation>>,
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
        unsafe extern "C" fn start_trampoline<P: FnOnce(Result<(), glib::Error>) + 'static>(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let _ = ffi::g_drive_start_finish(_source_object as *mut _, res, &mut error);
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
        let callback = start_trampoline::<P>;
        unsafe {
            ffi::g_drive_start(
                self.as_ref().to_glib_none().0,
                flags.into_glib(),
                mount_operation.map(|p| p.as_ref()).to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn start_future(
        &self,
        flags: DriveStartFlags,
        mount_operation: Option<&(impl IsA<MountOperation> + Clone + 'static)>,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        let mount_operation = mount_operation.map(ToOwned::to_owned);
        Box_::pin(crate::GioFuture::new(
            self,
            move |obj, cancellable, send| {
                obj.start(
                    flags,
                    mount_operation.as_ref().map(::std::borrow::Borrow::borrow),
                    Some(cancellable),
                    move |res| {
                        send.resolve(res);
                    },
                );
            },
        ))
    }

    fn stop<P: FnOnce(Result<(), glib::Error>) + 'static>(
        &self,
        flags: MountUnmountFlags,
        mount_operation: Option<&impl IsA<MountOperation>>,
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
        unsafe extern "C" fn stop_trampoline<P: FnOnce(Result<(), glib::Error>) + 'static>(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let _ = ffi::g_drive_stop_finish(_source_object as *mut _, res, &mut error);
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
        let callback = stop_trampoline::<P>;
        unsafe {
            ffi::g_drive_stop(
                self.as_ref().to_glib_none().0,
                flags.into_glib(),
                mount_operation.map(|p| p.as_ref()).to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn stop_future(
        &self,
        flags: MountUnmountFlags,
        mount_operation: Option<&(impl IsA<MountOperation> + Clone + 'static)>,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        let mount_operation = mount_operation.map(ToOwned::to_owned);
        Box_::pin(crate::GioFuture::new(
            self,
            move |obj, cancellable, send| {
                obj.stop(
                    flags,
                    mount_operation.as_ref().map(::std::borrow::Borrow::borrow),
                    Some(cancellable),
                    move |res| {
                        send.resolve(res);
                    },
                );
            },
        ))
    }

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn changed_trampoline<P: IsA<Drive>, F: Fn(&P) + 'static>(
            this: *mut ffi::GDrive,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Drive::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_disconnected<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn disconnected_trampoline<P: IsA<Drive>, F: Fn(&P) + 'static>(
            this: *mut ffi::GDrive,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Drive::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"disconnected\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    disconnected_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_eject_button<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn eject_button_trampoline<P: IsA<Drive>, F: Fn(&P) + 'static>(
            this: *mut ffi::GDrive,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Drive::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"eject-button\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    eject_button_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_stop_button<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn stop_button_trampoline<P: IsA<Drive>, F: Fn(&P) + 'static>(
            this: *mut ffi::GDrive,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Drive::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"stop-button\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    stop_button_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Drive {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Drive")
    }
}
