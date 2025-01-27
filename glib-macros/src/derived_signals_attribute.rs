// Take a look at the license at the top of the repository in the LICENSE file.

use proc_macro2::{Span, TokenStream};
use quote::quote;

pub const WRONG_PLACE_MSG: &str =
    "This macro should be used on `impl` block for `glib::ObjectImpl` trait";

pub fn impl_derived_signals(input: &syn::ItemImpl) -> syn::Result<TokenStream> {
    let syn::ItemImpl {
        attrs,
        generics,
        trait_,
        self_ty,
        items,
        ..
    } = input;

    let trait_path = &trait_
        .as_ref()
        .ok_or_else(|| syn::Error::new(Span::call_site(), WRONG_PLACE_MSG))?
        .1;

    let mut has_signals = false;

    for item in items {
        if let syn::ImplItem::Fn(method) = item {
            let ident = &method.sig.ident;

            if ident == "signals" {
                has_signals = true;
            }
        }
    }

    let glib = crate::utils::crate_ident_new();

    let signals = quote!(
        fn signals() -> &'static [#glib::subclass::signal::Signal] {
            Self::derived_signals()
        }
    );

    let generated = [
        (!has_signals).then_some(signals),
    ];

    Ok(quote!(
        #(#attrs)*
        impl #generics #trait_path for #self_ty {
            #(#items)*
            #(#generated)*
        }
    ))
}
