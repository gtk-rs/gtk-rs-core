// Take a look at the license at the top of the repository in the LICENSE file.
#![allow(unused)]

use std::sync;

use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{parse::Parse, spanned::Spanned, ExprAsync, Result};

use crate::utils::*;

pub const WRONG_PLACE_MSG: &str =
    "expected an `impl` block for an `ObjectSubclass` type";

pub struct SignalsArgs {
    wrapper_type: syn::Type,
}

impl Parse for SignalsArgs {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let mut wrapper_type = None;

        while !input.is_empty() {
            let ident = input.parse::<syn::Ident>()?;

            if ident == "wrapper_type" {
                let _eq = input.parse::<syn::Token![=]>()?;
                wrapper_type = Some(input.parse::<syn::Type>()?);
            }

            if input.peek(syn::Token![,]) {
                input.parse::<syn::Token![,]>()?;
            }
        }

        Ok(Self {
            wrapper_type: wrapper_type.ok_or_else(|| {
                syn::Error::new(
                    input.span(),
                    "missing wrapper_type for #[glib::signals]",
                )
            })?,
        })
    }
}

enum SignalArg {
    Name(syn::Ident, syn::Token![=], syn::LitStr),
    RunFirst(syn::Ident),
    RunLast(syn::Ident),
    RunCleanup(syn::Ident),
    NoRecurse(syn::Ident),
    Detailed(syn::Ident),
    Action(syn::Ident),
    NoHooks(syn::Ident),
    MustCollect(syn::Ident),
    Deprecated(syn::Ident),
    Accumulator(syn::Ident, syn::Token![=], syn::Expr),
}

impl SignalArg {
    fn span(&self) -> Span {
        match self {
            Self::Name(ident, _, _)
            | Self::RunFirst(ident)
            | Self::RunLast(ident)
            | Self::RunCleanup(ident)
            | Self::NoRecurse(ident)
            | Self::Detailed(ident)
            | Self::Action(ident)
            | Self::NoHooks(ident)
            | Self::MustCollect(ident)
            | Self::Deprecated(ident)
            | Self::Accumulator(ident, _, _) => ident.span(),
        }
    }
}

impl Parse for SignalArg {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let ident = input.parse::<syn::Ident>()?;

        if ident == "name" {
            let eq_token = input.parse::<syn::Token![=]>()?;
            let lit_str = input.parse::<syn::LitStr>()?;
            Ok(SignalArg::Name(ident, eq_token, lit_str))
        } else if ident == "run_first" {
            Ok(SignalArg::RunFirst(ident))
        } else if ident == "run_last" {
            Ok(SignalArg::RunLast(ident))
        } else if ident == "run_cleanup" {
            Ok(SignalArg::RunCleanup(ident))
        } else if ident == "no_recurse" {
            Ok(SignalArg::NoRecurse(ident))
        } else if ident == "detailed" {
            Ok(SignalArg::Detailed(ident))
        } else if ident == "action" {
            Ok(SignalArg::Action(ident))
        } else if ident == "no_hooks" {
            Ok(SignalArg::NoHooks(ident))
        } else if ident == "must_collect" {
            Ok(SignalArg::MustCollect(ident))
        } else if ident == "deprecated" {
            Ok(SignalArg::Deprecated(ident))
        } else if ident == "accumulator" {
            let eq_token = input.parse::<syn::Token![=]>()?;
            let expr = input.parse::<syn::Expr>()?;
            Ok(SignalArg::Accumulator(ident, eq_token, expr))
        } else {
            Err(syn::Error::new_spanned(ident, "invalid argument"))
        }
    }
}

#[derive(Default)]
struct SignalArgs(syn::punctuated::Punctuated<SignalArg, syn::Token![,]>);

impl Parse for SignalArgs {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        input
            .parse_terminated(SignalArg::parse, syn::Token![,])
            .map(Self)
    }
}

struct SignalBuilder {
    wrapper_type: syn::Type,
    impl_type: syn::Type,
    fn_ident: syn::Ident,
    param_types: Vec<syn::Type>,
    return_type: Option<syn::Type>,
    class_handler: bool,

    name: Option<syn::LitStr>,
    run_first: Option<syn::Ident>,
    run_last: Option<syn::Ident>,
    run_cleanup: Option<syn::Ident>,
    no_recurse: Option<syn::Ident>,
    detailed: Option<syn::Ident>,
    action: Option<syn::Ident>,
    no_hooks: Option<syn::Ident>,
    must_collect: Option<syn::Ident>,
    deprecated: Option<syn::Ident>,
    accumulator: Option<(syn::Ident, syn::Expr)>,
}

