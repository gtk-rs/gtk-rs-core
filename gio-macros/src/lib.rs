// Take a look at the license at the top of the repository in the LICENSE file.

use proc_macro::TokenStream;
use syn::parse_macro_input;

mod dbus_interface;
mod utils;

#[proc_macro_attribute]
pub fn dbus_interface(attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    dbus_interface::impl_dbus_interface(attr.into(), input).into()
}
