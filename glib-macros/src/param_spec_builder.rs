// Take a look at the license at the top of the repository in the LICENSE file.

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use std::borrow::Cow;
use syn::parse::{Parse, Parser};
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, Token};

struct Attr {
    name: syn::Ident,
    _eq: Token![=],
    expr: syn::Expr,
}
impl Parse for Attr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            name: input.parse()?,
            _eq: input.parse()?,
            expr: input.parse()?,
        })
    }
}

fn adjusted_lifetime<'a>(symbol: &str, ty: &'a syn::Type) -> Cow<'a, syn::Type> {
    match ty {
        syn::Type::Reference(r) => {
            let mut r = r.to_owned();
            r.lifetime = Some(syn::Lifetime::new(symbol, Span::call_site()));
            Cow::Owned(syn::Type::Reference(r))
        }
        syn::Type::Path(p) => match &p.path.segments.last().unwrap().arguments {
            syn::PathArguments::AngleBracketed(bracketed) => {
                let generic_arg = bracketed.args.first().unwrap();
                match generic_arg {
                    syn::GenericArgument::Type(ty) => adjusted_lifetime(symbol, ty),
                    _ => panic!("Can't parse this generic argument"),
                }
            }
            _ => Cow::Borrowed(ty),
        },
        _ => Cow::Borrowed(ty),
    }
}
pub fn impl_builder(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::ItemImpl);
    let new_fn = input
        .items
        .iter()
        .find_map(|x| match x {
            syn::ImplItem::Method(m) if m.sig.ident == "new" => Some(m),
            _ => None,
        })
        .expect("Missing function 'new'");
    let target_ident = match &*input.self_ty {
        syn::Type::Path(p) => p
            .path
            .get_ident()
            .expect("param_spec_builder can only be applied to an `impl Ident`"),
        _ => panic!("Missign target type of impl block"),
    };
    let builder_ident = syn::Ident::new(&format!("{}Builder", target_ident), Span::call_site());

    let args_parser = Punctuated::<Attr, Token![,]>::parse_terminated;
    let default_vals = args_parser.parse(args).unwrap();

    let params = new_fn.sig.inputs.iter().filter_map(|input| match input {
        syn::FnArg::Typed(arg) => match &*arg.pat {
            syn::Pat::Ident(ident) => Some((ident, &arg.ty)),
            _ => None,
        },
        _ => None,
    });
    let params_with_defaults: Vec<_> = params
        .map(|(id, ty)| {
            let default_val = default_vals
                .iter()
                .find_map(|attr| (attr.name == id.ident).then(|| &attr.expr));
            (id, ty, default_val)
        })
        .collect();

    let builder_struct_fields = params_with_defaults.iter().map(|(ident, ty, _)| {
        let ty = adjusted_lifetime("'a", ty);
        quote!(#ident: Option<#ty>)
    });
    let builder_setters = params_with_defaults.iter().map(|(ident, ty, _)| {
        let ty = adjusted_lifetime("'a", ty);
        quote!(pub fn #ident(mut self, value: #ty) -> Self {
            self.#ident = Some(value);
            self
        })
    });
    let spec_new_call_params = params_with_defaults.iter().map(|(ident, _, default_val)| {
        let missing_err = format!("Missing parameter {}", ident.ident);
        let default_val_quote = default_val.map(|v| quote!(.or_else(|| #v.into())));
        quote!(
            (self.#ident
                #default_val_quote)
                .expect(#missing_err)
                .into()
        )
    });

    quote!(
        #input

        #[derive(Default)]
        pub struct #builder_ident<'a> {
            #(#builder_struct_fields,)*
        }
        impl<'a> #builder_ident<'a> {
            pub fn new() -> Self {
                Self::default()
            }

            #(#builder_setters)*

            pub fn build(self) -> glib::ParamSpec {
                #target_ident::new(#(#spec_new_call_params,)*)
            }
        }
    )
    .into()
}
