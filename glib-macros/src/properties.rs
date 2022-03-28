// Take a look at the license at the top of the repository in the LICENSE file.

use crate::utils::crate_ident_new;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::format_ident;
use quote::quote;
use std::str::FromStr;
use syn::ext::IdentExt;
use syn::parenthesized;
use syn::parse::Parse;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::Token;

pub struct PropsMacroInput {
    ident: syn::Ident,
    props: Vec<PropDesc>,
}

impl Parse for PropsMacroInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let derive_input: syn::DeriveInput = input.parse()?;
        let props: Vec<_> = match derive_input.data {
            syn::Data::Struct(struct_data) => parse_fields(struct_data.fields)?,
            _ => {
                return Err(syn::Error::new(
                    derive_input.span(),
                    "props can only be derived on structs",
                ))
            }
        };
        Ok(Self {
            ident: derive_input.ident,
            props,
        })
    }
}

enum MaybeCustomFn {
    Custom(Box<syn::Expr>),
    Default,
}

impl std::convert::From<Option<syn::Expr>> for MaybeCustomFn {
    fn from(item: Option<syn::Expr>) -> Self {
        match item {
            Some(expr) => Self::Custom(Box::new(expr)),
            None => Self::Default,
        }
    }
}

enum PropAttr {
    Flag(&'static str),

    // builder(required_params).parameter(value)
    // becomes
    // Builder(Punctuated(required_params), Optionals(TokenStream))
    Builder(Punctuated<syn::Expr, Token![,]>, TokenStream2),

    // ident [= expr]
    Get(Option<syn::Expr>),
    Set(Option<syn::Expr>),

    // ident = expr
    Type(syn::Type),

    // ident = ident
    Member(syn::Ident),

    // ident = "literal"
    Name(syn::LitStr),
    Nick(syn::LitStr),
    Blurb(syn::LitStr),
}

const FLAGS: [&str; 16] = [
    "READABLE",
    "WRITABLE",
    "READWRITE",
    "CONSTRUCT",
    "CONSTRUCT_ONLY",
    "LAX_VALIDATION",
    "USER_1",
    "USER_2",
    "USER_3",
    "USER_4",
    "USER_5",
    "USER_6",
    "USER_7",
    "USER_8",
    "EXPLICIT_NOTIFY",
    "DEPRECATED",
];
impl Parse for PropAttr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = input.call(syn::Ident::parse_any)?;
        let name_str = name.to_string();

        let res = if input.peek(Token![=]) {
            let _assign_token: Token![=] = input.parse()?;
            if input.peek(syn::LitStr) {
                let lit: syn::LitStr = input.parse()?;
                // name = "literal"
                match &*name_str {
                    "name" => PropAttr::Name(lit),
                    "nick" => PropAttr::Nick(lit),
                    "blurb" => PropAttr::Blurb(lit),
                    _ => {
                        panic!("Invalid attribute for property")
                    }
                }
            } else {
                // name = expr | type | ident
                match &*name_str {
                    "get" => PropAttr::Get(Some(input.parse()?)),
                    "set" => PropAttr::Set(Some(input.parse()?)),
                    "type" => PropAttr::Type(input.parse()?),
                    "member" => PropAttr::Member(input.parse()?),
                    _ => {
                        panic!("Invalid attribute for property")
                    }
                }
            }
        } else if input.peek(syn::token::Paren) {
            match &*name_str {
                "builder" => {
                    let content;
                    parenthesized!(content in input);
                    let required = content.parse_terminated(syn::Expr::parse)?;
                    let rest: TokenStream2 = input.parse()?;
                    PropAttr::Builder(required, rest)
                }
                _ => panic!("Unsupported attribute list {}(...)", name_str),
            }
        } else {
            // attributes with only the identifier
            // name
            match &*name_str {
                "get" => PropAttr::Get(None),
                "set" => PropAttr::Set(None),
                name => {
                    if let Some(flag) = FLAGS.iter().find(|x| *x == &name.to_uppercase()) {
                        PropAttr::Flag(flag)
                    } else {
                        panic!("Invalid attribute for property")
                    }
                }
            }
        };
        Ok(res)
    }
}

