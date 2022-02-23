// Take a look at the license at the top of the repository in the LICENSE file.

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::format_ident;
use quote::quote;
use quote::ToTokens;
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
            syn::Data::Struct(struct_data) => parse_fields(struct_data.fields).collect(),
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
        fn properties() -> &'static [glib::ParamSpec] {
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
        match (member, get) {
            (_, Some(MaybeCustomFn::Custom(expr))) => {
                Some(quote!(#name => (#expr)(&self).to_value()))
            }
            (None, Some(MaybeCustomFn::Default)) => Some(quote!(
                    #name => self.#field_ident.get(|v| v.to_value())
            )),
            (Some(member), Some(MaybeCustomFn::Default)) => Some(quote!(
                    #name => self.#field_ident.get(|v| v.#member.to_value())
            )),
            _ => None,
        }
    });
    quote!(
        fn property(&self, _obj: &Self::Type, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
            match pspec.name() {
                #(#match_branch_get,)*
                p => unreachable!("Invalid property {}", p)
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
        match (member, set) {
            (_, Some(MaybeCustomFn::Custom(expr))) => {
                Some(quote!(#name => (#expr)(&self, value.get().unwrap())))
            }
            (None, Some(MaybeCustomFn::Default)) => Some(quote!(
                #name => self.#field_ident.set(move |v| *v = value.get()
                            .expect("Can't convert glib::value to property type"))
            )),
            (Some(member), Some(MaybeCustomFn::Default)) => Some(quote!(
                #name => self.#field_ident.set(move |v| v.#member = value.get()
                            .expect("Can't convert glib::value to property type"))
            )),
            (_, None) => None,
        }
    });
    quote!(
        fn set_property(&self, _obj: &Self::Type, _id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
            match pspec.name() {
                #(#match_branch_set,)*
                p => unreachable!("Invalid property {}", p)
            }
        }
    )
}

fn parse_fields(fields: syn::Fields) -> impl Iterator<Item = PropDesc> {
    fields.into_iter().flat_map(|field| {
        let syn::Field {
            ident, attrs, ty, ..
        } = field;
        attrs
            .into_iter()
            .filter(|a| a.path.is_ident("prop"))
            .flat_map(|attrs| {
                attrs.parse_args_with(
                    syn::punctuated::Punctuated::<PropAttr, Token![,]>::parse_terminated,
                )
            })
            .map(move |attrs| {
                PropDesc::new(
                    ident.as_ref().unwrap().clone(),
                    ty.clone(),
                    ReceivedAttrs::new(attrs),
                )
            })
    })
}

/// Converts a glib property name to a correct rust ident
fn name_to_ident(name: &syn::LitStr) -> syn::Ident {
    format_ident!("{}", name.value().replace('-', "_"))
}

/// Changes `Self` to another concrete type, to make it work in a different scope
fn change_self_type<T: ToTokens>(source: &T, ident: &str) -> TokenStream2 {
    TokenStream2::from_str(&source.to_token_stream().to_string().replace("Self", ident)).unwrap()
}

fn expand_getset_properties_def(props: &[PropDesc]) -> TokenStream2 {
    let defs = props.iter().map(|p| {
        let ident = name_to_ident(&p.name);
        let set_ident = format_ident!("set_{}", ident);
        let ty = &p.ty;
        let getter = p
            .get
            .is_some()
            .then(|| quote!(fn #ident(&self) -> <#ty as glib::Property>::Value;));
        let setter = p
            .set
            .is_some()
            .then(|| quote!(fn #set_ident(&self, value: <#ty as glib::Property>::Value);));
        quote!(
            #getter
            #setter
        )
    });
    quote!(#(#defs)*)
}

fn expand_getset_properties_impl(imp_type_ident: &syn::Ident, props: &[PropDesc]) -> TokenStream2 {
    let defs = props.iter().map(|p| {
        let ident = name_to_ident(&p.name);
        let set_ident = format_ident!("set_{}", ident);
        let field_ident = &p.field_ident;
        let ty = &p.ty;

        let getter = p.get.as_ref().map(|mfn| {
            let body = match (p.member.as_ref(), mfn) {
                (None, MaybeCustomFn::Default) => quote!(
                     self.imp().#field_ident.get(|x| x.to_owned())
                ),
                (Some(member), MaybeCustomFn::Default) => quote!(
                     self.imp().#field_ident.get(|x| x.#member.to_owned())
                ),
                (_, MaybeCustomFn::Custom(custom_fn)) => {
                    let custom_fn = change_self_type(custom_fn, &imp_type_ident.to_string());
                    quote!((#custom_fn)(&self.imp()))
                }
            };
            quote!(fn #ident(&self) -> <#ty as glib::Property>::Value {
                #body
            })
        });
        let setter = p.set.as_ref().map(|mfn| {
            let body = match (p.member.as_ref(), mfn) {
                (None, MaybeCustomFn::Default) => quote!(
                     self.imp().#field_ident.set(move |x| *x = value)
                ),
                (Some(member), MaybeCustomFn::Default) => quote!(
                     self.imp().#field_ident.set(move |x| x.#member = value)
                ),
                (_, MaybeCustomFn::Custom(custom_fn)) => {
                    let custom_fn = change_self_type(custom_fn, &imp_type_ident.to_string());
                    quote!((#custom_fn)(&self.imp(), value))
                }
            };
            quote!(fn #set_ident(&self, value: <#ty as glib::Property>::Value) {
                #body
            })
        });
        quote!(
            #getter
            #setter
        )
    });
    quote!(#(#defs)*)
}

pub fn impl_derive_props(input: PropsMacroInput) -> TokenStream {
    let struct_ident = &input.ident;
    let struct_ident_ext = format_ident!("{}Ext", &input.ident);
    let wrapper_type = quote!(<#struct_ident as glib::subclass::types::ObjectSubclass>::Type);
    let fn_properties = expand_properties_fn(&input.props);
    let fn_property = expand_property_fn(&input.props);
    let fn_set_property = expand_set_property_fn(&input.props);
    let getset_properties_def = expand_getset_properties_def(&input.props);
    let getset_properties_impl = expand_getset_properties_impl(struct_ident, &input.props);
    let expanded = quote! {
        use glib::{PropRead, PropWrite};
        impl ObjectImpl for #struct_ident {
            #fn_properties
            #fn_property
            #fn_set_property
        }

        pub trait #struct_ident_ext {
            #getset_properties_def
        }
        impl #struct_ident_ext for #wrapper_type {
            #getset_properties_impl
        }

    };
    println!("{}", expanded);
    proc_macro::TokenStream::from(expanded)
}
