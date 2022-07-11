// Take a look at the license at the top of the repository in the LICENSE file.

use crate::utils::crate_ident_new;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote_spanned;
use syn::spanned::Spanned;

pub(crate) fn class_handler_inner(input: TokenStream, has_token: bool) -> TokenStream {
    let crate_ident = crate_ident_new();
    let closure = syn::parse_macro_input!(input as syn::ExprClosure);
    let closure_ident = Ident::new("____closure", Span::mixed_site());
    let token_ident = has_token.then(|| Ident::new("____token", Span::mixed_site()));
    let values_ident = Ident::new("____values", Span::mixed_site());
    let offset = if has_token { 1 } else { 0 };
    let arg_names = closure
        .inputs
        .iter()
        .skip(offset)
        .enumerate()
        .map(|(index, _)| Ident::new(&format!("____arg{}", index), Span::mixed_site()));
    let arg_names = if let Some(token) = token_ident.as_ref().cloned() {
        std::iter::once(token).chain(arg_names).collect::<Vec<_>>()
    } else {
        arg_names.collect::<Vec<_>>()
    };
    let arg_values = closure
        .inputs
        .iter()
        .skip(offset)
        .enumerate()
        .map(|(index, pat)| {
            let err_msg = format!("Wrong type for argument {}: {{:?}}", index);
            let name = &arg_names[index + offset];
            quote_spanned! { pat.span() =>
                let #name = #crate_ident::Value::get(&#values_ident[#index])
                    .unwrap_or_else(|e| ::std::panic!(#err_msg, e));
            }
        });
    let args_len = closure.inputs.len().saturating_sub(offset);
    let token_arg = token_ident.map(|t| {
        quote_spanned! { t.span() =>
            #t: #crate_ident::subclass::SignalClassOverrideToken,
        }
    });
    let output = quote_spanned! { closure.span() => {
        let #closure_ident = #closure;
        move
            |#token_arg #values_ident: &[#crate_ident::Value]| -> ::std::option::Option<#crate_ident::Value> {
            assert_eq!(#values_ident.len(), #args_len);
            #(#arg_values)*
            #crate_ident::closure::ToClosureReturnValue::to_closure_return_value(
                &#closure_ident(#(#arg_names),*)
            )
        }
    } };
    output.into()
}
