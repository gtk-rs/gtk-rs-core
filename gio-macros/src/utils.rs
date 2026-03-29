// Take a look at the license at the top of the repository in the LICENSE file.

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::Ident;

#[cfg(feature = "proc_macro_crate")]
pub(crate) fn crate_ident_new() -> TokenStream {
    use proc_macro_crate::{FoundCrate, crate_name};

    match crate_name("gio") {
        Ok(FoundCrate::Name(name)) => Some(name),
        Ok(FoundCrate::Itself) => Some("gio".to_string()),
        Err(_) => None,
    }
    .map(|s| {
        let gio = Ident::new(&s, Span::call_site());
        quote!(#gio)
    })
    .unwrap_or_else(|| {
        // We couldn't find the gio crate (renamed or not) so let's just hope it's in scope!
        //
        // We will be able to have this information once this code is stable:
        //
        // ```
        // let span = Span::call_site();
        // let source = span.source_file();
        // let file_path = source.path();
        // ```
        //
        // Then we can use proc_macro to parse the file and check if gio is imported somehow.

        let gio = Ident::new("gio", Span::call_site());
        quote!(#gio)
    })
}

#[cfg(not(feature = "proc_macro_crate"))]
pub(crate) fn crate_ident_new() -> TokenStream {
    let gio = Ident::new("gio", Span::call_site());
    quote!(#gio)
}
