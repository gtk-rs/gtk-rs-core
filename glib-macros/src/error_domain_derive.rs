// Take a look at the license at the top of the repository in the LICENSE file.

use proc_macro2::TokenStream;
use proc_macro_error::abort_call_site;
use quote::quote;
use syn::Data;

use crate::utils::{crate_ident_new, gen_enum_from_glib};

#[derive(deluxe::ExtractAttributes, Default)]
#[deluxe(attributes(error_domain))]
struct ErrorDomainType {
    name: String,
}

pub fn impl_error_domain(mut input: syn::DeriveInput) -> TokenStream {
    let name = &input.ident;

    let enum_variants = match &mut input.data {
        Data::Enum(e) => &mut e.variants,
        _ => abort_call_site!("#[derive(glib::ErrorDomain)] only supports enums"),
    };

    let errors = deluxe::Errors::new();
    let ErrorDomainType {
        name: mut domain_name,
    } = deluxe::extract_attributes_optional(&mut input.attrs, &errors);
    domain_name.push('\0');

    let crate_ident = crate_ident_new();

    let from_glib = gen_enum_from_glib(name, enum_variants);

    quote! {
        #errors

        impl #crate_ident::error::ErrorDomain for #name {
            #[inline]
            fn domain() -> #crate_ident::Quark {
                use #crate_ident::translate::from_glib;

                static QUARK: #crate_ident::once_cell::sync::Lazy<#crate_ident::Quark> =
                    #crate_ident::once_cell::sync::Lazy::new(|| unsafe {
                        from_glib(#crate_ident::ffi::g_quark_from_static_string(#domain_name.as_ptr() as *const _))
                    });
                *QUARK
            }

            #[inline]
            fn code(self) -> i32 {
                self as i32
            }

            #[inline]
            fn from(value: i32) -> ::core::option::Option<Self>
            where
                Self: Sized
            {
                #from_glib
            }
        }
    }
}
