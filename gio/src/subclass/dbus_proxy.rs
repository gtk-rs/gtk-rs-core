// Take a look at the license at the top of the repository in the LICENSE file.

#![deny(unsafe_op_in_unsafe_fn)]

use glib::{prelude::*, subclass::prelude::*, translate::*, GStr, StrVRef, Variant};

use crate::{
    ffi,
    subclass::prelude::{AsyncInitableImpl, DBusInterfaceImpl, InitableImpl},
    DBusProxy,
};

pub trait DBusProxyImpl:
    ObjectImpl
    + AsyncInitableImpl
    + DBusInterfaceImpl
    + InitableImpl
    + ObjectSubclass<Type: IsA<DBusProxy>>
{
    fn g_properties_changed(&self, changed_properties: &Variant, invalidated_properties: &StrVRef) {
        self.parent_g_properties_changed(changed_properties, invalidated_properties);
    }

    fn g_signal(&self, sender_name: Option<&GStr>, signal_name: &GStr, parameters: &Variant) {
        self.parent_g_signal(sender_name, signal_name, parameters);
    }
}

pub trait DBusProxyImplExt: DBusProxyImpl {
    fn parent_g_properties_changed(
        &self,
        changed_properties: &Variant,
        invalidated_properties: &StrVRef,
    ) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *const ffi::GDBusProxyClass;

            if let Some(f) = (*parent_class).g_properties_changed {
                f(
                    self.obj().unsafe_cast_ref::<DBusProxy>().to_glib_none().0,
                    changed_properties.to_glib_none().0,
                    invalidated_properties.to_glib_none().0,
                );
            }
        }
    }

    fn parent_g_signal(
        &self,
        sender_name: Option<&GStr>,
        signal_name: &GStr,
        parameters: &Variant,
    ) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *const ffi::GDBusProxyClass;

            if let Some(f) = (*parent_class).g_signal {
                f(
                    self.obj().unsafe_cast_ref::<DBusProxy>().to_glib_none().0,
                    sender_name.to_glib_none().0,
                    signal_name.to_glib_none().0,
                    parameters.to_glib_none().0,
                );
            }
        }
    }
}

impl<T: DBusProxyImpl> DBusProxyImplExt for T {}

unsafe impl<T: DBusProxyImpl> IsSubclassable<T> for DBusProxy {
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(class);
        let class = class.as_mut();
        class.g_properties_changed = Some(g_properties_changed::<T>);
        class.g_signal = Some(g_signal::<T>);
    }
}

unsafe extern "C" fn g_properties_changed<T: DBusProxyImpl>(
    proxy: *mut ffi::GDBusProxy,
    changed_properties: *mut glib::ffi::GVariant,
    invalidated_properties: *const *const libc::c_char,
) {
    let instance = unsafe { &*(proxy as *mut T::Instance) };
    let imp = instance.imp();

    let changed_properties = unsafe { from_glib_borrow(changed_properties) };
    let invalidated_properties = unsafe { StrVRef::from_glib_borrow(invalidated_properties) };
    imp.g_properties_changed(&changed_properties, invalidated_properties);
}

unsafe extern "C" fn g_signal<T: DBusProxyImpl>(
    proxy: *mut ffi::GDBusProxy,
    sender_name: *const libc::c_char,
    signal_name: *const libc::c_char,
    parameters: *mut glib::ffi::GVariant,
) {
    let instance = unsafe { &*(proxy as *mut T::Instance) };
    let imp = instance.imp();

    let sender_name = unsafe { Option::<&GStr>::from_glib_none(sender_name) };
    let signal_name = unsafe { from_glib_none(signal_name) };
    let parameters = unsafe { from_glib_borrow(parameters) };
    imp.g_signal(sender_name, signal_name, &parameters);
}
