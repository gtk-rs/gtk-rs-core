use proc_macro::TokenStream;
use quote::quote;
use syn::parse::Parse;
use syn::spanned::Spanned;
use syn::Token;
use syn::{parse_macro_input, DeriveInput};
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

    // ident = "literal"
    Name(syn::LitStr),
    Nick(syn::LitStr),
    Blurb(syn::LitStr),
}

impl Parse for PropAttr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name: syn::Ident = input.parse()?;
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
                // name = expr
                let expr: syn::Expr = input.parse()?;
                match &*name_str {
                    "default" => PropAttr::DefaultVal(expr),
                    "get" => PropAttr::Get(Some(expr)),
                    "set" => PropAttr::Set(Some(expr)),
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
struct PropDesc {
    field: Option<syn::Field>,
    get: Option<MaybeCustomFn>,
    set: Option<MaybeCustomFn>,
    construct: bool,
    construct_only: bool,
    // These are not syn::LitStr because `name` may be set by default with `field.ident`
    name: Option<syn::LitStr>,
    nick: Option<syn::LitStr>,
    blurb: Option<syn::LitStr>,
    default: Option<syn::Expr>,
}
impl PropDesc {
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
        }
    }
    fn clean(mut self) -> Self {
        self.name = self.name.or_else(|| {
            self.field.as_ref().map(|x| {
                syn::LitStr::new(
                    &x.ident
                        .as_ref()
                        .unwrap()
                        .to_string()
                        .trim_matches('_')
                        .replace("_", "-"),
                    x.ident.span(),
                )
            })
        });
        self.nick = self.nick.or_else(|| self.name.clone());
        self.blurb = self.blurb.or_else(|| self.name.clone());
        self
    }
}
impl FromIterator<PropAttr> for PropDesc {
    fn from_iter<T: IntoIterator<Item = PropAttr>>(iter: T) -> Self {
        let mut this = PropDesc::default();
        for attr in iter {
            this.set_from_attr(attr);
        }
        this
    }
}

pub fn impl_derive_props(input: TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let props: Vec<PropDesc> = fields_prop(input.data).collect();

    let match_branch_get = props.iter().flat_map(|p| {
        let name = &p.name;
        let ident = &p.field.as_ref().unwrap().ident;
        match &p.get {
            Some(MaybeCustomFn::CustomFn(expr)) => Some(quote!(#name => (#expr)(&self))),
            Some(MaybeCustomFn::DefaultFn) => Some(quote!(#name => self.#ident.get())),
            None => None,
        }
    });
    let match_branch_set = props.iter().flat_map(|p| {
        let name = &p.name;
        let ident = &p.field.as_ref().unwrap().ident;
        match &p.set {
            Some(MaybeCustomFn::CustomFn(expr)) => Some(quote!(#name => (#expr)(&self, value))),
            Some(MaybeCustomFn::DefaultFn) => Some(quote!(#name => self.#ident.set(value))),
            None => None,
        }
    });

    let n_props = props.len();
    let properties_build_phase = props.iter().map(|prop| {
        let ty = &prop.field.as_ref().unwrap().ty;
        let name = &prop.name;
        let nick = &prop.nick;
        let blurb = &prop.blurb;
        let get = prop.get.is_some();
        let set = prop.get.is_some();
        let default = prop
            .default
            .as_ref()
            .map_or(quote!(None), |x| quote!(Some(#x)));
        quote! {
            {
                let mut flags = glib::ParamFlags::empty();
                if #get {
                    flags |= glib::ParamFlags::READABLE;
                }
                if #set {
                    flags |= glib::ParamFlags::WRITABLE;
                }
                <#ty as glib::HasParamSpec>::Spec::new(#name, #nick, #blurb, #default, flags)
            }
        }
    });
    let expanded = quote! {
        use glib::{ParamStoreRead, ParamStoreWrite};
        impl ObjectImpl for #name {
            fn properties() -> &'static [glib::ParamSpec] {
                use glib::once_cell::sync::Lazy;
                static PROPERTIES: Lazy<[glib::ParamSpec; #n_props]> = Lazy::new(|| [
                    #(#properties_build_phase,)*
                ]);
                PROPERTIES.as_ref()
            }
            fn property(&self, _obj: &Self::Type, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
                match pspec.name() {
                    #(#match_branch_get,)*
                    p => unreachable!("Invalid property {}", p)
                }
            }
            fn set_property(&self, _obj: &Self::Type, _id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
                match pspec.name() {
                    #(#match_branch_set,)*
                    p => unreachable!("Invalid property {}", p)
                }
            }
        }
    };
    proc_macro::TokenStream::from(expanded)
}

fn fields_prop(data: syn::Data) -> impl Iterator<Item = PropDesc> {
    match data {
        syn::Data::Struct(syn::DataStruct { fields, .. }) => {
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

                if let None = prop_attrs.peek() {
                    return None;
                }
                let mut prop_desc = PropDesc::from_iter(prop_attrs);
                prop_desc.field = Some(field);

                Some(prop_desc.clean())
            })
        }
        _ => {
            panic!("Can't derive Props on non struct")
        }
    }
}
