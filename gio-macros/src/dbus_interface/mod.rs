// Take a look at the license at the top of the repository in the LICENSE file.

use crate::dbus_interface::attributes::{DBusInterfaceAttribute, DBusMethodsAttribute};
use crate::dbus_interface::emit::{emit_interface_skeleton_impl, emit_items, emit_methods_impl};
use crate::dbus_interface::stub::{dbus_methods_stub_impl, stub_impl};
use crate::dbus_interface::transforms::remove_dbus_attribute_from_impl;
use crate::utils::crate_ident_new;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::spanned::Spanned;
use syn::{ItemImpl, ItemStruct, Path};

mod attributes;
mod emit;
mod emit_info;
mod parse;
mod stub;
mod transforms;

// [TODO] gate these macros behind the v2_88 feature

pub fn derive_dbus_interface_skeleton(item: ItemStruct) -> TokenStream {
    let attr = match DBusInterfaceAttribute::parse(&item.attrs, item.span()) {
        Ok(value) => value,
        Err(error) => {
            let stub = stub_impl(&item.ident, &crate_ident_new());
            let compile_error = error.into_compile_error();
            return quote! {
                #stub
                #compile_error
            };
        }
    };
    let gio = gio(&attr.crate_);
    emit_interface_skeleton_impl(&attr, &item.ident, &gio).unwrap_or_else(move |error| {
        let compile_error = error.into_compile_error();
        let stub = stub_impl(&item.ident, &gio);
        quote! {
            #stub
            #compile_error
        }
    })
}

pub fn dbus_methods(attr: TokenStream, mut impl_item: ItemImpl) -> TokenStream {
    let attr = match DBusMethodsAttribute::parse(attr) {
        Ok(value) => value,
        Err(error) => {
            let compile_error = error.into_compile_error();
            return quote! {
                #impl_item
                #compile_error
            };
        }
    };
    let gio = gio(&attr.crate_);
    try_dbus_methods(impl_item.clone(), &gio).unwrap_or_else(move |error| {
        remove_dbus_attribute_from_impl(&mut impl_item);
        let compile_error = error.into_compile_error();
        let stub = dbus_methods_stub_impl(&impl_item.self_ty, &gio);
        quote! {
            #impl_item
            #stub
            #compile_error
        }
    })
}

fn try_dbus_methods(
    ItemImpl {
        attrs,
        self_ty,
        items,
        ..
    }: ItemImpl,
    gio: &TokenStream,
) -> syn::Result<TokenStream> {
    let dbus_items = parse::parse_impl_items(items);

    let items = emit_items(&attrs, &self_ty, &dbus_items, gio)?;
    let exportable_interface_impl = emit_methods_impl(&self_ty, &dbus_items, gio)?;
    Ok(quote! {
        #items
        #exportable_interface_impl
    })
}

fn gio(path: &Option<Path>) -> TokenStream {
    path.as_ref()
        .map(ToTokens::to_token_stream)
        .unwrap_or_else(crate_ident_new)
}
