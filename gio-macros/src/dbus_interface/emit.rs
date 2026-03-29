// Take a look at the license at the top of the repository in the LICENSE file.

use crate::dbus_interface::attributes::DBusInterfaceAttribute;
use crate::dbus_interface::emit_info::{emit_interface_info, emit_method_info, emit_signal_info};
use crate::dbus_interface::parse::{DBusItems, DBusMethod, DBusMethodArgumentProvider, DBusSignal};
use crate::utils::ident_name;
use proc_macro2::{Span, TokenStream};
use quote::{ToTokens as _, quote, quote_spanned};
use syn::spanned::Spanned as _;
use syn::{Attribute, Ident, ReturnType, Token, Type, parse_quote};

pub(crate) fn emit_items(
    impl_attrs: &[Attribute],
    self_ty: &Type,
    items: &DBusItems,
    gio: &TokenStream,
) -> syn::Result<TokenStream> {
    let emit_methods = items.methods.values().map(|method| &method.item);
    let emit_signals = items
        .signals
        .values()
        .map(|signal| emit_signal_fn(signal, gio));
    let emit_errors = items.errors.iter().map(|(item, error)| {
        let compile_error = error.to_compile_error();
        quote! { #item #compile_error }
    });

    Ok(quote! {
        #(#impl_attrs)*
        impl #self_ty {
            #(#emit_methods)*
            #(#emit_signals)*
            #(#emit_errors)*
        }
    })
}

pub(crate) fn emit_interface_skeleton_impl(
    attr: &DBusInterfaceAttribute,
    ident: &Ident,
    gio: &TokenStream,
) -> syn::Result<TokenStream> {
    let interface_info = emit_interface_info(attr, ident, gio)?;
    Ok(quote! {
        unsafe impl #gio::subclass::prelude::DBusInterfaceSkeletonImpl for #ident {
            fn info(&self) -> *mut #gio::ffi::GDBusInterfaceInfo {
                use #gio::glib::translate::*;
                static INFO: ::std::sync::OnceLock<#gio::DBusInterfaceInfo> = ::std::sync::OnceLock::new();
                let info = INFO.get_or_init(|| #interface_info);
                info.as_ptr()
            }

            fn properties(&self) -> #gio::glib::Variant {
                todo!()
            }

            fn vtable(&self) -> *mut #gio::ffi::GDBusInterfaceVTable {
                use #gio::__macro_helpers::dbus_interface_skeleton::InterfaceVTable;
                static VTABLE: InterfaceVTable = unsafe {
                    InterfaceVTable(#gio::subclass::dbus_interface_skeleton_impl::vtable::<#ident>())
                };
                &VTABLE.0 as *const _ as *mut _
            }

            fn method_dispatch(
                &self,
                method_call_func: #gio::ffi::GDBusInterfaceMethodCallFunc,
                invocation: #gio::DBusMethodInvocation,
                flags: #gio::DBusInterfaceSkeletonFlags,
                object: Option<#gio::DBusObject>,
            ) {
                #gio::subclass::dbus_interface_skeleton_impl::method_dispatch_local(method_call_func, invocation, flags, object);
            }
        }

        impl #gio::subclass::prelude::DBusInterfaceSkeletonVtableImpl for #ident {
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
                #gio::__macro_helpers::dbus_interface_skeleton::DBusMethods::method_call(
                    self,
                    connection,
                    sender,
                    object_path,
                    interface_name,
                    method_name,
                    parameters,
                    invocation
                );
            }
        }
    })
}