#[derive(Default)]
struct ReceivedAttrs {
    get: Option<MaybeCustomFn>,
    set: Option<MaybeCustomFn>,
    ty: Option<syn::Type>,
    member: Option<syn::Ident>,
    flags: Vec<&'static str>,
    name: Option<syn::LitStr>,
    nick: Option<syn::LitStr>,
    blurb: Option<syn::LitStr>,
    builder: Option<(Punctuated<syn::Expr, Token![,]>, TokenStream2)>,
}
impl ReceivedAttrs {
    fn new(attrs: impl IntoIterator<Item = PropAttr>) -> Self {
        attrs.into_iter().fold(Self::default(), |mut this, attr| {
            this.set_from_attr(attr);
            this
        })
    }
    fn set_from_attr(&mut self, attr: PropAttr) {
        match attr {
            PropAttr::Get(some_fn) => self.get = Some(some_fn.into()),
            PropAttr::Set(some_fn) => self.set = Some(some_fn.into()),
            PropAttr::Name(lit) => self.name = Some(lit),
            PropAttr::Nick(lit) => self.nick = Some(lit),
            PropAttr::Blurb(lit) => self.blurb = Some(lit),
            PropAttr::Type(ty) => self.ty = Some(ty),
            PropAttr::Member(member) => self.member = Some(member),
            PropAttr::Flag(flag) => self.flags.push(flag),
            PropAttr::Builder(required_params, optionals) => {
                self.builder = Some((required_params, optionals))
            }
        }
    }
}
struct PropDesc {
    field_ident: syn::Ident,
    ty: syn::Type,
    name: syn::LitStr,
    nick: Option<syn::LitStr>,
    blurb: Option<syn::LitStr>,
    get: Option<MaybeCustomFn>,
    set: Option<MaybeCustomFn>,
    member: Option<syn::Ident>,
    flags: Vec<&'static str>,
    builder: Option<(Punctuated<syn::Expr, Token![,]>, TokenStream2)>,
}
impl PropDesc {
    fn new(field_ident: syn::Ident, field_ty: syn::Type, attrs: ReceivedAttrs) -> Self {
        let ReceivedAttrs {
            get,
            set,
            ty,
            member,
            flags,
            name,
            nick,
            blurb,
            builder,
        } = attrs;

        // Fill needed, but missing, attributes with calculated default values
        let name = name.unwrap_or_else(|| {
            syn::LitStr::new(
                &field_ident.to_string().trim_matches('_').replace('_', "-"),
                field_ident.span(),
            )
        });
        let ty = ty.unwrap_or_else(|| field_ty.clone());

        // Now that everything is set and safe, return the final proprety description
        Self {
            field_ident,
            get,
            set,
            ty,
            member,
            flags,
            name,
            nick,
            blurb,
            builder,
        }
    }
}

