// Take a look at the license at the top of the repository in the LICENSE file.

use crate::utils::crate_ident_new;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::format_ident;
use quote::{quote, quote_spanned};
use std::collections::HashMap;
use syn::ext::IdentExt;
use syn::parenthesized;
use syn::parse::Parse;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::Token;

pub struct PropsMacroInput {
    wrapper_ty: syn::Path,
    ident: syn::Ident,
    props: Vec<PropDesc>,
}

pub struct PropertiesAttr {
    _wrapper_ty_token: syn::Ident,
    _eq: Token![=],
    wrapper_ty: syn::Path,
}

impl Parse for PropertiesAttr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            _wrapper_ty_token: input.parse()?,
            _eq: input.parse()?,
            wrapper_ty: input.parse()?,
        })
    }
}

impl Parse for PropsMacroInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let derive_input: syn::DeriveInput = input.parse()?;
        let wrapper_ty = derive_input
            .attrs
            .iter()
            .find(|x| x.path.is_ident("properties"))
            .expect("missing #[properties(wrapper_type = ...)]");
        let wrapper_ty: PropertiesAttr = wrapper_ty.parse_args()?;
        let props: Vec<_> = match derive_input.data {
            syn::Data::Struct(struct_data) => parse_fields(struct_data.fields)?,
            _ => {
                return Err(syn::Error::new(
                    derive_input.span(),
                    "Properties can only be derived on structs",
                ))
            }
        };
        Ok(Self {
            wrapper_ty: wrapper_ty.wrapper_ty,
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
    // builder(required_params).parameter(value)
    // becomes
    // Builder(Punctuated(required_params), Optionals(TokenStream))
    Builder(Punctuated<syn::Expr, Token![,]>, TokenStream2),

    // ident [= expr]
    Get(Option<syn::Expr>),
    Set(Option<syn::Expr>),

    // ident = expr
    Type(syn::Type),

    // This will get translated from `ident = value` to `.ident(value)`
    // and will get appended after the `builder(...)` call.
    // ident [= expr]
    BuilderField((syn::Ident, Option<syn::Expr>)),

    // ident = ident
    Member(syn::Ident),

    // ident = "literal"
    Name(syn::LitStr),
}

impl Parse for PropAttr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = input.call(syn::Ident::parse_any)?;
        let name_str = name.to_string();

        let res = if input.peek(Token![=]) {
            let _assign_token: Token![=] = input.parse()?;
            // name = expr | type | ident
            match &*name_str {
                "name" => PropAttr::Name(input.parse()?),
                "get" => PropAttr::Get(Some(input.parse()?)),
                "set" => PropAttr::Set(Some(input.parse()?)),
                "type" => PropAttr::Type(input.parse()?),
                "member" => PropAttr::Member(input.parse()?),
                _ => PropAttr::BuilderField((name, Some(input.parse()?))),
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
                _ => {
                    return Err(syn::Error::new(
                        name.span(),
                        format!("Unsupported attribute list {name_str}(...)"),
                    ))
                }
            }
        } else {
            // attributes with only the identifier
            // name
            match &*name_str {
                "get" => PropAttr::Get(None),
                "set" => PropAttr::Set(None),
                "readwrite" | "read_only" | "write_only" => {
                    return Err(syn::Error::new(
                        name.span(),
                        format!(
                            "{name} is a flag managed by the Properties macro. \
                            Use `get` and `set` to manage read and write access to a property",
                        ),
                    ))
                }
                _ => PropAttr::BuilderField((name, None)),
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
    name: Option<syn::LitStr>,
    builder: Option<(Punctuated<syn::Expr, Token![,]>, TokenStream2)>,
    builder_fields: HashMap<syn::Ident, Option<syn::Expr>>,
}
impl ReceivedAttrs {
    fn new(
        attrs_span: &proc_macro2::Span,
        attrs: impl IntoIterator<Item = PropAttr>,
    ) -> syn::Result<Self> {
        let this = attrs.into_iter().fold(Self::default(), |mut this, attr| {
            this.set_from_attr(attr);
            this
        });

        if this.get.is_none() && this.set.is_none() {
            return Err(syn::Error::new(
                *attrs_span,
                "No `get` or `set` specified: at least one is required.".to_string(),
            ));
        }
        Ok(this)
    }
    fn set_from_attr(&mut self, attr: PropAttr) {
        match attr {
            PropAttr::Get(some_fn) => self.get = Some(some_fn.into()),
            PropAttr::Set(some_fn) => self.set = Some(some_fn.into()),
            PropAttr::Name(lit) => self.name = Some(lit),
            PropAttr::Type(ty) => self.ty = Some(ty),
            PropAttr::Member(member) => self.member = Some(member),
            PropAttr::Builder(required_params, optionals) => {
                self.builder = Some((required_params, optionals))
            }
            PropAttr::BuilderField((ident, expr)) => {
                self.builder_fields.insert(ident, expr);
            }
        }
    }
}

// It's a cleaned up version of `ReceivedAttrs` where some missing attributes get a default,
// generated value.
struct PropDesc {
    attrs_span: proc_macro2::Span,
    field_ident: syn::Ident,
    ty: syn::Type,
    name: syn::LitStr,
    get: Option<MaybeCustomFn>,
    set: Option<MaybeCustomFn>,
    member: Option<syn::Ident>,
    builder: Option<(Punctuated<syn::Expr, Token![,]>, TokenStream2)>,
    builder_fields: HashMap<syn::Ident, Option<syn::Expr>>,
}
impl PropDesc {
    fn new(
        attrs_span: proc_macro2::Span,
        field_ident: syn::Ident,
        field_ty: syn::Type,
        attrs: ReceivedAttrs,
    ) -> syn::Result<Self> {
        let ReceivedAttrs {
            get,
            set,
            ty,
            member,
            name,
            builder,
            builder_fields,
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
        Ok(Self {
            attrs_span,
            field_ident,
            get,
            set,
            ty,
            member,
            name,
            builder,
            builder_fields,
        })
    }
}

fn expand_properties_fn(props: &[PropDesc]) -> TokenStream2 {
    let n_props = props.len();
    let crate_ident = crate_ident_new();
    let properties_build_phase = props.iter().map(|prop| {
        let PropDesc {
            ty, name, builder, ..
        } = prop;

        let rw_flags = match (&prop.get, &prop.set) {
            (Some(_), Some(_)) => quote!(.readwrite()),
            (Some(_), None) => quote!(.read_only()),
            (None, Some(_)) => quote!(.write_only()),
            (None, None) => unreachable!("No `get` or `set` specified"),
        };

        let builder_call = builder
            .as_ref()
            .cloned()
            .map(|(mut required_params, chained_methods)| {
                let name_expr = syn::ExprLit {
                    attrs: vec![],
                    lit: syn::Lit::Str(name.to_owned()),
                };
                required_params.insert(0, name_expr.into());
                let required_params = required_params.iter();

                quote!((#(#required_params,)*)#chained_methods)
            })
            .unwrap_or(quote!((#name)));

        let builder_fields = prop.builder_fields.iter().map(|(k, v)| quote!(.#k(#v)));

        let span = prop.attrs_span;
        quote_spanned! {span=>
            <<#ty as #crate_ident::Property>::Value as #crate_ident::HasParamSpec>
                ::param_spec_builder() #builder_call
                #rw_flags
                #(#builder_fields)*
                .build()
        }
    });
    quote!(
        fn derived_properties() -> &'static [#crate_ident::ParamSpec] {
            use #crate_ident::once_cell::sync::Lazy;
            static PROPERTIES: Lazy<[#crate_ident::ParamSpec; #n_props]> = Lazy::new(|| [
                #(#properties_build_phase,)*
            ]);
            PROPERTIES.as_ref()
        }
    )
}
fn expand_property_fn(props: &[PropDesc]) -> TokenStream2 {
    let crate_ident = crate_ident_new();
    let match_branch_get = props.iter().flat_map(|p| {
        let PropDesc {
            name,
            field_ident,
            member,
            get,
            ..
        } = p;

        let enum_ident = name_to_enum_ident(name.value());
        let span = p.attrs_span;
        get.as_ref().map(|get| {
            let body = match (member, get) {
                (_, MaybeCustomFn::Custom(expr)) => quote!(
                    DerivedPropertiesEnum::#enum_ident => ::std::convert::From::from((#expr)(&self))
                ),
                (None, MaybeCustomFn::Default) => quote!(
                    DerivedPropertiesEnum::#enum_ident =>
                        #crate_ident::PropertyGet::get(&self.#field_ident, |v| ::std::convert::From::from(v))

                ),
                (Some(member), MaybeCustomFn::Default) => quote!(
                    DerivedPropertiesEnum::#enum_ident =>
                        #crate_ident::PropertyGet::get(&self.#field_ident, |v| ::std::convert::From::from(&v.#member))

                ),
            };
            quote_spanned!(span=> #body)
        })
    });
    quote!(
        fn derived_property(
            &self,
            id: usize,
            pspec: &#crate_ident::ParamSpec
        ) -> #crate_ident::Value {
            let prop = DerivedPropertiesEnum::try_from(id-1)
                .unwrap_or_else(|_| panic!("missing handler for property {}", pspec.name()));
            match prop {
                #(#match_branch_get,)*
                _ => unreachable!(),
            }
        }
    )
}
fn expand_set_property_fn(props: &[PropDesc]) -> TokenStream2 {
    let crate_ident = crate_ident_new();
    let match_branch_set = props.iter().flat_map(|p| {
        let PropDesc {
            name,
            field_ident,
            member,
            set,
            ..
        } = p;

        let crate_ident = crate_ident_new();
        let enum_ident = name_to_enum_ident(name.value());
        let span = p.attrs_span;
        let expect = quote!(.expect("Can't convert glib::value to property type"));
        set.as_ref().map(|set| {
            let body = match (member, set) {
                (_, MaybeCustomFn::Custom(expr)) => quote!(
                    DerivedPropertiesEnum::#enum_ident => {
                        (#expr)(&self, #crate_ident::Value::get(value)#expect);
                    }
                ),
                (None, MaybeCustomFn::Default) => quote!(
                    DerivedPropertiesEnum::#enum_ident => {
                        #crate_ident::PropertySet::set(
                            &self.#field_ident,
                            #crate_ident::Value::get(value)#expect
                        );
                    }
                ),
                (Some(member), MaybeCustomFn::Default) => quote!(
                    DerivedPropertiesEnum::#enum_ident => {
                        #crate_ident::PropertySetNested::set_nested(
                            &self.#field_ident,
                            move |v| v.#member = #crate_ident::Value::get(value)#expect
                        );
                    }
                ),
            };
            quote_spanned!(span=> #body)
        })
    });
    quote!(
        fn derived_set_property(&self,
            id: usize,
            value: &#crate_ident::Value,
            pspec: &#crate_ident::ParamSpec
        ){
            let prop = DerivedPropertiesEnum::try_from(id-1)
                .unwrap_or_else(|_| panic!("missing handler for property {}", pspec.name()));;
            match prop {
                #(#match_branch_set,)*
                _ => unreachable!(),
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
                .filter(|a| a.path.is_ident("property"))
                .map(move |attrs| {
                    let span = attrs.span();
                    let attrs = attrs.parse_args_with(
                        syn::punctuated::Punctuated::<PropAttr, Token![,]>::parse_terminated,
                    )?;
                    PropDesc::new(
                        span,
                        ident.as_ref().unwrap().clone(),
                        ty.clone(),
                        ReceivedAttrs::new(&span, attrs)?,
                    )
                })
        })
        .collect::<syn::Result<_>>()
}

/// Converts a glib property name to a correct rust ident
fn name_to_ident(name: &syn::LitStr) -> syn::Ident {
    format_ident!("{}", name.value().replace('-', "_"))
}

fn expand_getset_properties_impl(props: &[PropDesc]) -> TokenStream2 {
    let crate_ident = crate_ident_new();
    let defs = props.iter().map(|p| {
        let name = &p.name;
        let ident = name_to_ident(name);
        let ty = &p.ty;

        let getter = p.get.is_some().then(|| {
            quote!(pub fn #ident(&self) -> <#ty as #crate_ident::Property>::Value {
                 self.property::<<#ty as #crate_ident::Property>::Value>(#name)
            })
        });
        let is_construct_only = p.builder_fields.iter().any(|(k, _)| *k == "construct_only");
        let setter = (p.set.is_some() && !is_construct_only).then(|| {
            let ident = format_ident!("set_{}", ident);
            quote!(pub fn #ident<'a>(&self, value: impl std::borrow::Borrow<<<#ty as #crate_ident::Property>::Value as #crate_ident::HasParamSpec>::SetValue>) {
                self.set_property_from_value(#name, &::std::convert::From::from(std::borrow::Borrow::borrow(&value)))
            })
        });
        let span = p.attrs_span;
        quote_spanned!(span=>
            #getter
            #setter
        )
    });
    quote!(#(#defs)*)
}

fn expand_connect_prop_notify_impl(props: &[PropDesc]) -> TokenStream2 {
    let crate_ident = crate_ident_new();
    let connection_fns = props.iter().map(|p| {
        let name = &p.name;
        let fn_ident = format_ident!("connect_{}_notify", name_to_ident(name));
        let span = p.attrs_span;
        quote_spanned!(span=> pub fn #fn_ident<F: Fn(&Self) + 'static>(&self, f: F) -> #crate_ident::SignalHandlerId {
            self.connect_notify_local(Some(#name), move |this, _| {
                f(this)
            })
        })
    });
    quote!(#(#connection_fns)*)
}

fn expand_notify_impl(props: &[PropDesc]) -> TokenStream2 {
    let crate_ident = crate_ident_new();
    let emit_fns = props.iter().map(|p| {
        let name = &p.name;
        let fn_ident = format_ident!("notify_{}", name_to_ident(name));
        let span = p.attrs_span;
        let enum_ident = name_to_enum_ident(name.value());
        quote_spanned!(span=> pub fn #fn_ident(&self) {
            self.notify_by_pspec(
                &<<Self as #crate_ident::object::ObjectSubclassIs>::Subclass
                    as #crate_ident::subclass::object::DerivedObjectProperties>::derived_properties()
                [DerivedPropertiesEnum::#enum_ident as usize]
            );
        })
    });
    quote!(#(#emit_fns)*)
}

fn name_to_enum_ident(mut name: String) -> syn::Ident {
    let mut slice = name.as_mut_str();
    while let Some(i) = slice.find('-') {
        let (head, tail) = slice.split_at_mut(i);
        if let Some(c) = head.get_mut(0..1) {
            c.make_ascii_uppercase();
        }
        slice = &mut tail[1..];
    }
    if let Some(c) = slice.get_mut(0..1) {
        c.make_ascii_uppercase();
    }
    let enum_member: String = name.split('-').collect();
    format_ident!("{}", enum_member)
}

fn expand_properties_enum(props: &[PropDesc]) -> TokenStream2 {
    let properties: Vec<syn::Ident> = props
        .iter()
        .map(|p| {
            let name: String = p.name.value();
            name_to_enum_ident(name)
        })
        .collect();
    let props = properties.iter();
    let indices = 0..properties.len();
    quote! {
        #[repr(usize)]
        #[derive(Debug, Copy, Clone)]
        enum DerivedPropertiesEnum {
            #(#props,)*
        }
        impl std::convert::TryFrom<usize> for DerivedPropertiesEnum {
            type Error = usize;

            fn try_from(item: usize) -> Result<Self, Self::Error> {
                match item {
                    #(#indices => Ok(Self::#properties),)*
                    _ => Err(item)
                }
            }
        }
    }
}

pub fn impl_derive_props(input: PropsMacroInput) -> TokenStream {
    let struct_ident = &input.ident;
    let crate_ident = crate_ident_new();
    let wrapper_type = input.wrapper_ty;
    let fn_properties = expand_properties_fn(&input.props);
    let fn_property = expand_property_fn(&input.props);
    let fn_set_property = expand_set_property_fn(&input.props);
    let getset_properties_impl = expand_getset_properties_impl(&input.props);
    let connect_prop_notify_impl = expand_connect_prop_notify_impl(&input.props);
    let notify_impl = expand_notify_impl(&input.props);
    let properties_enum = expand_properties_enum(&input.props);

    let expanded = quote! {
        use #crate_ident::{PropertyGet, PropertySet, ToValue};

        #properties_enum

        impl #crate_ident::subclass::object::DerivedObjectProperties for #struct_ident {
            #fn_properties
            #fn_property
            #fn_set_property
        }

        impl #wrapper_type {
            #getset_properties_impl
            #connect_prop_notify_impl
            #notify_impl
        }

    };
    proc_macro::TokenStream::from(expanded)
}
