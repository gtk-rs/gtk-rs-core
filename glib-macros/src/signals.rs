use proc_macro2::TokenStream;
use syn::{parse::Parse, spanned::Spanned, Token};

pub const WRONG_PLACE_MSG: &str =
    "This macro should be used on a plain `impl` block of the inner object type";

pub struct SignalAttrInput {
    wrapper_ty: syn::Path,
    // None => no ext trait,
    // Some(None) => derive the ext trait from the wrapper type,
    // Some(Some(ident)) => use the given ext trait Ident
    ext_trait: Option<Option<syn::Ident>>,
}

impl Parse for SignalAttrInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut wrapper_ty = None;
        let mut ext_trait = None;

        while !input.is_empty() {
            let ident = input.parse::<syn::Ident>()?;
            if ident == "wrapper_type" {
                let _eq = input.parse::<Token![=]>()?;
                wrapper_ty = Some(input.parse::<syn::Path>()?);
            } else if ident == "ext_trait" {
                if input.peek(Token![=]) {
                    let _eq = input.parse::<Token![=]>()?;
                    let ident = input.parse::<syn::Ident>()?;
                    ext_trait = Some(Some(ident));
                } else {
                    ext_trait = Some(None);
                }
            }
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(Self {
            wrapper_ty: wrapper_ty.ok_or_else(|| {
                syn::Error::new(input.span(), "missing #[signals(wrapper_type = ...)]")
            })?,
            ext_trait,
        })
    }
}

pub fn impl_signals(attr: SignalAttrInput, item: syn::ItemImpl) -> syn::Result<TokenStream> {
    if let Some((_, trait_path, _)) = &item.trait_ {
        return Err(syn::Error::new_spanned(trait_path, WRONG_PLACE_MSG));
    }




    Ok(TokenStream::new())
}