// Take a look at the license at the top of the repository in the LICENSE file.

use proc_macro::TokenStream;
use syn::parse_macro_input;

mod dbus_interface;
mod utils;

#[proc_macro_derive(DBusInterfaceSkeleton, attributes(dbus_interface, property))]
pub fn derive_dbus_interface_skeleton(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    dbus_interface::derive_dbus_interface_skeleton(input).into()
}

#[proc_macro_attribute]
pub fn dbus_methods(attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    dbus_interface::dbus_methods(attr.into(), input).into()
}
