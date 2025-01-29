// Take a look at the license at the top of the repository in the LICENSE file.

use bitflags::bitflags;
use proc_macro2::{Literal, Span, TokenStream};
use quote::ToTokens;
use syn::{ext::IdentExt, parse::Parse, punctuated::Punctuated, Token};

use crate::utils::crate_ident_new;

pub const WRONG_PLACE_MSG: &str =
    "This macro should be used on a plain `impl` block of the inner object type";

/// An incomplete function in an impl block.
/// This is used to declare a signal with no class handler.
#[allow(unused)]
struct ImplItemIncompleteFn {
    pub attrs: Vec<syn::Attribute>,
    pub vis: syn::Visibility,
    pub defaultness: Option<Token![default]>,
    pub sig: syn::Signature,
    pub semi_token: Token![;],
}
impl Parse for ImplItemIncompleteFn {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            attrs: input.call(syn::Attribute::parse_outer)?,
            vis: input.parse()?,
            defaultness: input.parse()?,
            sig: input.parse()?,
            semi_token: input.parse()?,
        })
    }
}

/// Arguments to the `#[signals]` attribute.
pub struct Args {
    wrapper_ty: syn::Path,
    // None => no ext trait,
    // Some(None) => derive the ext trait from the wrapper type,
    // Some(Some(ident)) => use the given ext trait Ident
    ext_trait: Option<Option<syn::Ident>>,
}

impl Parse for Args {
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

/// A single parameter in `#[signal(...)]`.
pub enum SignalAttr {
    RunFirst(syn::Ident),
    RunLast(syn::Ident),
    RunCleanup(syn::Ident),
    NoRecurse(syn::Ident),
    Detailed(syn::Ident),
    Action(syn::Ident),
    NoHooks(syn::Ident),
    Accum(syn::Expr),
}
impl Parse for SignalAttr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = input.call(syn::Ident::parse_any)?;
        let name_str = name.to_string();

        let result = if input.peek(Token![=]) {
            let _assign_token: Token![=] = input.parse()?;

            match &*name_str {
                "accum" => Self::Accum(input.parse()?),
                _ => {
                    return Err(syn::Error::new_spanned(
                        name,
                        format!("Unsupported option {name_str}"),
                    ))
                }
            }
        } else {
            match &*name_str {
                "run_first" => Self::RunFirst(name),
                "run_last" => Self::RunLast(name),
                "run_cleanup" => Self::RunCleanup(name),
                "no_recurse" => Self::NoRecurse(name),
                "detailed" => Self::Detailed(name),
                "action" => Self::Action(name),
                "no_hooks" => Self::NoHooks(name),
                _ => {
                    return Err(syn::Error::new_spanned(
                        name,
                        format!("Unsupported option {name_str}"),
                    ))
                }
            }
        };

        Ok(result)
    }
}
impl SignalAttr {
    fn extract_items<'a>(
        attrs: impl IntoIterator<Item = &'a syn::Attribute>,
    ) -> syn::Result<Option<Vec<Self>>> {
        let attr = match attrs
            .into_iter()
            .find(|attr| attr.path().is_ident("signal"))
        {
            Some(attr) => attr,
            None => return Ok(None),
        };
        match &attr.meta {
            syn::Meta::Path(_) => Ok(Some(Vec::new())),
            syn::Meta::List(meta_list) => {
                let attrs: Punctuated<SignalAttr, Token![,]> =
                    meta_list.parse_args_with(Punctuated::parse_separated_nonempty)?;
                Ok(Some(attrs.into_iter().collect()))
            }
            syn::Meta::NameValue(_) => {
                return Err(syn::Error::new_spanned(
                    attr,
                    "expected #[signal] or #[signal(<arg1>, ...)]",
                ))
            }
        }
    }
}

bitflags! {
    #[derive(Default, Clone, Copy, PartialEq, Eq)]
    struct SignalFlags: u16 {
        const RUN_FIRST = 1 << 0;
        const RUN_LAST = 1 << 1;
        const RUN_CLEANUP = 1 << 2;
        const NO_RECURSE = 1 << 3;
        const DETAILED = 1 << 4;
        const ACTION = 1 << 5;
        const NO_HOOKS = 1 << 6;
    }
}

