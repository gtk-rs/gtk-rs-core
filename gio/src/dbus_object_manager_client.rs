// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{
    ffi, BusType, Cancellable, DBusConnection, DBusObjectManagerClient,
    DBusObjectManagerClientFlags, GioFuture,
};
use glib::object::IsA;
use glib::translate::{from_glib_borrow, from_glib_full, Borrowed, IntoGlib as _, ToGlibPtr as _};
use std::future::Future;
use std::pin::Pin;

type DBusProxyTypeFn =
    Box<dyn Fn(&DBusObjectManagerClient, &str, Option<&str>) -> glib::types::Type + 'static>;

impl DBusObjectManagerClient {
    #[doc(alias = "g_dbus_object_manager_client_new")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new<P: FnOnce(Result<DBusObjectManagerClient, glib::Error>) + 'static>(
        connection: &DBusConnection,
        flags: DBusObjectManagerClientFlags,
        name: &str,
        object_path: &str,
        get_proxy_type_func: Option<DBusProxyTypeFn>,
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

        unsafe extern "C" fn get_proxy_type_func_func(
            manager: *mut ffi::GDBusObjectManagerClient,
            object_path: *const std::ffi::c_char,
            interface_name: *const std::ffi::c_char,
            data: glib::ffi::gpointer,
        ) -> glib::ffi::GType {
            let manager = from_glib_borrow(manager);
            let object_path: Borrowed<glib::GString> = from_glib_borrow(object_path);
            let interface_name: Borrowed<Option<glib::GString>> = from_glib_borrow(interface_name);
            let callback = &*(data as *mut Option<DBusProxyTypeFn>);
            if let Some(ref callback) = *callback {
                callback(
                    &manager,
                    object_path.as_str(),
                    (*interface_name).as_ref().map(|s| s.as_str()),
                )
            } else {
                panic!("cannot get closure...")
            }
            .into_glib()
        }

        unsafe extern "C" fn get_proxy_type_destroy_notify_func(data: glib::ffi::gpointer) {
            let _callback = Box::from_raw(data as *mut Option<DBusProxyTypeFn>);
        }

        unsafe extern "C" fn new_trampoline<
            P: FnOnce(Result<DBusObjectManagerClient, glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = std::ptr::null_mut();
            let ret = ffi::g_dbus_object_manager_client_new_finish(res, &mut error);
            let result = if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box<glib::thread_guard::ThreadGuard<P>> =
                Box::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }

        let get_proxy_type_user_data = Box::new(get_proxy_type_func);
        let get_proxy_type_func = if get_proxy_type_user_data.is_some() {
            Some(get_proxy_type_func_func as _)
        } else {
            None
        };
        let get_proxy_type_destroy_notify = if get_proxy_type_user_data.is_some() {
            Some(get_proxy_type_destroy_notify_func as _)
        } else {
            None
        };

        let user_data: Box<glib::thread_guard::ThreadGuard<P>> =
            Box::new(glib::thread_guard::ThreadGuard::new(callback));
        let callback = new_trampoline::<P>;