fn expand_properties_fn(props: &[PropDesc]) -> TokenStream2 {
    let n_props = props.len();
    let properties_build_phase = props.iter().map(|prop| {
        let PropDesc {
            ty,
            name,
            nick,
            blurb,
            builder,
            ..
        } = prop;

        let flags = {
            let write = prop.set.as_ref().map(|_| quote!(WRITABLE));
            let read = prop.get.as_ref().map(|_| quote!(READABLE));

            let flags_iter = [write, read].into_iter().flatten().chain(
                prop.flags
                    .iter()
                    .map(|f| TokenStream2::from_str(f).unwrap()),
            );
            quote!(glib::ParamFlags::empty() #(| glib::ParamFlags::#flags_iter)*)
        };

        let builder_call = builder
            .as_ref()
            .cloned()
            .map(|(mut required_params, opts)| {
                let name_expr = syn::ExprLit {
                    attrs: vec![],
                    lit: syn::Lit::Str(name.to_owned()),
                };
                required_params.insert(0, name_expr.into());
                let required_params = required_params.iter();

                quote!((#(#required_params,)*)#opts)
            })
            .unwrap_or(quote!((#name)));

        let build_nick = nick.as_ref().map(|x| quote!(.nick(#x)));
        let build_blurb = blurb.as_ref().map(|x| quote!(.blurb(#x)));
        quote! {
            <<#ty as glib::Property>::ParamSpec>
                ::builder #builder_call
                #build_nick
                #build_blurb
                .flags(#flags)
                .build()
        }
    });
    quote!(
        fn derived_properties() -> &'static [glib::ParamSpec] {
            use glib::once_cell::sync::Lazy;
            static PROPERTIES: Lazy<[glib::ParamSpec; #n_props]> = Lazy::new(|| [
                #(#properties_build_phase,)*
            ]);
            PROPERTIES.as_ref()
        }
    )
}
fn expand_property_fn(props: &[PropDesc]) -> TokenStream2 {
    let match_branch_get = props.iter().flat_map(|p| {
        let PropDesc {
            name,
            field_ident,
            member,
            get,
            ..
        } = p;
        get.as_ref().map(|get| match (member, get) {
            (_, MaybeCustomFn::Custom(expr)) => quote!(
                #name => Ok((#expr)(&self).to_value())
            ),
            (None, MaybeCustomFn::Default) => quote!(
                #name => Ok(self.#field_ident.get(|v| v.to_value()))
            ),
            (Some(member), MaybeCustomFn::Default) => quote!(
                #name => Ok(self.#field_ident.get(|v| v.#member.to_value()))
            ),
        })
    });
    quote!(
        fn derived_property<'a>(&self, _obj: &Self::Type, _id: usize, pspec: &'a glib::ParamSpec) -> Result<glib::Value, glib::subclass::object::MissingPropertyHandler<'a>> {
            match pspec.name() {
                #(#match_branch_get,)*
                p => Err(pspec.into())
            }
        }
    )
}
fn expand_set_property_fn(props: &[PropDesc]) -> TokenStream2 {
    let match_branch_set = props.iter().flat_map(|p| {
        let PropDesc {
            name,
            field_ident,
            member,
            set,
            ..
        } = p;

        let expect = quote!(.expect("Can't convert glib::value to property type"));
        set.as_ref().map(|set| match (member, set) {
            (_, MaybeCustomFn::Custom(expr)) => quote!(
                #name => {
                    (#expr)(&self, value.get()#expect);
                    Ok(())
                }
            ),
            (None, MaybeCustomFn::Default) => quote!(
                #name => {
                    self.#field_ident.set(move |v| *v = value.get()#expect);
                    Ok(())
                }
            ),
            (Some(member), MaybeCustomFn::Default) => quote!(
                #name => {
                    self.#field_ident.set(move |v| v.#member = value.get()#expect);
                    Ok(())
                }
            ),
        })
    });
    quote!(
        fn derived_set_property<'a>(&self, _obj: &Self::Type, _id: usize, value: &glib::Value, pspec: &'a glib::ParamSpec) -> Result<(), glib::subclass::object::MissingPropertyHandler<'a>> {
            match pspec.name() {
                #(#match_branch_set,)*
                p => Err(pspec.into())
            }
        }
    )
}

fn parse_fields(fields: syn::Fields) -> syn::Result<Vec<PropDesc>> {
    fields
        .into_iter()
        .flat_map(|field| {
            let syn::Field {
                ident, attrs, ty, ..
            } = field;
            attrs
                .into_iter()
                .filter(|a| a.path.is_ident("prop"))
                .map(move |attrs| {
                    let attrs = attrs.parse_args_with(
                        syn::punctuated::Punctuated::<PropAttr, Token![,]>::parse_terminated,
                    )?;
                    Ok(PropDesc::new(
                        ident.as_ref().unwrap().clone(),
                        ty.clone(),
                        ReceivedAttrs::new(attrs),
                    ))
                })
        })
        .collect::<syn::Result<_>>()
}

/// Converts a glib property name to a correct rust ident
fn name_to_ident(name: &syn::LitStr) -> syn::Ident {
    format_ident!("{}", name.value().replace('-', "_"))
}

fn getter_prototype(ident: &syn::Ident, ty: &syn::Type) -> TokenStream2 {
    quote!(fn #ident(&self) -> <#ty as glib::Property>::Value)
}
fn setter_prototype(ident: &syn::Ident, ty: &syn::Type) -> TokenStream2 {
    let ident = format_ident!("set_{}", ident);
    quote!(fn #ident(&self, value: <#ty as glib::Property>::Value))
}
fn expand_getset_properties_def(props: &[PropDesc]) -> TokenStream2 {
    let defs = props
        .iter()
        .flat_map(|p| {
            let ident = name_to_ident(&p.name);
            let getter = p.get.is_some().then(|| getter_prototype(&ident, &p.ty));
            let setter = p.set.is_some().then(|| setter_prototype(&ident, &p.ty));
            [getter, setter]
        })
        .flatten();
    quote!(#(#defs;)*)
}

fn expand_getset_properties_impl(props: &[PropDesc]) -> TokenStream2 {
    let defs = props.iter().map(|p| {
        let name = &p.name;
        let ident = name_to_ident(name);
        let ty = &p.ty;

        let getter = p.get.is_some().then(|| {
            let body = quote!(self.property::<<#ty as glib::Property>::Value>(#name));
            let fn_prototype = getter_prototype(&ident, ty);
            quote!(#fn_prototype { #body })
        });
        let setter = p.set.is_some().then(|| {
            let body = quote!(self.set_property::<<#ty as glib::Property>::Value>(#name, value));
            let fn_prototype = setter_prototype(&ident, ty);
            quote!(#fn_prototype { #body })
        });
        quote!(
            #getter
            #setter
        )
    });
    quote!(#(#defs)*)
}

fn expand_connect_prop_notify(p: &PropDesc) -> TokenStream2 {
    let name = &p.name;
    let fn_ident = format_ident!("connect_{}_notify", name_to_ident(name));
    quote!(fn #fn_ident<F: Fn(&Self) + 'static>(&self, f: F) -> glib::SignalHandlerId)
}
fn expand_connect_prop_notify_def(props: &[PropDesc]) -> TokenStream2 {
    let connection_fns = props.iter().map(expand_connect_prop_notify);
    quote!(#(#connection_fns;)*)
}
fn expand_connect_prop_notify_impl(props: &[PropDesc]) -> TokenStream2 {
    let connection_fns = props.iter().map(|p| {
        let name = &p.name;
        let fn_prototype = expand_connect_prop_notify(p);
        quote!(#fn_prototype {
            self.connect_notify_local(Some(#name), move |this, _| {
                f(this)
            })
        })
    });
    quote!(#(#connection_fns)*)
}

pub fn impl_derive_props(input: PropsMacroInput) -> TokenStream {
    let struct_ident = &input.ident;
    let struct_ident_ext = format_ident!("{}PropertiesExt", &input.ident);
    let crate_ident = crate_ident_new();
    let wrapper_type = quote!(<#struct_ident as glib::subclass::types::ObjectSubclass>::Type);
    let fn_properties = expand_properties_fn(&input.props);
    let fn_property = expand_property_fn(&input.props);
    let fn_set_property = expand_set_property_fn(&input.props);
    let getset_properties_def = expand_getset_properties_def(&input.props);
    let getset_properties_impl = expand_getset_properties_impl(&input.props);
    let connect_prop_notify_def = expand_connect_prop_notify_def(&input.props);
    let connect_prop_notify_impl = expand_connect_prop_notify_impl(&input.props);
    let expanded = quote! {
        use glib::{PropertyRead, PropertyWrite};

        impl glib::subclass::object::DerivedObjectProperties for #struct_ident {
            #fn_properties
            #fn_property
            #fn_set_property
        }

        pub trait #struct_ident_ext {
            #getset_properties_def
            #connect_prop_notify_def
        }
        impl<T: #crate_ident::IsA<#wrapper_type>> #struct_ident_ext for T {
            #getset_properties_impl
            #connect_prop_notify_impl
        }

    };
    proc_macro::TokenStream::from(expanded)
}
