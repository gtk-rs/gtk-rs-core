// Take a look at the license at the top of the repository in the LICENSE file.

use std::{boxed::Box as Box_, future::Future, marker::PhantomData, num::NonZeroU32};

use crate::{
    ffi, ActionGroup, DBusConnection, DBusInterfaceInfo, DBusMessage, DBusMethodInvocation,
    DBusSignalFlags, MenuModel,
};
use glib::{prelude::*, translate::*};

pub trait DBusMethodCall: Sized {
    fn parse_call(
        obj_path: &str,
        interface: Option<&str>,
        method: &str,
        params: glib::Variant,
    ) -> Result<Self, glib::Error>;
}

// rustdoc-stripper-ignore-next
/// Handle method invocations.
pub struct MethodCallBuilder<'a, T> {
    registration: RegistrationBuilder<'a>,
    capture_type: PhantomData<T>,
}

impl<'a, T: DBusMethodCall> MethodCallBuilder<'a, T> {
    // rustdoc-stripper-ignore-next
    /// Handle invocation of a parsed method call.
    ///
    /// For each DBus method call parse the call, and then invoke the given closure
    /// with
    ///
    /// 1. the DBus connection object,
    /// 2. the name of the sender of the method call,
    /// 3. the parsed call, and
    /// 4. the method invocation object.
    ///
    /// The closure **must** return a value through the invocation object in all
    /// code paths, using any of its `return_` functions, such as
    /// [`DBusMethodInvocation::return_result`] or
    /// [`DBusMethodInvocation::return_future_local`], to finish the call.
    ///
    /// If direct access to the invocation object is not needed,
    /// [`invoke_and_return`] and [`invoke_and_return_future_local`] provide a
    /// safer interface where the callback returns a result directly.
    pub fn invoke<F>(self, f: F) -> RegistrationBuilder<'a>
    where
        F: Fn(DBusConnection, Option<&str>, T, DBusMethodInvocation) + 'static,
    {
        self.registration.method_call(
            move |connection, sender, obj_path, interface, method, params, invocation| {
                match T::parse_call(obj_path, interface, method, params) {
                    Ok(call) => f(connection, sender, call, invocation),
                    Err(error) => invocation.return_gerror(error),
                }
            },
        )
    }

    // rustdoc-stripper-ignore-next
    /// Handle invocation of a parsed method call.
    ///
    /// For each DBus method call parse the call, and then invoke the given closure
    /// with
    ///
    /// 1. the DBus connection object,
    /// 2. the name of the sender of the method call, and
    /// 3. the parsed call.
    ///
    /// The return value of the closure is then returned on the method call.
    /// If the returned variant value is not a tuple, it is automatically wrapped
    /// in a single element tuple, as DBus methods must always return tuples.
    /// See [`DBusMethodInvocation::return_result`] for details.
    pub fn invoke_and_return<F>(self, f: F) -> RegistrationBuilder<'a>
    where
        F: Fn(DBusConnection, Option<&str>, T) -> Result<Option<glib::Variant>, glib::Error>
            + 'static,
    {
        self.invoke(move |connection, sender, call, invocation| {
            invocation.return_result(f(connection, sender, call))
        })
    }

