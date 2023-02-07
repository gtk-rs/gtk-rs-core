// Take a look at the license at the top of the repository in the LICENSE file.

use proc_macro2::{Ident, Span, TokenStream};
use proc_macro_crate::crate_name;
use quote::{quote, quote_spanned};
use syn::{punctuated::Punctuated, spanned::Spanned, token::Comma, Variant};

pub fn crate_ident_new() -> TokenStream {
    use proc_macro_crate::FoundCrate;

    match crate_name("glib") {
        Ok(FoundCrate::Name(name)) => Some(name),
        Ok(FoundCrate::Itself) => Some("glib".to_string()),
        Err(_) => None,
    }
    .map(|s| {
        let glib = Ident::new(&s, Span::call_site());
        quote!(#glib)
    })
    .unwrap_or_else(|| {
        // We couldn't find the glib crate (renamed or not) so let's just hope it's in scope!
        //
        // We will be able to have this information once this code is stable:
        //
        // ```
        // let span = Span::call_site();
        // let source = span.source_file();
        // let file_path = source.path();
        // ```
        //
        // Then we can use proc_macro to parse the file and check if glib is imported somehow.
        let glib = Ident::new("glib", Span::call_site());
        quote!(#glib)
    })
}

// Generate i32 to enum mapping, used to implement
// glib::translate::TryFromGlib<i32>, such as:
//
//   if value == Animal::Goat as i32 {
//       return Some(Animal::Goat);
//   }
pub fn gen_enum_from_glib(
    enum_name: &Ident,
    enum_variants: &Punctuated<Variant, Comma>,
) -> TokenStream {
    // FIXME: can we express this with a match()?
    let recurse = enum_variants.iter().map(|v| {
        let name = &v.ident;
        quote_spanned! { v.span() =>
            if value == #enum_name::#name as i32 {
                return Some(#enum_name::#name);
            }
        }
    });
    quote! {
        #(#recurse)*
        None
    }
}
