// Take a look at the license at the top of the repository in the LICENSE file.

use proc_macro2::TokenStream;
use quote::{quote, quote_spanned, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    spanned::Spanned,
    token, Expr, Ident, TypePath,
};

pub(crate) struct ThreadLocalObjectTokens {
    name: Ident,
    ty: TypePath,
    init_expr: Option<Expr>,
}

impl Parse for ThreadLocalObjectTokens {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        let _comma: token::Comma = input.parse()?;
        let ty: TypePath = input.parse()?;

        let init_expr = if input.is_empty() {
            None
        } else {
            let _comma: token::Comma = input.parse()?;
            Some(input.parse()?)
        };

        Ok(Self {
            name,
            ty,
            init_expr,
        })
    }
}

impl ToTokens for ThreadLocalObjectTokens {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            name,
            ty,
            init_expr,
        } = self;

        let init_stream = if let Some(init_expr) = init_expr {
            init_expr.to_token_stream()
        } else {
            quote_spanned! {
                ty.span() => #ty::default()
            }
        };

        let mod_name = Ident::new(
            &format!("__thread_local_object_private_{name}"),
            name.span(),
        );

        tokens.extend(quote! {
            mod #mod_name {
                use super::*;
                ::std::thread_local!(static THREAD_LOCAL_OBJ: #ty = #init_stream);

                pub fn #name() -> #ty {
                    THREAD_LOCAL_OBJ.with(|w| w.clone())
                }
            }

            pub use #mod_name::#name;
        });
    }
}