pub(crate) fn emit_methods_impl(
    self_ty: &Type,
    items: &DBusItems,
    gio: &TokenStream,
) -> syn::Result<TokenStream> {
    let method_handlers = items.methods.values().map(|method| {
        let handler = emit_method_call_handler(method);
        let dbus_name = &method.dbus_name;
        quote! {
            #dbus_name => { #handler }
        }
    });
    let method_infos = items
        .methods
        .values()
        .map(|method| emit_method_info(method, gio))
        .collect::<Result<Vec<_>, _>>()?;
    let signal_infos = items
        .signals
        .values()
        .map(|signal| emit_signal_info(signal, gio))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(quote! {
        impl #gio::__macro_helpers::dbus_interface_skeleton::DBusMethods for #self_ty {
            fn method_infos() -> impl IntoIterator<Item = #gio::DBusMethodInfo> {
                [#(#method_infos,)*]
            }

            fn signal_infos() -> impl IntoIterator<Item = #gio::DBusSignalInfo> {
                [#(#signal_infos,)*]
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
                use #gio::__macro_helpers::dbus_interface_skeleton::result::*;
                use #gio::glib::subclass::types::{ObjectSubclassExt as _, ObjectSubclassIsExt as _};

                debug_assert_eq!(#gio::glib::VariantClass::Tuple, parameters.classify());

                match method_name {
                    #(#method_handlers,)*
                    _ => unreachable!(),
                }
            }
        }
    })
}

fn emit_method_call_handler(method: &DBusMethod) -> TokenStream {
    let fn_ident = &method.item.sig.ident;
    let arg_count = method
        .args
        .iter()
        .filter(|arg| matches!(arg.provider, DBusMethodArgumentProvider::Parameters { .. }))
        .count();
    let arg_names =
        (0..method.args.len()).map(|index| Ident::new(&format!("arg_{index}"), Span::call_site()));
    let arg_initializers = method
        .args
        .iter()
        .zip(arg_names.clone())
        .map(|(arg, arg_name)| {
            let span = arg.arg.ty.span();
            match arg.provider {
                DBusMethodArgumentProvider::Parameters { index } => quote_spanned! {
                    span => let #arg_name = parameters.child_get(#index);
                },
                DBusMethodArgumentProvider::Connection => quote_spanned! {
                    span => let #arg_name = connection.clone();
                },
                DBusMethodArgumentProvider::Invocation => quote_spanned! {
                    span => let #arg_name = invocation.clone();
                },
            }
        });
    if method.manual_return.is_some() {
        quote! {
            debug_assert_eq!(#arg_count, parameters.n_children());
            let obj = self.obj().clone();
            #(#arg_initializers)*
            #[allow(deprecated)]
            let _output: () = obj.imp().#fn_ident(#(#arg_names,)*);
        }
    } else {
        let map_output_to_result = quote_spanned! {
            method.item.sig.output.span() => (&&&output).kind().to_method_call_result(output)
        };
        quote! {
            debug_assert_eq!(#arg_count, parameters.n_children());
            let obj = self.obj().clone();
            #(#arg_initializers)*
            invocation.return_future_local(async move {
                #[allow(deprecated)]
                let output = obj.imp().#fn_ident(#(#arg_names,)*);
                #map_output_to_result.await
            });
        }
    }
}

fn emit_signal_fn(signal: &DBusSignal, gio: &TokenStream) -> TokenStream {
    let ident = &signal.item.sig.ident;
    let ident = Ident::new(&format!("emit_{}", ident_name(ident)), ident.span());
    let arg_idents = signal.args.iter().map(|arg| &arg.ident);
    let signal_name = &signal.dbus_name;
    let return_type: Type = parse_quote!(Result<(), #gio::glib::Error>);
    let block = parse_quote! {{
        let obj = &*#gio::glib::subclass::types::ObjectSubclassExt::obj(self);
        let connections = #gio::prelude::DBusInterfaceSkeletonExt::connections(obj);
        let object_path = #gio::prelude::DBusInterfaceSkeletonExt::object_path(obj);
        let object_path = object_path.as_deref();
        let info = gio::prelude::DBusInterfaceSkeletonExt::info(obj);
        let interface_name = info.name();
        let variant = (#(#arg_idents,)*).to_variant();
        for connection in connections {
            connection.emit_signal(
                None,
                object_path.expect("object path should be set when there is at least one connection"),
                interface_name,
                #signal_name,
                Some(&variant),
            )?;
        }
        Ok(())
    }};
    let mut item = signal.item.clone();
    item.block = block;
    item.sig.ident = ident;
    item.sig.output = ReturnType::Type(Token![->](Span::call_site()), Box::new(return_type));
    item.into_token_stream()
}
