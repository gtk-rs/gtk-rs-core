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

// Simplified DeriveInput without fields, for faster parsing
pub struct DeriveHeader {
    pub attrs: Vec<syn::Attribute>,
    pub vis: syn::Visibility,
    pub ident: syn::Ident,
    pub generics: syn::Generics,
    pub data: DeriveData,
}

pub enum DeriveData {
    Struct,
    Enum,
    Union,
}

impl syn::parse::Parse for DeriveHeader {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        let vis = input.parse()?;
        let data = input.parse()?;
        let ident = input.parse()?;
        let mut generics = input.parse::<syn::Generics>()?;
        let lookahead = input.lookahead1();
        if lookahead.peek(syn::Token![where]) {
            generics.where_clause = Some(input.parse()?);
        } else if input.peek(syn::token::Paren) {
            let content;
            syn::parenthesized!(content in input);
            skip_all(&content);
            if input.peek(syn::Token![where]) {
                generics.where_clause = Some(input.parse()?);
            }
        } else if lookahead.peek(syn::token::Brace) {
            let content;
            syn::braced!(content in input);
            skip_all(&content);
        } else if lookahead.peek(syn::Token![;]) {
            input.parse::<syn::Token![;]>()?;
        } else {
            return Err(lookahead.error());
        }
        Ok(Self {
            attrs,
            vis,
            ident,
            generics,
            data,
        })
    }
}

impl syn::parse::Parse for DeriveData {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(syn::Token![struct]) {
            input.parse::<syn::Token![struct]>()?;
            Ok(Self::Struct)
        } else if lookahead.peek(syn::Token![enum]) {
            input.parse::<syn::Token![enum]>()?;
            Ok(Self::Enum)
        } else if lookahead.peek(syn::Token![union]) {
            input.parse::<syn::Token![union]>()?;
            Ok(Self::Union)
        } else {
            Err(lookahead.error())
        }
    }
}

impl deluxe::HasAttributes for DeriveHeader {
    #[inline]
    fn attrs(&self) -> &[syn::Attribute] {
        &self.attrs
    }
    #[inline]
    fn attrs_mut(&mut self) -> deluxe::Result<&mut Vec<syn::Attribute>> {
        Ok(&mut self.attrs)
    }
}

#[inline]
pub fn skip_all(input: syn::parse::ParseStream) {
    let _ = input.step(|cursor| {
        let mut cur = *cursor;
        while let Some((_, next)) = cur.token_tree() {
            cur = next;
        }
        Ok(((), cur))
    });
}
