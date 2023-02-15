// Take a look at the license at the top of the repository in the LICENSE file.

use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::DeriveHeader;

#[derive(Default, deluxe::ExtractAttributes)]
#[deluxe(attributes(object_impl))]
struct ObjectImpl {
    derived_properties: deluxe::Flag,
    #[deluxe(map(|e| FlagOrExpr::into_expr(e, "signals")))]
    signals: Option<syn::Expr>,
    #[deluxe(map(|e| FlagOrExpr::into_expr(e, "constructed")))]
    constructed: Option<syn::Expr>,
    #[deluxe(map(|e| FlagOrExpr::into_expr(e, "dispose")))]
    dispose: Option<syn::Expr>,
}

enum FlagOrExpr {
    Flag,
    Expr(syn::Expr),
}

impl deluxe::ParseMetaItem for FlagOrExpr {
    #[inline]
    fn parse_meta_item(
        input: syn::parse::ParseStream,
        _mode: deluxe::ParseMode,
    ) -> deluxe::Result<Self> {
        Ok(Self::Expr(input.parse()?))
    }
    #[inline]
    fn parse_meta_item_flag(_span: proc_macro2::Span) -> deluxe::Result<Self> {
        Ok(Self::Flag)
    }
}

impl FlagOrExpr {
    #[inline]
    fn into_expr(e: Option<Self>, default_name: &str) -> Option<syn::Expr> {
        e.map(|e| match e {
            Self::Flag => {
                let func = syn::Ident::new(default_name, proc_macro2::Span::call_site());
                syn::parse_quote! { Self::#func }
            }
            Self::Expr(expr) => expr,
        })
    }
}

pub fn impl_object_impl(mut input: DeriveHeader) -> TokenStream {
    let errors = deluxe::Errors::new();
    let ObjectImpl {
        derived_properties,
        signals,
        constructed,
        dispose,
    } = deluxe::extract_attributes_optional(&mut input, &errors);

    let glib = crate::utils::crate_ident_new();
    let ident = &input.ident;
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();

    let properties = derived_properties.is_set().then(|| {
        quote! {
            fn properties() -> &'static [#glib::ParamSpec] {
                Self::derived_properties()
            }
            fn property(&self, id: ::std::primitive::usize, pspec: &#glib::ParamSpec) -> #glib::Value {
                Self::derived_property(self, id, pspec)
            }
            fn set_property(&self, id: ::std::primitive::usize, value: &#glib::Value, pspec: &#glib::ParamSpec) {
                Self::derived_set_property(self, id, value, pspec)
            }
        }
    });
    let signals = signals.map(|signals| {
        quote! {
            fn signals() -> &'static [#glib::subclass::Signal] {
                (#signals)()
            }
        }
    });
    let constructed = constructed.map(|constructed| {
        quote! {
            fn constructed(&self) {
                (#constructed)(self)
            }
        }
    });
    let dispose = dispose.map(|dispose| {
        quote! {
            fn dispose(&self) {
                (#dispose)(self)
            }
        }
    });
    quote! {
        #errors
        impl #impl_generics #glib::subclass::object::ObjectImpl for #ident #type_generics #where_clause {
            #properties
            #signals
            #constructed
            #dispose
        }
    }
}