/// Full description of an eventual signal, based on the provided
/// method signature and signal tag info.
struct SignalDesc {
    name: String,
    rs_name: String,
    param_types: Vec<syn::Type>,
    return_type: Option<syn::Type>,
    flags: Vec<SignalAttr>,
    class_handler: Option<syn::Ident>,
}
impl SignalDesc {
    fn new(
        flags: Vec<SignalAttr>,
        signature: &syn::Signature,
        complete: bool,
    ) -> syn::Result<Self> {
        const EXPECT_SELF_REF: &str = "signal method must take &self as its first parameter";

        // ensure function takes &self first
        match signature.inputs.get(0) {
            Some(syn::FnArg::Receiver(syn::Receiver {
                reference: Some(_),
                mutability: None,
                ..
            })) => (),
            _ => return Err(syn::Error::new_spanned(signature, EXPECT_SELF_REF)),
        }

        // for now, get name from signature
        let rs_name = signature.ident.to_string();
        let name = rs_name.replace('_', "-");

        // parameters are remaining signature types
        let param_types = if signature.inputs.len() >= 2 {
            signature
                .inputs
                .iter()
                .skip(1)
                .map(|arg| match arg {
                    syn::FnArg::Receiver(_) => panic!("unexpected receiver"),
                    syn::FnArg::Typed(pat_type) => (&*pat_type.ty).clone(),
                })
                .collect()
        } else {
            Vec::new()
        };

        let return_type = match &signature.output {
            syn::ReturnType::Default => None,
            syn::ReturnType::Type(_, rt) => match &**rt {
                syn::Type::Tuple(syn::TypeTuple { elems, .. }) if elems.is_empty() => None,
                rt => Some(rt.clone()),
            },
        };

        let class_handler = complete.then(|| signature.ident.clone());

        Ok(Self {
            name,
            rs_name,
            param_types,
            return_type,
            flags,
            class_handler,
        })
    }
}

pub fn impl_signals(attr: Args, input: syn::ItemImpl) -> syn::Result<TokenStream> {
    if let Some((_, trait_path, _)) = &input.trait_ {
        return Err(syn::Error::new_spanned(trait_path, WRONG_PLACE_MSG));
    }
    let crate_name = crate_ident_new();

    let mut out_impl = input.clone();
    out_impl.items.clear();
    let mut out_signals = Vec::<SignalDesc>::new();

    // Extract signal methods
    for item in &input.items {
        match item {
            item @ syn::ImplItem::Fn(method) => {
                let flags = match SignalAttr::extract_items(&method.attrs)? {
                    Some(flags) => flags,
                    None => {
                        out_impl.items.push(item.clone());
                        continue;
                    }
                };
                let mut out_method = method.clone();
                out_method
                    .attrs
                    .retain(|item| !item.path().is_ident("signal"));
                out_impl.items.push(syn::ImplItem::Fn(out_method));

                let desc = SignalDesc::new(flags, &method.sig, true)?;
                out_signals.push(desc);
            }
            item @ syn::ImplItem::Verbatim(tokens) => {
                // try to parse as an incomplete function
                let method = match syn::parse2::<ImplItemIncompleteFn>(tokens.clone()) {
                    Ok(parsed) => parsed,
                    Err(_) => {
                        out_impl.items.push(item.clone());
                        continue;
                    }
                };
                // if it has the signal attribute, it's a signal
                // let the Rust compiler generate an error if not
                let flags = match SignalAttr::extract_items(&method.attrs)? {
                    Some(flags) => flags,
                    None => {
                        out_impl.items.push(item.clone());
                        continue;
                    }
                };
                let desc = SignalDesc::new(flags, &method.sig, false)?;
                out_signals.push(desc);
            }
            item => out_impl.items.push(item.clone()),
        }
    }

    // Implement DerivedObjectSignals
    let derive_signals = impl_object_signals(&crate_name, &*input.self_ty, &out_signals);

    // Implement wrapper type
    let wrapper_impl = impl_signal_wrapper(attr, &crate_name, &out_signals);

    Ok(quote::quote! {
        #out_impl
        #derive_signals
        #wrapper_impl
    })
}