impl SignalBuilder {
    fn new(
        fn_: &syn::ImplItemFn,
        impl_type: syn::Type,
        wrapper_type: syn::Type,
        args: Vec<SignalArg>,
    ) -> Result<Self> {
        if !fn_.sig.inputs.get(0).is_some_and(|fn_arg| {
            matches!(
                fn_arg,
                syn::FnArg::Receiver(syn::Receiver {
                    reference: Some(_),
                    mutability: None,
                    ..
                })
            )
        }) {
            return Err(syn::Error::new_spanned(
                fn_,
                "#[signal] function needs a &self parameter",
            ));
        }

        let fn_ident = fn_.sig.ident.clone();

        let param_types = fn_
            .sig
            .inputs
            .iter()
            .filter_map(|fn_arg| match fn_arg {
                syn::FnArg::Receiver(_) => None,
                syn::FnArg::Typed(pat) => Some(pat.ty.as_ref().clone()),
            })
            .collect();

        let return_type = match &fn_.sig.output {
            syn::ReturnType::Default => None,
            syn::ReturnType::Type(_, type_) => Some(type_.as_ref().clone()),
        };

        let class_handler = !fn_.block.stmts.is_empty();

        let mut builder = Self {
            wrapper_type,
            impl_type,
            fn_ident,
            param_types,
            return_type,
            class_handler,
            name: None,
            run_first: None,
            run_last: None,
            run_cleanup: None,
            no_recurse: None,
            detailed: None,
            action: None,
            no_hooks: None,
            must_collect: None,
            deprecated: None,
            accumulator: None,
        };

        for arg in args {
            builder.add_arg(arg)?;
        }

        Ok(builder)
    }

    fn add_arg(&mut self, arg: SignalArg) -> Result<()> {
        match arg {
            SignalArg::Name(_, _, lit_str) => {
                self.name = Some(lit_str);
            }

            SignalArg::RunFirst(ident) if self.run_first.is_none() => {
                self.run_first = Some(ident);
            }

            SignalArg::RunLast(ident) if self.run_last.is_none() => {
                self.run_last = Some(ident);
            }

            SignalArg::RunCleanup(ident) if self.run_cleanup.is_none() => {
                self.run_cleanup = Some(ident);
            }

            SignalArg::NoRecurse(ident) if self.no_recurse.is_none() => {
                self.no_recurse = Some(ident);
            }

            SignalArg::Detailed(ident) if self.detailed.is_none() => {
                self.detailed = Some(ident);
            }

            SignalArg::Action(ident) if self.action.is_none() => {
                self.action = Some(ident);
            }

            SignalArg::NoHooks(ident) if self.no_hooks.is_none() => {
                self.no_hooks = Some(ident);
            }

            SignalArg::MustCollect(ident) if self.must_collect.is_none() => {
                self.must_collect = Some(ident);
            }

            SignalArg::Deprecated(ident) if self.deprecated.is_none() => {
                self.deprecated = Some(ident);
            }

            SignalArg::Accumulator(ident, _, expr) if self.accumulator.is_none() => {
                self.accumulator = Some((ident, expr));
            }

            duplicate => {
                return Err(syn::Error::new(
                    duplicate.span(),
                    "duplicate argument",
                ))
            }
        }

        Ok(())
    }

