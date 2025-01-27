use bitflags::bitflags;
use proc_macro2::{Punct, Span, TokenStream, TokenTree};
use syn::{ext::IdentExt, parse::Parse, Token};

use crate::utils::crate_ident_new;

pub const WRONG_PLACE_MSG: &str =
    "This macro should be used on a plain `impl` block of the inner object type";

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
    RunFirst,
    RunLast,
    RunCleanup,
    NoRecurse,
    Detailed,
    Action,
    NoHooks,
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
                "run_first" => Self::RunFirst,
                "run_last" => Self::RunLast,
                "run_cleanup" => Self::RunCleanup,
                "no_recurse" => Self::NoRecurse,
                "detailed" => Self::Detailed,
                "action" => Self::Action,
                "no_hooks" => Self::NoHooks,
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

/// Collected info from the #[signal(...)] attribute.
#[derive(Default)]
struct SignalTagInfo {
    flags: SignalFlags,
    accum: Option<syn::Expr>,
}
impl SignalTagInfo {
    fn set_from_attr(&mut self, attr: SignalAttr) {
        match attr {
            SignalAttr::RunFirst => self.flags |= SignalFlags::RUN_FIRST,
            SignalAttr::RunLast => self.flags |= SignalFlags::RUN_LAST,
            SignalAttr::RunCleanup => self.flags |= SignalFlags::RUN_CLEANUP,
            SignalAttr::NoRecurse => self.flags |= SignalFlags::NO_RECURSE,
            SignalAttr::Detailed => self.flags |= SignalFlags::DETAILED,
            SignalAttr::Action => self.flags |= SignalFlags::ACTION,
            SignalAttr::NoHooks => self.flags |= SignalFlags::NO_HOOKS,
            SignalAttr::Accum(expr) => self.accum = Some(expr),
        }
    }
}
impl SignalTagInfo {
    fn extract_from<'a, I: IntoIterator<Item = &'a syn::Attribute>>(
        attrs: I,
    ) -> syn::Result<Option<Self>> {
        let attribute = match attrs
            .into_iter()
            .find(|attr| attr.path().is_ident("signal"))
        {
            Some(attr) => attr,
            None => return Ok(None),
        };
        match &attribute.meta {
            syn::Meta::Path(_) => Ok(Some(Self::default())),
            syn::Meta::List(meta_list) => Ok(Some(meta_list.parse_args::<Self>()?)),
            syn::Meta::NameValue(meta_name_value) => {
                return Err(syn::Error::new_spanned(
                    meta_name_value,
                    "Invalid #[signal] syntax",
                ))
            }
        }
    }
}
impl Parse for SignalTagInfo {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs =
            syn::punctuated::Punctuated::<SignalAttr, Token![,]>::parse_separated_nonempty(input)?;

        let mut value = Self::default();
        for attr in attrs {
            value.set_from_attr(attr);
        }
        Ok(value)
    }
}

/// Full description of an eventual signal, based on the provided
/// method signature and signal tag info.
struct SignalDesc {
    name: String,
    param_types: Vec<syn::Type>,
    return_type: Option<syn::Type>,
    flags: SignalFlags,
    class_handler: Option<syn::Ident>,
    accum: Option<syn::Expr>,
}
impl SignalDesc {
    fn new(
        tag_info: SignalTagInfo,
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
        let name = signature.ident.to_string().replace('_', "-");

        // parameters are remaining signature types
        let param_types = if signature.inputs.len() >= 2 {
            signature
                .inputs
                .iter()
                .skip(1)
                .map(|arg| match arg {
                    syn::FnArg::Receiver(receiver) => panic!("unexpected receiver"),
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

        let flags = tag_info.flags;

        let class_handler = complete.then(|| signature.ident.clone());

        let accum = tag_info.accum;

        Ok(Self {
            name,
            param_types,
            return_type,
            flags,
            class_handler,
            accum,
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
                let attr_info = match SignalTagInfo::extract_from(&method.attrs)? {
                    Some(attr_info) => attr_info,
                    None => {
                        out_impl.items.push(item.clone());
                        continue;
                    }
                };
                let desc = SignalDesc::new(attr_info, &method.sig, true)?;
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
                let attr_info = match SignalTagInfo::extract_from(&method.attrs)? {
                    Some(attr_info) => attr_info,
                    None => {
                        out_impl.items.push(item.clone());
                        continue;
                    }
                };
                let desc = SignalDesc::new(attr_info, &method.sig, false)?;
                out_signals.push(desc);
            }
            item => out_impl.items.push(item.clone()),
        }
    }

    // Implement DerivedObjectSignals
    let derive_signals = impl_object_signals(&crate_name, &*input.self_ty, &out_signals);

    // Implement wrapper type

    Ok(quote::quote! {
        #out_impl
        #derive_signals
    })
}

fn impl_object_signals<'a>(
    glib: &TokenStream,
    ty: &syn::Type,
    signals: impl IntoIterator<Item = &'a SignalDesc>,
) -> TokenStream {
    let builders: Vec<_> = signals
        .into_iter()
        .map(|signal| {
            let name = syn::LitStr::new(&signal.name, Span::call_site());
            let run_first = signal.flags.contains(SignalFlags::RUN_FIRST).then(|| {
                quote::quote! {
                    .run_first()
                }
            });
            let run_last = signal.flags.contains(SignalFlags::RUN_LAST).then(|| {
                quote::quote! {
                    .run_last()
                }
            });
            let run_cleanup = signal.flags.contains(SignalFlags::RUN_CLEANUP).then(|| {
                quote::quote! {
                    .run_cleanup()
                }
            });
            let no_recurse = signal.flags.contains(SignalFlags::NO_RECURSE).then(|| {
                quote::quote! {
                    .no_recurse()
                }
            });
            let detailed = signal.flags.contains(SignalFlags::DETAILED).then(|| {
                quote::quote! {
                    .detailed()
                }
            });
            let action = signal.flags.contains(SignalFlags::ACTION).then(|| {
                quote::quote! {
                    .action()
                }
            });
            let no_hooks = signal.flags.contains(SignalFlags::NO_HOOKS).then(|| {
                quote::quote! {
                    .no_hooks()
                }
            });
            quote::quote! {
                #glib::subclass::signal::Signal::builder(#name)
                    #run_first
                    #run_last
                    #run_cleanup
                    #no_recurse
                    #detailed
                    #action
                    #no_hooks
                    .build()
            }
        })
        .collect();
    let count = builders.len();
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
