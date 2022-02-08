// Take a look at the license at the top of the repository in the LICENSE file.

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use quote::ToTokens;
use syn::ext::IdentExt;
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
    CustomFn(syn::Expr),
    DefaultFn,
}

enum PropAttr {
    // flags(ident, ident, ident)
    Flags(syn::punctuated::Punctuated<syn::Ident, Token![,]>),

    // ident [= expr]
    Get(Option<syn::Expr>),
    Set(Option<syn::Expr>),

    // ident = expr
    DefaultVal(syn::Expr),
    Type(syn::Type),

    // ident = ident
    Member(syn::Ident),

    // ident = "literal"
    Name(syn::LitStr),
    Nick(syn::LitStr),
    Blurb(syn::LitStr),
}

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
                    "default" => PropAttr::DefaultVal(input.parse()?),
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
                "flags" => {
                    let content;
                    syn::parenthesized!(content in input);
                    PropAttr::Flags(content.call(
                        syn::punctuated::Punctuated::<syn::Ident, Token![,]>::parse_terminated,
                    )?)
                }
                _ => panic!("Unsupported attribute list {}(...)", name_str),
            }
        } else {
            // attributes with only the identifier
            // name
            match &*name_str {
                "get" => PropAttr::Get(None),
                "set" => PropAttr::Set(None),
                _ => {
                    panic!("Invalid attribute for property")
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
    flags: Punctuated<syn::Ident, Token![,]>,
    // These are not syn::LitStr because `name` may be set by default with `field.ident`
    name: Option<syn::LitStr>,
    nick: Option<syn::LitStr>,
    blurb: Option<syn::LitStr>,
    default: Option<syn::Expr>,
}
impl ReceivedAttrs {
    fn new(attrs: impl IntoIterator<Item = PropAttr>) -> Self {
        let mut this = Self::default();
        for attr in attrs {
            this.set_from_attr(attr);
        }
        this
    }
    fn set_from_attr(&mut self, attr: PropAttr) {
        match attr {
            PropAttr::Get(Some(expr)) => self.get = Some(MaybeCustomFn::CustomFn(expr)),
            PropAttr::Get(None) => self.get = Some(MaybeCustomFn::DefaultFn),
            PropAttr::Set(Some(expr)) => self.set = Some(MaybeCustomFn::CustomFn(expr)),
            PropAttr::Set(None) => self.set = Some(MaybeCustomFn::DefaultFn),
            PropAttr::DefaultVal(expr) => self.default = Some(expr),
            PropAttr::Name(lit) => self.name = Some(lit),
            PropAttr::Nick(lit) => self.nick = Some(lit),
            PropAttr::Blurb(lit) => self.blurb = Some(lit),
            PropAttr::Type(ty) => self.ty = Some(ty),
            PropAttr::Member(member) => self.member = Some(member),
            PropAttr::Flags(flags) => self.flags = flags,
        }
    }
}
struct PropDesc {
    field_ident: syn::Ident,
    field_ty: syn::Type,
    attrs: ReceivedAttrs,
}
impl PropDesc {
    fn new(field_ident: syn::Ident, field_ty: syn::Type, attrs: ReceivedAttrs) -> Self {
        let this = Self {
            field_ident,
            field_ty,
            attrs,
        };
        this.fill_unset_attrs()
    }
    fn fill_unset_attrs(mut self) -> Self {
        self.attrs.name = self.attrs.name.or_else(|| {
            Some(syn::LitStr::new(
                &self
                    .field_ident
                    .to_string()
                    .trim_matches('_')
                    .replace('_', "-"),
                self.field_ident.span(),
            ))
        });
        self.attrs.nick = self.attrs.nick.or_else(|| self.attrs.name.clone());
        self.attrs.blurb = self.attrs.blurb.or_else(|| self.attrs.name.clone());
        self.attrs.ty = Some(self.attrs.ty.unwrap_or_else(|| self.field_ty.clone()));
        self
    }
}

fn expand_properties_fn(props: &[PropDesc]) -> TokenStream2 {
    let n_props = props.len();
    let properties_build_phase = props.iter().map(|prop| {
        let ty = &prop.attrs.ty;
        let name = &prop.attrs.name;
        let nick = &prop.attrs.nick;
        let blurb = &prop.attrs.blurb;
        let default = prop
            .attrs
            .default
            .as_ref()
            .map_or(quote!(None), |x| quote!(Some(#x)));

        let flags = {
            let write = prop.attrs.set.as_ref().map(|_| quote!(WRITABLE));
            let read = prop.attrs.get.as_ref().map(|_| quote!(READABLE));

            let flags_iter = [write, read]
                .into_iter()
                .flatten()
                .chain(prop.attrs.flags.iter().map(|f| f.to_token_stream()));
            quote!(glib::ParamFlags::empty() #(| glib::ParamFlags::#flags_iter)*)
        };
        quote! {
            <#ty as glib::HasParamSpec>::Spec::new(#name, #nick, #blurb, #default, #flags)
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
        let name = &p.attrs.name;
        let ident = &p.field_ident;
        match (&p.attrs.member, &p.attrs.get) {
            (_, Some(MaybeCustomFn::CustomFn(expr))) => Some(quote!(#name => (#expr)(&self))),
            (None, Some(MaybeCustomFn::DefaultFn)) => Some(quote!(#name => self.#ident
                        .get(|v| v.to_value()))),
            (Some(member), Some(MaybeCustomFn::DefaultFn)) => Some(quote!(#name => self.#ident
                        .get(|v| v.#member.to_value()))),
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
        let name = &p.attrs.name;
        let ident = &p.field_ident;
        match (&p.attrs.member, &p.attrs.set) {
            (_, Some(MaybeCustomFn::CustomFn(expr))) => {
                Some(quote!(#name => (#expr)(&self, value)))
            }
            (None, Some(MaybeCustomFn::DefaultFn)) => Some(quote!(#name =>
                        self.#ident.set(|v| *v = value.get_owned()
                            .expect("Can't convert glib::value to property type")))),
            (Some(member), Some(MaybeCustomFn::DefaultFn)) => Some(quote!(#name => 
                        self.#ident.set(|v| v.#member = value.get_owned()
                            .expect("Can't convert glib::value to property type")))),
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

pub fn impl_derive_props(input: PropsMacroInput) -> TokenStream {
    let struct_ident = &input.ident;
    let fn_properties = expand_properties_fn(&input.props);
    let fn_property = expand_property_fn(&input.props);
    let fn_set_property = expand_set_property_fn(&input.props);
    let expanded = quote! {
        use glib::{ParamStoreRead, ParamStoreWrite};
        impl ObjectImpl for #struct_ident {
            #fn_properties
            #fn_property
            #fn_set_property
        }
    };
    proc_macro::TokenStream::from(expanded)
}
