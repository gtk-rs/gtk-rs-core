// Take a look at the license at the top of the repository in the LICENSE file.

#![deny(unsafe_op_in_unsafe_fn)]

use glib::{prelude::*, subclass::prelude::*, translate::*};

use crate::{DBusConnection, DBusError, DBusInterfaceSkeleton, DBusMethodInvocation, ffi};
#[cfg(feature = "v2_88")]
use crate::{DBusInterfaceSkeletonFlags, DBusObject};
#[cfg(feature = "v2_88")]
use gio_sys::GDBusInterfaceMethodCallFunc;

/// Use [`#[gio::dbus_interface]`](`crate::dbus_interface`) to implement this trait.
///
/// ## Safety
/// * The `method_call` vfunc is assumed to be safe to call from different threads since
///   one can set [`HANDLE_METHOD_INVOCATIONS_IN_THREAD`] freely on the skeleton object.
///   Implementors of this trait must therefore ensure that either
///   Self is `Send + Sync` or they need to override [`DBusInterfaceSkeletonImpl::method_dispatch`] ensuring that
///   [`HANDLE_METHOD_INVOCATIONS_IN_THREAD`] is ignored.
/// * `g_authorize_method` (if provided) is invoked in another thread
///   if you provide that method then you must ensure that `Self` is `Send + Sync`.
///
/// [`HANDLE_METHOD_INVOCATIONS_IN_THREAD`]: `crate::DBusInterfaceSkeletonFlags::HANDLE_METHOD_INVOCATIONS_IN_THREAD`
pub unsafe trait DBusInterfaceSkeletonImpl:
    ObjectImpl + ObjectSubclass<Type: IsA<DBusInterfaceSkeleton>>
{
    // Note: The base class optimizes for the case where `g_authorize_method` is
    // not overwritten. So setting `g_authorize_method` and calling the parent impl is not
    // equivalent to not setting the `g_authorize_method` vfunc at all.
    const OVERRIDE_G_AUTHORIZE_METHOD: bool = false;

    fn flush(&self) {}

    /// ## Remarks
    /// If you implement this method, make sure to also set [`DBusInterfaceSkeletonImpl::OVERRIDE_G_AUTHORIZE_METHOD`] to `true`.
    fn g_authorize_method(&self, _invocation: DBusMethodInvocation) -> bool {
        unimplemented!()
    }

    // ## Safety
    // The returned pointer is owned by the object instance (it must thus keep a strong reference).
    fn info(&self) -> *mut ffi::GDBusInterfaceInfo;

    fn properties(&self) -> glib::Variant;

    // ## Safety
    // The returned pointer is owned by the object instance (it must thus keep a strong reference).
    fn vtable(&self) -> *mut ffi::GDBusInterfaceVTable;

    #[cfg(feature = "v2_88")]
    fn method_dispatch(
        &self,
        method_call_func: GDBusInterfaceMethodCallFunc,
        invocation: DBusMethodInvocation,
        flags: DBusInterfaceSkeletonFlags,
        object: Option<DBusObject>,
    ) {
        self.parent_method_dispatch(method_call_func, invocation, flags, object);
    }
}

pub trait DBusInterfaceSkeletonVtableImpl: DBusInterfaceSkeletonImpl {
    const IMPLEMENT_GET_PROPERTY: bool = false;
    const IMPLEMENT_SET_PROPERTY: bool = false;

    fn method_call(
        &self,
        connection: DBusConnection,
        sender: Option<&str>,
        object_path: &str,
        interface_name: Option<&str>,
        method_name: &str,
        parameters: glib::Variant,
        invocation: DBusMethodInvocation,
    );

    /// ## Remarks
    /// If you implement this method, make sure to also set [`DBusInterfaceSkeletonVtableImpl::IMPLEMENT_GET_PROPERTY`] to `true`.
    fn get_property(
        &self,
        _connection: DBusConnection,
        _sender: Option<&str>,
        _object_path: &str,
        _interface_name: &str,
        _property_name: &str,
    ) -> Result<glib::Variant, glib::Error> {
        Err(glib::Error::new(DBusError::NotSupported, "not implemented"))
    }

