// Take a look at the license at the top of the repository in the LICENSE file.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Type};

pub(crate) fn stub_impl(ident: &Ident, gio: &TokenStream) -> TokenStream {
    quote! {
        unsafe impl #gio::subclass::prelude::DBusInterfaceSkeletonImpl for #ident {
            fn info(&self) -> *mut #gio::ffi::GDBusInterfaceInfo {
                todo!()
            }

            fn properties(&self) -> #gio::glib::Variant {
                todo!()
            }

            fn vtable(&self) -> *mut #gio::ffi::GDBusInterfaceVTable {
                todo!()
            }
        }
    }
}

pub(crate) fn dbus_methods_stub_impl(self_ty: &Type, gio: &TokenStream) -> TokenStream {
    quote! {
        impl #gio::__macro_helpers::dbus_interface_skeleton::DBusMethods for #self_ty {
            fn method_infos() -> impl IntoIterator<Item = #gio::DBusMethodInfo> {
                []
            }

            fn method_call(
                &self,
                connection: #gio::DBusConnection,
                sender: Option<&str>,
                object_path: &str,
                interface_name: Option<&str>,
                method_name: &str,
                parameters: #gio::glib::Variant,
                invocation: #gio::DBusMethodInvocation,
            ) {
                todo!();
            }
        }
    }
}
