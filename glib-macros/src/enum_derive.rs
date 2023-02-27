// Take a look at the license at the top of the repository in the LICENSE file.

use heck::{ToKebabCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use proc_macro_error::abort_call_site;
use quote::{quote, quote_spanned};
use syn::{punctuated::Punctuated, spanned::Spanned, token::Comma, Data, Ident, Variant};

use crate::utils::{crate_ident_new, gen_enum_from_glib};

#[derive(deluxe::ExtractAttributes, Default)]
#[deluxe(attributes(enum_type))]
struct EnumType {
    name: String,
}

#[derive(deluxe::ExtractAttributes, Default)]
#[deluxe(attributes(enum_value), default)]
struct EnumValue {
    name: Option<String>,
    nick: Option<String>,
}

// Generate glib::gobject_ffi::GEnumValue structs mapping the enum such as:
//     glib::gobject_ffi::GEnumValue {
//         value: Animal::Goat as i32,
//         value_name: "Goat\0" as *const _ as *const _,
//         value_nick: "goat\0" as *const _ as *const _,
//     },
fn gen_enum_values(
    enum_name: &Ident,
    enum_variants: &mut Punctuated<Variant, Comma>,
    errors: &deluxe::Errors,
) -> (TokenStream, usize) {
    let crate_ident = crate_ident_new();

    // start at one as GEnumValue array is null-terminated
    let mut n = 1;
    let recurse = enum_variants.iter_mut().map(|v| {
        let EnumValue {
            name: value_name,
            nick: value_nick,
        } = deluxe::extract_attributes_optional(v, errors);

        let name = &v.ident;
        let mut value_name = value_name.unwrap_or_else(|| name.to_string().to_upper_camel_case());
        let mut value_nick = value_nick.unwrap_or_else(|| name.to_string().to_kebab_case());
        value_name.push('\0');
        value_nick.push('\0');

        n += 1;
        quote_spanned! {v.span()=>
            #crate_ident::gobject_ffi::GEnumValue {
                value: #enum_name::#name as i32,
                value_name: #value_name as *const _ as *const _,
                value_nick: #value_nick as *const _ as *const _,
            },
        }
    });
    (
        quote! {
            #(#recurse)*
        },
        n,
    )
}

pub fn impl_enum(mut input: syn::DeriveInput) -> TokenStream {
    let enum_variants = match &mut input.data {
        Data::Enum(e) => &mut e.variants,
        _ => abort_call_site!("#[derive(glib::Enum)] only supports enums"),
    };

    let errors = deluxe::Errors::new();
    let EnumType {
        name: mut gtype_name,
    } = deluxe::extract_attributes_optional(&mut input.attrs, &errors);
    gtype_name.push('\0');

    let name = &input.ident;
    let from_glib = gen_enum_from_glib(name, enum_variants);
    let (enum_values, nb_enum_values) = gen_enum_values(name, enum_variants, &errors);

    let crate_ident = crate_ident_new();

    quote! {
        #errors

        impl #crate_ident::translate::IntoGlib for #name {
            type GlibType = i32;

            #[inline]
            fn into_glib(self) -> i32 {
                self as i32
            }
        }

        impl #crate_ident::translate::TryFromGlib<i32> for #name {
            type Error = i32;

            #[inline]
            unsafe fn try_from_glib(value: i32) -> ::core::result::Result<Self, i32> {
                let from_glib = || {
                    #from_glib
                };

                from_glib().ok_or(value)
            }
        }

        impl #crate_ident::translate::FromGlib<i32> for #name {
            #[inline]
            unsafe fn from_glib(value: i32) -> Self {
                use #crate_ident::translate::TryFromGlib;

                Self::try_from_glib(value).unwrap()
            }
        }

        impl #crate_ident::value::ValueType for #name {
            type Type = Self;
        }

        unsafe impl<'a> #crate_ident::value::FromValue<'a> for #name {
            type Checker = #crate_ident::value::GenericValueTypeChecker<Self>;

            #[inline]
            unsafe fn from_value(value: &'a #crate_ident::value::Value) -> Self {
                #crate_ident::translate::from_glib(#crate_ident::gobject_ffi::g_value_get_enum(
                    #crate_ident::translate::ToGlibPtr::to_glib_none(value).0
                ))
            }
        }

        impl #crate_ident::value::ToValue for #name {
            #[inline]
            fn to_value(&self) -> #crate_ident::value::Value {
                let mut value = #crate_ident::value::Value::for_value_type::<Self>();
                unsafe {
                    #crate_ident::gobject_ffi::g_value_set_enum(
                        #crate_ident::translate::ToGlibPtrMut::to_glib_none_mut(&mut value).0,
                        #crate_ident::translate::IntoGlib::into_glib(*self)
                    )
                }
                value
            }

            #[inline]
            fn value_type(&self) -> #crate_ident::Type {
                <Self as #crate_ident::StaticType>::static_type()
            }
        }

        impl ::std::convert::From<#name> for #crate_ident::Value {
            #[inline]
            fn from(v: #name) -> Self {
                #crate_ident::value::ToValue::to_value(&v)
            }
        }

        impl #crate_ident::StaticType for #name {
            #[inline]
            fn static_type() -> #crate_ident::Type {
                static ONCE: ::std::sync::Once = ::std::sync::Once::new();
                static mut TYPE: #crate_ident::Type = #crate_ident::Type::INVALID;

                ONCE.call_once(|| {
                    static mut VALUES: [#crate_ident::gobject_ffi::GEnumValue; #nb_enum_values] = [
                        #enum_values
                        #crate_ident::gobject_ffi::GEnumValue {
                            value: 0,
                            value_name: ::std::ptr::null(),
                            value_nick: ::std::ptr::null(),
                        },
                    ];

                    unsafe {
                        let type_ = #crate_ident::gobject_ffi::g_enum_register_static(
                            #gtype_name.as_ptr() as *const _,
                            VALUES.as_ptr(),
                        );
                        let type_: #crate_ident::Type = #crate_ident::translate::from_glib(type_);
                        assert!(type_.is_valid());
                        TYPE = type_;
                    }
                });

                unsafe {
                    TYPE
                }
            }
        }

        impl #crate_ident::HasParamSpec for #name {
            type ParamSpec = #crate_ident::ParamSpecEnum;
            type SetValue = Self;
            type BuilderFn = fn(&str, Self) -> #crate_ident::ParamSpecEnumBuilder<Self>;

            fn param_spec_builder() -> Self::BuilderFn {
                |name, default_value| Self::ParamSpec::builder_with_default(name, default_value)
            }
        }
    }
}