    /// ## Remarks
    /// If you implement this method, make sure to also set [`DBusInterfaceSkeletonVtableImpl::IMPLEMENT_SET_PROPERTY`] to `true`.
    fn set_property(
        &self,
        _connection: DBusConnection,
        _sender: Option<&str>,
        _object_path: &str,
        _interface_name: &str,
        _property_name: &str,
        _value: glib::Variant,
    ) -> Result<(), glib::Error> {
        Err(glib::Error::new(DBusError::NotSupported, "not implemented"))
    }
}

pub mod impl_helpers {
    //! This module provides some utilities for safely implementing [`DBusInterfaceSkeletonImpl`].

    use super::*;

    /// ## Safety
    /// You must ensure that the `method_call` vfunc is
    /// not called from another thread if `T` doesn't implement `Send + Sync`.
    pub const unsafe fn vtable<T: DBusInterfaceSkeletonVtableImpl>() -> ffi::GDBusInterfaceVTable {
        ffi::GDBusInterfaceVTable {
            method_call: Some(method_call::<T>),
            get_property: if T::IMPLEMENT_GET_PROPERTY {
                Some(get_property::<T>)
            } else {
                None
            },
            set_property: if T::IMPLEMENT_SET_PROPERTY {
                Some(set_property::<T>)
            } else {
                None
            },
            padding: [::std::ptr::null_mut(); 8],
        }
    }

    /// An implementation for [`DBusInterfaceSkeletonImpl::method_dispatch`] that directly
    /// calls the registered vfunc without creating a new thread.
    #[cfg(feature = "v2_88")]
    pub fn method_dispatch_local(
        method_call_func: GDBusInterfaceMethodCallFunc,
        invocation: DBusMethodInvocation,
        flags: DBusInterfaceSkeletonFlags,
        _object: Option<DBusObject>,
    ) {
        assert!(
            !flags.contains(DBusInterfaceSkeletonFlags::HANDLE_METHOD_INVOCATIONS_IN_THREAD),
            "method invocation in thread is not supported"
        );

        if let Some(f) = method_call_func {
            unsafe {
                f(
                    ffi::g_dbus_method_invocation_get_connection(invocation.to_glib_none().0),
                    ffi::g_dbus_method_invocation_get_sender(invocation.to_glib_none().0),
                    ffi::g_dbus_method_invocation_get_object_path(invocation.to_glib_none().0),
                    ffi::g_dbus_method_invocation_get_interface_name(invocation.to_glib_none().0),
                    ffi::g_dbus_method_invocation_get_method_name(invocation.to_glib_none().0),
                    ffi::g_dbus_method_invocation_get_parameters(invocation.to_glib_none().0),
                    invocation.to_glib_full(),
                    ffi::g_dbus_method_invocation_get_user_data(invocation.to_glib_none().0),
                );
            }
        }
    }
}

pub trait DBusInterfaceSkeletonImplExt: DBusInterfaceSkeletonImpl {
    #[cfg(feature = "v2_88")]
    fn parent_method_dispatch(
        &self,
        method_call_func: GDBusInterfaceMethodCallFunc,
        invocation: DBusMethodInvocation,
        flags: DBusInterfaceSkeletonFlags,
        object: Option<DBusObject>,
    ) {
        unsafe {
            let data = Self::type_data();
            let parent_class =
                data.as_ref().parent_class() as *const ffi::GDBusInterfaceSkeletonClass;

            if let Some(f) = (*parent_class).method_dispatch {
                f(
                    self.obj()
                        .unsafe_cast_ref::<DBusInterfaceSkeleton>()
                        .to_glib_none()
                        .0,
                    method_call_func,
                    invocation.to_glib_full(),
                    flags.into_glib(),
                    object.to_glib_none().0,
                );
            }
        }
    }
}

