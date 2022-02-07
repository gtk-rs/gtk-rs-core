use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use proc_macro_error::emit_warning;
use quote::quote;
use quote::ToTokens;
use syn::ext::IdentExt;
use syn::parse::Parse;
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
            _ => Err(syn::Error::new(
                derive_input.span(),
                "props can only be derived on structs",
            ))?,
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
        dbg!(&name_str);

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
    construct: bool,
    construct_only: bool,
    // These are not syn::LitStr because `name` may be set by default with `field.ident`
    name: Option<syn::LitStr>,
    nick: Option<syn::LitStr>,
    blurb: Option<syn::LitStr>,
    default: Option<syn::Expr>,
}
impl ReceivedAttrs {
    fn new(attrs: impl Iterator<Item = PropAttr>) -> Self {
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
        }
    }
}
struct PropDesc {
    field: syn::Field,
    attrs: ReceivedAttrs,
}
impl PropDesc {
    fn new(attrs: ReceivedAttrs, field: syn::Field) -> Self {
        let this = Self { field, attrs };
        this.fill_unset_attrs()
    }
    fn fill_unset_attrs(mut self) -> Self {
        self.attrs.name = self.attrs.name.or_else(|| {
            Some(syn::LitStr::new(
                &self
                    .field
                    .ident
                    .as_ref()
                    .unwrap()
                    .to_string()
                    .trim_matches('_')
                    .replace("_", "-"),
                self.field.ident.span(),
            ))
        });
        self.attrs.nick = self.attrs.nick.or_else(|| self.attrs.name.clone());
        self.attrs.blurb = self.attrs.blurb.or_else(|| self.attrs.name.clone());
        self.attrs.ty = Some(self.attrs.ty.unwrap_or(self.field.ty.clone()));
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
        let flags = match (prop.attrs.get.is_some(), prop.attrs.set.is_some()) {
            (false, false) => quote!(glib::ParamFlags::empty()),
            (false, true) => quote!(glib::ParamFlags::WRITABLE),
            (true, false) => quote!(glib::ParamFlags::READABLE),
            (true, true) => quote!(glib::ParamFlags::READWRITE),
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
        let ident = &p.field.ident;
        match (&p.attrs.member, &p.attrs.get) {
            (None, Some(MaybeCustomFn::CustomFn(expr))) => Some(quote!(#name => (#expr)(&self))),
            (None, Some(MaybeCustomFn::DefaultFn)) => {
                Some(quote!(#name => self.#ident.get().to_value()))
            }
            (Some(member), Some(MaybeCustomFn::DefaultFn)) => {
                Some(quote!(#name => self.#ident.get().#member.to_value()))
            }
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
        let ident = &p.field.ident;
        match &p.attrs.set {
            Some(MaybeCustomFn::CustomFn(expr)) => Some(quote!(#name => (#expr)(&self, value))),
            Some(MaybeCustomFn::DefaultFn) => {
                Some(quote!(#name => self.#ident.set(value.get_owned().expect("Can't convert glib::value to property type"))))
            }
            None => None,
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
    fields.into_iter().filter_map(|field| {
        let mut prop_attrs = field
            .attrs
            .iter()
            .filter(|a| a.path.is_ident("prop"))
            .flat_map(|attrs| {
                attrs.parse_args_with(
                    syn::punctuated::Punctuated::<PropAttr, Token![,]>::parse_terminated,
                )
            })
            .flatten()
            .peekable();

        if prop_attrs.peek().is_none() {
            return None;
        }
        Some(PropDesc::new(ReceivedAttrs::new(prop_attrs), field.clone()))
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
    println!("{}", expanded.to_token_stream().to_string());
    proc_macro::TokenStream::from(expanded)
}