        unsafe {
            ffi::g_dbus_object_manager_client_new(
                connection.to_glib_none().0,
                flags.into_glib(),
                name.to_glib_none().0,
                object_path.to_glib_none().0,
                get_proxy_type_func,
                Box::into_raw(get_proxy_type_user_data) as *mut _,
                get_proxy_type_destroy_notify,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn new_future(
        connection: &DBusConnection,
        flags: DBusObjectManagerClientFlags,
        name: &str,
        object_path: &str,
        get_proxy_type_func: Option<DBusProxyTypeFn>,
    ) -> Pin<Box<dyn Future<Output = Result<DBusObjectManagerClient, glib::Error>> + 'static>> {
        let connection = connection.clone();
        let name = String::from(name);
        let object_path = String::from(object_path);
        Box::pin(GioFuture::new(&(), move |_obj, cancellable, send| {
            Self::new(
                &connection,
                flags,
                &name,
                &object_path,
                get_proxy_type_func,
                Some(cancellable),
                move |res| {
                    send.resolve(res);
                },
            );
        }))
    }

    #[doc(alias = "g_dbus_object_manager_client_new_for_bus")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new_for_bus<P: FnOnce(Result<DBusObjectManagerClient, glib::Error>) + 'static>(
        bus_type: BusType,
        flags: DBusObjectManagerClientFlags,
        name: &str,
        object_path: &str,
        get_proxy_type_func: Option<DBusProxyTypeFn>,
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

        unsafe extern "C" fn get_proxy_type_func_func(
            manager: *mut ffi::GDBusObjectManagerClient,
            object_path: *const std::ffi::c_char,
            interface_name: *const std::ffi::c_char,
            data: glib::ffi::gpointer,
        ) -> glib::ffi::GType {
            let manager = from_glib_borrow(manager);
            let object_path: Borrowed<glib::GString> = from_glib_borrow(object_path);
            let interface_name: Borrowed<Option<glib::GString>> = from_glib_borrow(interface_name);
            let callback = &*(data as *mut Option<DBusProxyTypeFn>);
            if let Some(ref callback) = *callback {
                callback(
                    &manager,
                    object_path.as_str(),
                    (*interface_name).as_ref().map(|s| s.as_str()),
                )
            } else {
                panic!("cannot get closure...")
            }
            .into_glib()
        }

        unsafe extern "C" fn get_proxy_type_destroy_notify_func(data: glib::ffi::gpointer) {
            let _callback = Box::from_raw(data as *mut Option<DBusProxyTypeFn>);
        }

        unsafe extern "C" fn new_for_bus_trampoline<
            P: FnOnce(Result<DBusObjectManagerClient, glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = std::ptr::null_mut();
            let ret = ffi::g_dbus_object_manager_client_new_finish(res, &mut error);
            let result = if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box<glib::thread_guard::ThreadGuard<P>> =
                Box::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }

        let get_proxy_type_user_data = Box::new(get_proxy_type_func);
        let get_proxy_type_func = if get_proxy_type_user_data.is_some() {
            Some(get_proxy_type_func_func as _)
        } else {
            None
        };
        let get_proxy_type_destroy_notify = if get_proxy_type_user_data.is_some() {
            Some(get_proxy_type_destroy_notify_func as _)
        } else {
            None
        };

        let user_data: Box<glib::thread_guard::ThreadGuard<P>> =
            Box::new(glib::thread_guard::ThreadGuard::new(callback));
        let callback = new_for_bus_trampoline::<P>;

        unsafe {
            ffi::g_dbus_object_manager_client_new_for_bus(
                bus_type.into_glib(),
                flags.into_glib(),
                name.to_glib_none().0,
                object_path.to_glib_none().0,
                get_proxy_type_func,
                Box::into_raw(get_proxy_type_user_data) as *mut _,
                get_proxy_type_destroy_notify,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn new_for_bus_future(
        bus_type: BusType,
        flags: DBusObjectManagerClientFlags,
        name: &str,
        object_path: &str,
        get_proxy_type_func: Option<DBusProxyTypeFn>,
    ) -> Pin<Box<dyn Future<Output = Result<DBusObjectManagerClient, glib::Error>> + 'static>> {
        let name = String::from(name);
        let object_path = String::from(object_path);
        Box::pin(GioFuture::new(&(), move |_obj, cancellable, send| {
            Self::new_for_bus(
                bus_type,
                flags,
                &name,
                &object_path,
                get_proxy_type_func,
                Some(cancellable),
                move |res| {
                    send.resolve(res);
                },
            );
        }))
    }

    #[doc(alias = "g_dbus_object_manager_client_new_for_bus_sync")]
    #[doc(alias = "new_for_bus_sync")]
    pub fn for_bus_sync(
        bus_type: BusType,
        flags: DBusObjectManagerClientFlags,
        name: &str,
        object_path: &str,
        get_proxy_type_func: Option<
            Box<
                dyn Fn(&DBusObjectManagerClient, &str, Option<&str>) -> glib::types::Type + 'static,
            >,
        >,
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<DBusObjectManagerClient, glib::Error> {
        let get_proxy_type_func_data: Box<
            Option<
                Box<
                    dyn Fn(&DBusObjectManagerClient, &str, Option<&str>) -> glib::types::Type
                        + 'static,
                >,
            >,
        > = Box::new(get_proxy_type_func);
        unsafe extern "C" fn get_proxy_type_func_func(
            manager: *mut ffi::GDBusObjectManagerClient,
            object_path: *const std::ffi::c_char,
            interface_name: *const std::ffi::c_char,
            data: glib::ffi::gpointer,
        ) -> glib::ffi::GType {
            let manager = from_glib_borrow(manager);
            let object_path: Borrowed<glib::GString> = from_glib_borrow(object_path);
            let interface_name: Borrowed<Option<glib::GString>> = from_glib_borrow(interface_name);
            let callback = &*(data as *mut Option<
                Box<
                    dyn Fn(&DBusObjectManagerClient, &str, Option<&str>) -> glib::types::Type
                        + 'static,
                >,
            >);
            if let Some(ref callback) = *callback {
                callback(
                    &manager,
                    object_path.as_str(),
                    (*interface_name).as_ref().map(|s| s.as_str()),
                )
            } else {
                panic!("cannot get closure...")
            }
            .into_glib()
        }
        let get_proxy_type_func = if get_proxy_type_func_data.is_some() {
            Some(get_proxy_type_func_func as _)
        } else {
            None
        };
        unsafe extern "C" fn get_proxy_type_destroy_notify_func(data: glib::ffi::gpointer) {
            let _callback = Box::from_raw(
                data as *mut Option<
                    Box<
                        dyn Fn(&DBusObjectManagerClient, &str, Option<&str>) -> glib::types::Type
                            + 'static,
                    >,
                >,
            );
        }
        let destroy_call6 = Some(get_proxy_type_destroy_notify_func as _);
        let super_callback0: Box<
            Option<
                Box<
                    dyn Fn(&DBusObjectManagerClient, &str, Option<&str>) -> glib::types::Type
                        + 'static,
                >,
            >,
        > = get_proxy_type_func_data;
        unsafe {
            let mut error = std::ptr::null_mut();
            let ret = ffi::g_dbus_object_manager_client_new_for_bus_sync(
                bus_type.into_glib(),
                flags.into_glib(),
                name.to_glib_none().0,
                object_path.to_glib_none().0,
                get_proxy_type_func,
                Box::into_raw(super_callback0) as *mut _,
                destroy_call6,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_dbus_object_manager_client_new_sync")]
    pub fn new_sync(
        connection: &DBusConnection,
        flags: DBusObjectManagerClientFlags,
        name: Option<&str>,
        object_path: &str,
        get_proxy_type_func: Option<
            Box<
                dyn Fn(&DBusObjectManagerClient, &str, Option<&str>) -> glib::types::Type + 'static,
            >,
        >,
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<DBusObjectManagerClient, glib::Error> {
        let get_proxy_type_func_data: Box<
            Option<
                Box<
                    dyn Fn(&DBusObjectManagerClient, &str, Option<&str>) -> glib::types::Type
                        + 'static,
                >,
            >,
        > = Box::new(get_proxy_type_func);
        unsafe extern "C" fn get_proxy_type_func_func(
            manager: *mut ffi::GDBusObjectManagerClient,
            object_path: *const std::ffi::c_char,
            interface_name: *const std::ffi::c_char,
            data: glib::ffi::gpointer,
        ) -> glib::ffi::GType {
            let manager = from_glib_borrow(manager);
            let object_path: Borrowed<glib::GString> = from_glib_borrow(object_path);
            let interface_name: Borrowed<Option<glib::GString>> = from_glib_borrow(interface_name);
            let callback = &*(data as *mut Option<
                Box<
                    dyn Fn(&DBusObjectManagerClient, &str, Option<&str>) -> glib::types::Type
                        + 'static,
                >,
            >);
            if let Some(ref callback) = *callback {
                callback(
                    &manager,
                    object_path.as_str(),
                    (*interface_name).as_ref().map(|s| s.as_str()),
                )
            } else {
                panic!("cannot get closure...")
            }
            .into_glib()
        }
        let get_proxy_type_func = if get_proxy_type_func_data.is_some() {
            Some(get_proxy_type_func_func as _)
        } else {
            None
        };
        unsafe extern "C" fn get_proxy_type_destroy_notify_func(data: glib::ffi::gpointer) {
            let _callback = Box::from_raw(
                data as *mut Option<
                    Box<
                        dyn Fn(&DBusObjectManagerClient, &str, Option<&str>) -> glib::types::Type
                            + 'static,
                    >,
                >,
            );
        }
        let destroy_call6 = Some(get_proxy_type_destroy_notify_func as _);
        let super_callback0: Box<
            Option<
                Box<
                    dyn Fn(&DBusObjectManagerClient, &str, Option<&str>) -> glib::types::Type
                        + 'static,
                >,
            >,
        > = get_proxy_type_func_data;
        unsafe {
            let mut error = std::ptr::null_mut();
            let ret = ffi::g_dbus_object_manager_client_new_sync(
                connection.to_glib_none().0,
                flags.into_glib(),
                name.to_glib_none().0,
                object_path.to_glib_none().0,
                get_proxy_type_func,
                Box::into_raw(super_callback0) as *mut _,
                destroy_call6,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}