impl<T: DBusInterfaceSkeletonImpl> DBusInterfaceSkeletonImplExt for T {}

unsafe impl<T: DBusInterfaceSkeletonImpl> IsSubclassable<T> for DBusInterfaceSkeleton {
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(class);
        let class = class.as_mut();
        class.flush = Some(flush::<T>);
        if T::OVERRIDE_G_AUTHORIZE_METHOD {
            class.g_authorize_method = Some(g_authorize_method::<T>);
        }
        class.get_info = Some(get_info::<T>);
        class.get_properties = Some(get_properties::<T>);
        class.get_vtable = Some(get_vtable::<T>);
        #[cfg(feature = "v2_88")]
        {
            class.method_dispatch = Some(method_dispatch::<T>);
        }
    }
}

unsafe extern "C" fn flush<T: DBusInterfaceSkeletonImpl>(
    skeleton: *mut ffi::GDBusInterfaceSkeleton,
) {
    let instance = unsafe { &*(skeleton as *mut T::Instance) };
    let imp = instance.imp();
    imp.flush();
}

unsafe extern "C" fn g_authorize_method<T: DBusInterfaceSkeletonImpl>(
    skeleton: *mut ffi::GDBusInterfaceSkeleton,
    invocation: *mut ffi::GDBusMethodInvocation,
) -> glib::ffi::gboolean {
    let instance = unsafe { &*(skeleton as *mut T::Instance) };
    let imp = instance.imp();
    let invocation = unsafe { from_glib_none(invocation) };
    imp.g_authorize_method(invocation).into_glib()
}

unsafe extern "C" fn get_info<T: DBusInterfaceSkeletonImpl>(
    skeleton: *mut ffi::GDBusInterfaceSkeleton,
) -> *mut ffi::GDBusInterfaceInfo {
    let instance = unsafe { &*(skeleton as *mut T::Instance) };
    let imp = instance.imp();
    imp.info()
}

unsafe extern "C" fn get_properties<T: DBusInterfaceSkeletonImpl>(
    skeleton: *mut ffi::GDBusInterfaceSkeleton,
) -> *mut glib::ffi::GVariant {
    let instance = unsafe { &*(skeleton as *mut T::Instance) };
    let imp = instance.imp();
    imp.properties().to_glib_full()
}

unsafe extern "C" fn get_vtable<T: DBusInterfaceSkeletonImpl>(
    skeleton: *mut ffi::GDBusInterfaceSkeleton,
) -> *mut ffi::GDBusInterfaceVTable {
    let instance = unsafe { &*(skeleton as *mut T::Instance) };
    let imp = instance.imp();
    imp.vtable()
}

#[cfg(feature = "v2_88")]
unsafe extern "C" fn method_dispatch<T: DBusInterfaceSkeletonImpl>(
    skeleton: *mut ffi::GDBusInterfaceSkeleton,
    method_call_func: ffi::GDBusInterfaceMethodCallFunc,
    invocation: *mut ffi::GDBusMethodInvocation,
    flags: ffi::GDBusInterfaceSkeletonFlags,
    object: *mut ffi::GDBusObject,
) {
    let instance = unsafe { &*(skeleton as *mut T::Instance) };
    let imp = instance.imp();
    let invocation = unsafe { from_glib_full(invocation) };
    let flags = unsafe { from_glib(flags) };
    let object = unsafe { from_glib_none(object) };
    imp.method_dispatch(method_call_func, invocation, flags, object);
}

