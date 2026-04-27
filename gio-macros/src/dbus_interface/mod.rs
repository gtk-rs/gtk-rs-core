// Take a look at the license at the top of the repository in the LICENSE file.

use crate::dbus_interface::attributes::DBusInterfaceAttribute;
use crate::dbus_interface::parse::parse_trait_items;
use crate::dbus_interface::transforms::remove_dbus_attribute_from_trait;
use crate::utils::crate_ident_new;
use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemTrait;

mod attributes;
mod emit;
mod parse;
mod transforms;

pub(crate) fn impl_dbus_interface(attr: TokenStream, mut input: ItemTrait) -> TokenStream {
    let attr = match DBusInterfaceAttribute::parse(attr) {
        Ok(value) => value,
        Err(error) => {
            let compile_error = error.into_compile_error();
            remove_dbus_attribute_from_trait(&mut input);
            return quote! {
                #input
                #compile_error
            };
        }
    };
    let items = parse_trait_items(&input.items);
    let gio = attr.crate_.clone().unwrap_or_else(crate_ident_new);
    emit::emit_interface(&input, &items, &attr, &gio)
}