    // rustdoc-stripper-ignore-next
    /// Handle an async invocation of a parsed method call.
    ///
    /// For each DBus method call parse the call, and then invoke the given closure
    /// with
    ///
    /// 1. the DBus connection object,
    /// 2. the name of the sender of the method call, and
    /// 3. the parsed call.
    ///
    /// The output of the future is then returned on the method call.
    /// If the returned variant value is not a tuple, it is automatically wrapped
    /// in a single element tuple, as DBus methods must always return tuples.
    /// See [`DBusMethodInvocation::return_future_local`] for details.
    pub fn invoke_and_return_future_local<F, Fut>(self, f: F) -> RegistrationBuilder<'a>
    where
        F: Fn(DBusConnection, Option<&str>, T) -> Fut + 'static,
        Fut: Future<Output = Result<Option<glib::Variant>, glib::Error>> + 'static,
    {
        self.invoke(move |connection, sender, call, invocation| {
            invocation.return_future_local(f(connection, sender, call));
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct RegistrationId(NonZeroU32);
#[derive(Debug, Eq, PartialEq)]
pub struct WatcherId(NonZeroU32);
#[derive(Debug, Eq, PartialEq)]
pub struct ActionGroupExportId(NonZeroU32);
#[derive(Debug, Eq, PartialEq)]
pub struct MenuModelExportId(NonZeroU32);
#[derive(Debug, Eq, PartialEq)]
pub struct FilterId(NonZeroU32);
#[derive(Debug, Eq, PartialEq)]
pub struct SignalSubscriptionId(NonZeroU32);

// rustdoc-stripper-ignore-next
/// Build a registered DBus object, by handling different parts of DBus.
#[must_use = "The builder must be built to be used"]
pub struct RegistrationBuilder<'a> {
    connection: &'a DBusConnection,
    object_path: &'a str,
    interface_info: &'a DBusInterfaceInfo,
    #[allow(clippy::type_complexity)]
    method_call: Option<
        Box_<
            dyn Fn(
                DBusConnection,
                Option<&str>,
                &str,
                Option<&str>,
                &str,
                glib::Variant,
                DBusMethodInvocation,
            ),
        >,
    >,
    #[allow(clippy::type_complexity)]
    get_property:
        Option<Box_<dyn Fn(DBusConnection, Option<&str>, &str, &str, &str) -> glib::Variant>>,
    #[allow(clippy::type_complexity)]
    set_property:
        Option<Box_<dyn Fn(DBusConnection, Option<&str>, &str, &str, &str, glib::Variant) -> bool>>,
}

impl<'a> RegistrationBuilder<'a> {
    pub fn method_call<
        F: Fn(
                DBusConnection,
                Option<&str>,
                &str,
                Option<&str>,
                &str,
                glib::Variant,
                DBusMethodInvocation,
            ) + 'static,
    >(
        mut self,
        f: F,
    ) -> Self {
        self.method_call = Some(Box_::new(f));
        self
    }

    // rustdoc-stripper-ignore-next
    /// Handle method calls on this object.
    ///
    /// Return a builder for method calls which parses method names and
    /// parameters with the given [`DBusMethodCall`] and then allows to dispatch
    /// the parsed call either synchronously or asynchronously.
    pub fn typed_method_call<T: DBusMethodCall>(self) -> MethodCallBuilder<'a, T> {
        MethodCallBuilder {
            registration: self,
            capture_type: Default::default(),
        }
    }

    #[doc(alias = "get_property")]
    pub fn property<
        F: Fn(DBusConnection, Option<&str>, &str, &str, &str) -> glib::Variant + 'static,
    >(
        mut self,
        f: F,
    ) -> Self {
        self.get_property = Some(Box_::new(f));
        self
    }

    pub fn set_property<
        F: Fn(DBusConnection, Option<&str>, &str, &str, &str, glib::Variant) -> bool + 'static,
    >(
        mut self,
        f: F,
    ) -> Self {
        self.set_property = Some(Box_::new(f));
        self
    }

    pub fn build(self) -> Result<RegistrationId, glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let id = ffi::g_dbus_connection_register_object_with_closures(
                self.connection.to_glib_none().0,
                self.object_path.to_glib_none().0,
                self.interface_info.to_glib_none().0,
                self.method_call
                    .map(|f| {
                        glib::Closure::new_local(move |args| {
                            let conn = args[0].get::<DBusConnection>().unwrap();
                            let sender = args[1].get::<Option<&str>>().unwrap();
                            let object_path = args[2].get::<&str>().unwrap();
                            let interface_name = args[3].get::<Option<&str>>().unwrap();
                            let method_name = args[4].get::<&str>().unwrap();
                            let parameters = args[5].get::<glib::Variant>().unwrap();

                            // Work around GLib memory leak: Assume that the invocation is passed
                            // as `transfer full` into the closure.
                            //
                            // This workaround is not going to break with future versions of
                            // GLib as fixing the bug was considered a breaking API change.
                            //
                            // See https://gitlab.gnome.org/GNOME/glib/-/merge_requests/4427
                            let invocation = from_glib_full(glib::gobject_ffi::g_value_get_object(
                                args[6].as_ptr(),
                            )
                                as *mut ffi::GDBusMethodInvocation);

                            f(
                                conn,
                                sender,
                                object_path,
                                interface_name,
                                method_name,
                                parameters,
                                invocation,
                            );
                            None
                        })
                    })
                    .to_glib_none()
                    .0,
                self.get_property
                    .map(|f| {
                        glib::Closure::new_local(move |args| {
                            let conn = args[0].get::<DBusConnection>().unwrap();
                            let sender = args[1].get::<Option<&str>>().unwrap();
                            let object_path = args[2].get::<&str>().unwrap();
                            let interface_name = args[3].get::<&str>().unwrap();
                            let property_name = args[4].get::<&str>().unwrap();
                            let result =
                                f(conn, sender, object_path, interface_name, property_name);
                            Some(result.to_value())
                        })
                    })
                    .to_glib_none()
                    .0,
                self.set_property
                    .map(|f| {
                        glib::Closure::new_local(move |args| {
                            let conn = args[0].get::<DBusConnection>().unwrap();
                            let sender = args[1].get::<Option<&str>>().unwrap();
                            let object_path = args[2].get::<&str>().unwrap();
                            let interface_name = args[3].get::<&str>().unwrap();
                            let property_name = args[4].get::<&str>().unwrap();
                            let value = args[5].get::<glib::Variant>().unwrap();
                            let result = f(
                                conn,
                                sender,
                                object_path,
                                interface_name,
                                property_name,
                                value,
                            );
                            Some(result.to_value())
                        })
                    })
                    .to_glib_none()
                    .0,
                &mut error,
            );

            if error.is_null() {
                Ok(RegistrationId(NonZeroU32::new_unchecked(id)))
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}

impl DBusConnection {
    #[doc(alias = "g_dbus_connection_register_object")]
    #[doc(alias = "g_dbus_connection_register_object_with_closures")]
    pub fn register_object<'a>(
        &'a self,
        object_path: &'a str,
        interface_info: &'a DBusInterfaceInfo,
    ) -> RegistrationBuilder<'a> {
        RegistrationBuilder {
            connection: self,
            object_path,
            interface_info,
            method_call: None,
            get_property: None,
            set_property: None,
        }
    }

    #[doc(alias = "g_dbus_connection_unregister_object")]
    pub fn unregister_object(
        &self,
        registration_id: RegistrationId,
    ) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::g_dbus_connection_unregister_object(
                    self.to_glib_none().0,
                    registration_id.0.into()
                ),
                "Failed to unregister D-Bus object"
            )
        }
    }

    #[doc(alias = "g_dbus_connection_export_action_group")]
    pub fn export_action_group<P: IsA<ActionGroup>>(
        &self,
        object_path: &str,
        action_group: &P,
    ) -> Result<ActionGroupExportId, glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let id = ffi::g_dbus_connection_export_action_group(
                self.to_glib_none().0,
                object_path.to_glib_none().0,
                action_group.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(ActionGroupExportId(NonZeroU32::new_unchecked(id)))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_dbus_connection_unexport_action_group")]
    pub fn unexport_action_group(&self, export_id: ActionGroupExportId) {
        unsafe {
            ffi::g_dbus_connection_unexport_action_group(self.to_glib_none().0, export_id.0.into());
        }
    }

    #[doc(alias = "g_dbus_connection_export_menu_model")]
    pub fn export_menu_model<P: IsA<MenuModel>>(
        &self,
        object_path: &str,
        menu: &P,
    ) -> Result<MenuModelExportId, glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let id = ffi::g_dbus_connection_export_menu_model(
                self.to_glib_none().0,
                object_path.to_glib_none().0,
                menu.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(MenuModelExportId(NonZeroU32::new_unchecked(id)))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_dbus_connection_unexport_menu_model")]
    pub fn unexport_menu_model(&self, export_id: MenuModelExportId) {
        unsafe {
            ffi::g_dbus_connection_unexport_menu_model(self.to_glib_none().0, export_id.0.into());
        }
    }

    #[doc(alias = "g_dbus_connection_add_filter")]
    pub fn add_filter<
        P: Fn(&DBusConnection, &DBusMessage, bool) -> Option<DBusMessage> + 'static,
    >(
        &self,
        filter_function: P,
    ) -> FilterId {
        let filter_function_data: Box_<P> = Box_::new(filter_function);
        unsafe extern "C" fn filter_function_func<
            P: Fn(&DBusConnection, &DBusMessage, bool) -> Option<DBusMessage> + 'static,
        >(
            connection: *mut ffi::GDBusConnection,
            message: *mut ffi::GDBusMessage,
            incoming: glib::ffi::gboolean,
            user_data: glib::ffi::gpointer,
        ) -> *mut ffi::GDBusMessage {
            let connection = from_glib_borrow(connection);
            let message = from_glib_full(message);
            let incoming = from_glib(incoming);
            let callback: &P = &*(user_data as *mut _);
            let res = (*callback)(&connection, &message, incoming);
            res.into_glib_ptr()
        }
        let filter_function = Some(filter_function_func::<P> as _);
        unsafe extern "C" fn user_data_free_func_func<
            P: Fn(&DBusConnection, &DBusMessage, bool) -> Option<DBusMessage> + 'static,
        >(
            data: glib::ffi::gpointer,
        ) {
            let _callback: Box_<P> = Box_::from_raw(data as *mut _);
        }
        let destroy_call3 = Some(user_data_free_func_func::<P> as _);
        let super_callback0: Box_<P> = filter_function_data;
        unsafe {
            let id = ffi::g_dbus_connection_add_filter(
                self.to_glib_none().0,
                filter_function,
                Box_::into_raw(super_callback0) as *mut _,
                destroy_call3,
            );
            FilterId(NonZeroU32::new_unchecked(id))
        }
    }

    #[doc(alias = "g_dbus_connection_remove_filter")]
    pub fn remove_filter(&self, filter_id: FilterId) {
        unsafe {
            ffi::g_dbus_connection_remove_filter(self.to_glib_none().0, filter_id.0.into());
        }
    }

    #[doc(alias = "g_dbus_connection_signal_subscribe")]
    #[allow(clippy::too_many_arguments)]
    pub fn signal_subscribe<
        P: Fn(&DBusConnection, &str, &str, &str, &str, &glib::Variant) + 'static,
    >(
        &self,
        sender: Option<&str>,
        interface_name: Option<&str>,
        member: Option<&str>,
        object_path: Option<&str>,
        arg0: Option<&str>,
        flags: DBusSignalFlags,
        callback: P,
    ) -> SignalSubscriptionId {
        let callback_data: Box_<P> = Box_::new(callback);
        unsafe extern "C" fn callback_func<
            P: Fn(&DBusConnection, &str, &str, &str, &str, &glib::Variant) + 'static,
        >(
            connection: *mut ffi::GDBusConnection,
            sender_name: *const libc::c_char,
            object_path: *const libc::c_char,
            interface_name: *const libc::c_char,
            signal_name: *const libc::c_char,
            parameters: *mut glib::ffi::GVariant,
            user_data: glib::ffi::gpointer,
        ) {
            let connection = from_glib_borrow(connection);
            let sender_name: Borrowed<glib::GString> = from_glib_borrow(sender_name);
            let object_path: Borrowed<glib::GString> = from_glib_borrow(object_path);
            let interface_name: Borrowed<glib::GString> = from_glib_borrow(interface_name);
            let signal_name: Borrowed<glib::GString> = from_glib_borrow(signal_name);
            let parameters = from_glib_borrow(parameters);
            let callback: &P = &*(user_data as *mut _);
            (*callback)(
                &connection,
                sender_name.as_str(),
                object_path.as_str(),
                interface_name.as_str(),
                signal_name.as_str(),
                &parameters,
            );
        }
        let callback = Some(callback_func::<P> as _);
        unsafe extern "C" fn user_data_free_func_func<
            P: Fn(&DBusConnection, &str, &str, &str, &str, &glib::Variant) + 'static,
        >(
            data: glib::ffi::gpointer,
        ) {
            let _callback: Box_<P> = Box_::from_raw(data as *mut _);
        }
        let destroy_call9 = Some(user_data_free_func_func::<P> as _);
        let super_callback0: Box_<P> = callback_data;
        unsafe {
            let id = ffi::g_dbus_connection_signal_subscribe(
                self.to_glib_none().0,
                sender.to_glib_none().0,
                interface_name.to_glib_none().0,
                member.to_glib_none().0,
                object_path.to_glib_none().0,
                arg0.to_glib_none().0,
                flags.into_glib(),
                callback,
                Box_::into_raw(super_callback0) as *mut _,
                destroy_call9,
            );
            SignalSubscriptionId(NonZeroU32::new_unchecked(id))
        }
    }

    #[doc(alias = "g_dbus_connection_signal_unsubscribe")]
    pub fn signal_unsubscribe(&self, subscription_id: SignalSubscriptionId) {
        unsafe {
            ffi::g_dbus_connection_signal_unsubscribe(
                self.to_glib_none().0,
                subscription_id.0.into(),
            );
        }
    }
}