#[doc(hidden)]
unsafe extern "C" fn method_call<T: DBusInterfaceSkeletonVtableImpl>(
    connection: *mut ffi::GDBusConnection,
    sender: *const glib::ffi::gchar,
    object_path: *const glib::ffi::gchar,
    interface_name: *const glib::ffi::gchar,
    method_name: *const glib::ffi::gchar,
    parameters: *mut glib::ffi::GVariant,
    invocation: *mut ffi::GDBusMethodInvocation,
    user_data: glib::ffi::gpointer,
) {
    use glib::subclass::prelude::*;
    use glib::translate::*;

    let connection: DBusConnection = unsafe { from_glib_none(connection) };
    let sender: Borrowed<Option<glib::GString>> = unsafe { from_glib_borrow(sender) };
    let object_path: Borrowed<glib::GString> = unsafe { from_glib_borrow(object_path) };
    let interface_name: Borrowed<Option<glib::GString>> =
        unsafe { from_glib_borrow(interface_name) };
    let method_name: Borrowed<glib::GString> = unsafe { from_glib_borrow(method_name) };
    let parameters: glib::Variant = unsafe { from_glib_none(parameters) };
    let invocation: DBusMethodInvocation = unsafe { from_glib_full(invocation) };

    let instance = unsafe { &*(user_data as *mut T::Instance) };
    let imp = instance.imp();

    imp.method_call(
        connection,
        sender.as_deref(),
        &object_path,
        interface_name.as_deref(),
        &method_name,
        parameters,
        invocation,
    )
}

unsafe extern "C" fn get_property<T: DBusInterfaceSkeletonVtableImpl>(
    connection: *mut ffi::GDBusConnection,
    sender: *const glib::ffi::gchar,
    object_path: *const glib::ffi::gchar,
    interface_name: *const glib::ffi::gchar,
    property_name: *const glib::ffi::gchar,
    error_out: *mut *mut glib::ffi::GError,
    user_data: glib::ffi::gpointer,
) -> *mut glib::ffi::GVariant {
    use glib::subclass::prelude::*;
    use glib::translate::*;

    let connection = unsafe { from_glib_none(connection) };
    let sender: Borrowed<Option<glib::GString>> = unsafe { from_glib_borrow(sender) };
    let object_path: Borrowed<glib::GString> = unsafe { from_glib_borrow(object_path) };
    let interface_name: Borrowed<glib::GString> = unsafe { from_glib_borrow(interface_name) };
    let property_name: Borrowed<glib::GString> = unsafe { from_glib_borrow(property_name) };

    let instance =
        unsafe { &*(user_data as *mut <T as glib::subclass::types::ObjectSubclass>::Instance) };
    let imp = instance.imp();

    let result = imp.get_property(
        connection,
        sender.as_deref(),
        &object_path,
        &interface_name,
        &property_name,
    );
    match result {
        Ok(variant) => variant.to_glib_full(),
        Err(error) => {
            unsafe { *error_out = error.to_glib_full() };
            std::ptr::null_mut()
        }
    }
}

unsafe extern "C" fn set_property<T: DBusInterfaceSkeletonVtableImpl>(
    connection: *mut ffi::GDBusConnection,
    sender: *const glib::ffi::gchar,
    object_path: *const glib::ffi::gchar,
    interface_name: *const glib::ffi::gchar,
    property_name: *const glib::ffi::gchar,
    value: *mut glib::ffi::GVariant,
    error_out: *mut *mut glib::ffi::GError,
    user_data: glib::ffi::gpointer,
) -> glib::ffi::gboolean {
    let connection = unsafe { from_glib_none(connection) };
    let sender: Borrowed<Option<glib::GString>> = unsafe { from_glib_borrow(sender) };
    let object_path: Borrowed<glib::GString> = unsafe { from_glib_borrow(object_path) };
    let interface_name: Borrowed<glib::GString> = unsafe { from_glib_borrow(interface_name) };
    let property_name: Borrowed<glib::GString> = unsafe { from_glib_borrow(property_name) };
    let value = unsafe { from_glib_none(value) };

    let instance = unsafe { &*(user_data as *mut T::Instance) };
    let imp = instance.imp();

    let result = DBusInterfaceSkeletonVtableImpl::set_property(
        imp,
        connection,
        sender.as_deref(),
        &object_path,
        &interface_name,
        &property_name,
        value,
    );
    match result {
        Ok(()) => true.into_glib(),
        Err(error) => {
            unsafe { *error_out = error.to_glib_full() };
            false.into_glib()
        }
    }
}