fn impl_object_signals<'a>(
    glib: &TokenStream,
    ty: &syn::Type,
    signals: impl IntoIterator<
        Item = &'a SignalDesc,
        IntoIter = impl Iterator<Item = &'a SignalDesc> + ExactSizeIterator,
    >,
) -> TokenStream {
    let signal_iter = signals.into_iter();
    let count = signal_iter.len();
    let builders = signal_iter.map(|signal| {
        let name = syn::LitStr::new(&signal.name, Span::call_site());
        let param_types = match signal.param_types.is_empty() {
            true => None,
            false => {
                let param_types = &signal.param_types;
                Some(quote::quote! {
                    .param_types([#(<#param_types as #glib::types::StaticType>::static_type()),*])
                })
            }
        };
        let return_type = match signal.return_type.as_ref() {
            Some(rt) => Some(quote::quote! {
                .return_type::<#rt>()
            }),
            None => None,
        };
        let flags = signal.flags.iter().map(|item| match item {
            SignalAttr::RunFirst(ident) => quote::quote! {
                .#ident()
            },
            SignalAttr::RunLast(ident) => quote::quote! {
                .#ident()
            },
            SignalAttr::RunCleanup(ident) => quote::quote! {
                .#ident()
            },
            SignalAttr::NoRecurse(ident) => quote::quote! {
                .#ident()
            },
            SignalAttr::Detailed(ident) => quote::quote! {
                .#ident()
            },
            SignalAttr::Action(ident) => quote::quote! {
                .#ident()
            },
            SignalAttr::NoHooks(ident) => quote::quote! {
                .#ident()
            },
            SignalAttr::Accum(expr) => quote::quote! {
                .accumulator(#expr)
            },
        });
        let class_handler = match &signal.class_handler {
            Some(handler_fn) => {
                let mut param_idents: Vec<syn::Ident> =
                    Vec::with_capacity(signal.param_types.len());
                let mut param_stmts: Vec<TokenStream> =
                    Vec::with_capacity(signal.param_types.len());

                for i in 0..signal.param_types.len() {
                    let i_h = i + 1;
                    let ty = &signal.param_types[i];
                    let ident = quote::format_ident!("param{}", i);
                    let err_msg = Literal::string(&format!(
                        "Parameter {} for signal did not match ({})",
                        i_h,
                        ty.to_token_stream().to_string()
                    ));

                    let stmt = quote::quote! {
                        let #ident = values[#i_h].get::<#ty>()
                            .expect(#err_msg);
                    };

                    param_idents.push(ident);
                    param_stmts.push(stmt);
                }

                Some(quote::quote! {
                    .class_handler(|values| {
                        let this = values[0].get::<
                            <Self as #glib::subclass::types::ObjectSubclass>::Type
                        >()
                            .expect("`Self` parameter for signal did not match");
                        #(#param_stmts)*
                        let result = <
                            <Self as #glib::subclass::types::ObjectSubclass>::Type
                            as #glib::subclass::types::ObjectSubclassIsExt
                        >::imp(&this)
                            .#handler_fn(#(#param_idents),*);
                        None
                    })
                })
            }
            None => None,
        };
        quote::quote! {
            #glib::subclass::signal::Signal::builder(#name)
                #param_types
                #return_type
                #(#flags)*
                #class_handler
                .build()
        }
    });
    quote::quote! {
        #[automatically_derived]
        impl #glib::subclass::object::DerivedObjectSignals for #ty {
            fn derived_signals() -> &'static [glib::subclass::signal::Signal] {
                static SIGNALS: ::std::sync::OnceLock<[#glib::subclass::signal::Signal; #count]> =
                    ::std::sync::OnceLock::new();
                SIGNALS.get_or_init(|| {
                    [
                        #(#builders),*
                    ]
                })
            }
        }
    }
}

fn impl_signal_wrapper<'a>(
    args: Args,
    glib: &TokenStream,
    signals: impl IntoIterator<Item = &'a SignalDesc>,
) -> TokenStream {
    let signal_iter = signals.into_iter();
    let methods = signal_iter
        .map(|signal| {
            let connect_id = quote::format_ident!("connect_{}", &signal.rs_name);
            let emit_id = quote::format_ident!("emit_{}", &signal.rs_name);

            let signal_name = Literal::string(&signal.name);

            let closure_bound = {
                let param_types = &signal.param_types;
                let return_type = signal
                    .return_type
                    .as_ref()
                    .map_or_else(
                        || quote::quote! { () },
                        |value| value.to_token_stream()
                );
                quote::quote! {
                    Fn(&Self, #(#param_types),*) -> #return_type + 'static
                }
            };

            let param_types = &signal.param_types;

            let return_type = signal
                .return_type
                .as_ref()
                .map_or_else(
                    || quote::quote! { () },
                    |value| value.to_token_stream()
                );

            let closure_params: Vec<_> = (0..signal.param_types.len())
                .map(|i| quote::format_ident!("param{}", i))
                .collect();

            let emit_coda = signal
                .return_type
                .as_ref()
                .map_or_else(
                    || quote::quote! { ; },
                    |ty| quote::quote! {
                        .unwrap().get::<#ty>().unwrap()
                    }
                );


            quote::quote! {
                pub fn #connect_id<F: #closure_bound>(&self, f: F) -> #glib::SignalHandlerId {
                    <Self as #glib::object::ObjectExt>::connect_closure(
                        self,
                        #signal_name,
                        false,
                        #glib::closure_local!(|this: &Self, #(#closure_params: #param_types),*| -> #return_type {
                            f(this, #(#closure_params),*)
                        })
                    )
                }
                pub fn #emit_id(&self, #(#closure_params: #param_types),*) -> #return_type {
                    <Self as #glib::object::ObjectExt>::emit_by_name_with_values(
                        self,
                        #signal_name,
                        &[
                            #(#glib::value::ToValue::to_value(&#closure_params)),*
                        ]
                    )
                    #emit_coda
                }
            }
        });
    let ty = &args.wrapper_ty;

    quote::quote! {
        impl #ty {
            #(#methods)*
        }
    }
}