    fn expr(&self) -> TokenStream {
        let crate_ident = crate_ident_new();

        let Self {
            wrapper_type,
            impl_type,
            fn_ident,
            param_types,
            return_type,
            class_handler,
            name,
            run_first,
            run_last,
            run_cleanup,
            no_recurse,
            detailed,
            action,
            no_hooks,
            must_collect,
            deprecated,
            accumulator,
        } = self;

        let name = name.clone().unwrap_or_else(|| {
            syn::LitStr::new(
                &fn_ident.to_string().trim_matches('_').replace('_', "-"),
                fn_ident.span(),
            )
        });

        let param_static_types = (!param_types.is_empty()).then(|| {
            param_types.iter().map(|type_| {
                quote! { <#type_ as #crate_ident::types::StaticType>::static_type() }
            })
        });

        let class_handler_wrapper = class_handler.then(|| {
            let wrapper_ident = syn::Ident::new("wrapper", Span::mixed_site());
            let params_ident = syn::Ident::new("params", Span::mixed_site());
            let param_idents = (0..param_types.len())
                .map(|idx| quote::format_ident!("arg{}", idx, span = Span::mixed_site()))
                .collect::<Vec<syn::Ident>>();
            let return_ident = syn::Ident::new("return_", Span::mixed_site());

            let param_bindings = param_types.iter().zip(&param_idents).enumerate().map(|(idx, (type_, ident))| {
                let real_idx = idx + 1;
                quote! { let #ident: #type_ = #params_ident[#real_idx].get().unwrap(); }
            });

            let return_wrapper = if *class_handler {
                quote! {
                    |#return_ident| {
                        Some(<#return_type as #crate_ident::value::ToValue>::to_value(&#return_ident))
                    }
                }
            } else {
                quote! { (|()| None) }
            };

            let unwrapped = quote! {
                #impl_type::#fn_ident(
                    <#impl_type as #crate_ident::subclass::types::ObjectSubclassExt>::from_obj(&#wrapper_ident),
                    #(#param_idents,)*
                )
            };

            quote! {
                |#params_ident: &[#crate_ident::value::Value]| -> ::std::option::Option<#crate_ident::value::Value> {
                    let #wrapper_ident: #wrapper_type = #params_ident[0].get().unwrap();
                    #(#param_bindings)*
                    (#return_wrapper)(#unwrapped)
                }
            }
        });

        let return_type = return_type.iter();
        let class_handler = class_handler_wrapper.into_iter();
        let param_types = param_static_types
            .map(|static_types| quote! { param_types([#(#static_types),*]) })
            .into_iter();
        let run_first = run_first.iter();
        let run_last = run_last.iter();
        let run_cleanup = run_cleanup.iter();
        let no_recurse = no_recurse.iter();
        let detailed = detailed.iter();
        let action = action.iter();
        let no_hooks = no_hooks.iter();
        let must_collect = must_collect.iter();
        let deprecated = deprecated.iter();

        quote! {
            #crate_ident::subclass::signal::Signal::builder(#name)
                #(.#param_types)*
                #(.return_type::<#return_type>())*
                #(.class_handler(#class_handler))*
                #(.#run_first())*
                #(.#run_last())*
                #(.#run_cleanup())*
                #(.#no_recurse())*
                #(.#detailed())*
                #(.#action())*
                #(.#no_hooks())*
                #(.#must_collect())*
                #(.#deprecated())*
                .build()
        }
    }
}

fn parse_signal_attr(attr: syn::Attribute) -> Result<impl Iterator<Item = SignalArg>> {
    let args = match attr.meta {
        syn::Meta::Path(path) => {
            assert!(path.get_ident().is_some_and(|ident| ident == "signal"));
            SignalArgs::default()
        }

        syn::Meta::List(list) => {
            assert!(list.path.get_ident().is_some_and(|ident| ident == "signal"));
            syn::parse2(list.tokens)?
        }

        other => {
            return Err(syn::Error::new_spanned(
                other,
                "invalid attribute",
            ))
        }
    };

    Ok(args.0.into_iter())
}

fn parse_signal_attrs(attrs: &mut Vec<syn::Attribute>) -> Result<Option<Vec<SignalArg>>> {
    let mut args = None;
    let mut idx = 0;

    while let Some(attr) = attrs.get(idx) {
        if attr
            .path()
            .get_ident()
            .is_some_and(|ident| ident == "signal")
        {
            args.get_or_insert_with(Vec::new)
                .extend(parse_signal_attr(attrs.remove(idx))?);
        } else {
            idx += 1;
        }
    }

    Ok(args)
}

fn parse_signals(
    items: &mut [syn::ImplItem],
    impl_type: &syn::Type,
    wrapper_type: &syn::Type,
) -> Result<Vec<SignalBuilder>> {
    let mut signals = Vec::new();

    for item in items.iter_mut() {
        let syn::ImplItem::Fn(fn_) = item else {
            continue;
        };

        let Some(args) = parse_signal_attrs(&mut fn_.attrs)? else {
            continue;
        };

        if fn_.block.stmts.is_empty() {
            fn_.attrs.push(syn::parse_quote!(#[allow(dead_code)]));
        }

        signals.push(SignalBuilder::new(
            fn_,
            impl_type.clone(),
            wrapper_type.clone(),
            args,
        )?);
    }

    Ok(signals)
}

fn impl_derived_signals(signals: &[SignalBuilder], impl_type: &syn::Type) -> TokenStream {
    let crate_ident = crate_ident_new();
    let count = signals.len();
    let exprs = signals.iter().map(|builder| builder.expr());

    quote! {
        #[automatically_derived]
        impl #crate_ident::subclass::object::DerivedObjectSignals for #impl_type {
            fn derived_signals() -> &'static [#crate_ident::subclass::signal::Signal] {
                static SIGNALS: ::std::sync::LazyLock<[#crate_ident::subclass::signal::Signal; #count]> =
                    ::std::sync::LazyLock::new(|| [ #(#exprs),* ]);

                SIGNALS.as_ref()
            }
        }
    }
}

pub fn impl_signals(mut input: syn::ItemImpl, args: SignalsArgs) -> Result<TokenStream> {
    let signals = parse_signals(&mut input.items, &input.self_ty, &args.wrapper_type)?;
    let derived_signals = impl_derived_signals(&signals, &input.self_ty);

    Ok(quote! {
        #input
        #derived_signals
    })
}
