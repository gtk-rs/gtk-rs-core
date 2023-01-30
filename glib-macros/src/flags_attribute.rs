// Take a look at the license at the top of the repository in the LICENSE file.

use heck::{ToKebabCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use proc_macro_error::abort_call_site;
use quote::{quote, quote_spanned};
use syn::{
    punctuated::Punctuated, spanned::Spanned, token::Comma, Attribute, Data, DeriveInput, Ident,
    NestedMeta, Variant, Visibility,
};

use crate::utils::{
    crate_ident_new, find_attribute_meta, find_nested_meta, parse_item_attributes,
    parse_name_attribute, ItemAttribute,
};

// Flag is not registered if it has the #[flags_value(skip)] meta
fn attribute_has_skip(attrs: &[Attribute]) -> bool {
    let meta = find_attribute_meta(attrs, "flags_value").unwrap();

    match meta {
        None => false,
        Some(meta) => find_nested_meta(&meta, "skip").is_some(),
    }
}

// Generate glib::gobject_ffi::GFlagsValue structs mapping the enum such as:
//     glib::gobject_ffi::GFlagsValue {
//         value: MyFlags::A.bits(),
//         value_name: "The Name\0" as *const _ as *const _,
//         value_nick: "nick\0" as *const _ as *const _,
//     },
fn gen_flags_values(
    enum_name: &Ident,
    enum_variants: &Punctuated<Variant, Comma>,
) -> (TokenStream, usize) {
    let crate_ident = crate_ident_new();

    // start at one as GFlagsValue array is null-terminated
    let mut n = 1;
    let recurse = enum_variants.iter().filter(|v| { !attribute_has_skip(&v.attrs) } ).map(|v| {
        let name = &v.ident;
        let mut value_name = name.to_string().to_upper_camel_case();
        let mut value_nick = name.to_string().to_kebab_case();

        let attrs = parse_item_attributes("flags_value", &v.attrs);
        let attrs = match attrs {
            Ok(attrs) => attrs,
            Err(e) => abort_call_site!(
                "{}: #[glib::flags] supports only the following optional attributes: #[flags_value(name = \"The Name\", nick = \"the-nick\")] or #[flags_value(skip)]",
                e
            ),
        };

        attrs.into_iter().for_each(|attr|
            match attr {
                ItemAttribute::Name(n) => value_name = n,
                ItemAttribute::Nick(n) => value_nick = n,
            }
        );

        let value_name = format!("{value_name}\0");
        let value_nick = format!("{value_nick}\0");

        n += 1;
        quote_spanned! {v.span()=>
            #crate_ident::gobject_ffi::GFlagsValue {
                value: #enum_name::#name.bits(),
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

fn gen_bitflags(
    enum_name: &Ident,
    visibility: &Visibility,
    enum_variants: &Punctuated<Variant, Comma>,
    crate_ident: &TokenStream,
) -> TokenStream {
    let recurse = enum_variants.iter().map(|v| {
        let name = &v.ident;
        let disc = v.discriminant.as_ref().expect("missing discriminant");
        let value = &disc.1;

        quote_spanned! {v.span()=>
            const #name = #value;
        }
    });

    quote! {
        #crate_ident::bitflags::bitflags! {
            #visibility struct #enum_name: u32 {
                #(#recurse)*
            }
        }
    }
}

pub fn impl_flags(attrs: &NestedMeta, input: &DeriveInput) -> TokenStream {
    let gtype_name = match parse_name_attribute(attrs) {
        Ok(name) => name,
        Err(e) => abort_call_site!(
            "{}: [glib::flags] requires #[glib::flags(name = \"FlagsTypeName\")]",
            e
        ),
    };

    let name = &input.ident;
    let visibility = &input.vis;

    let enum_variants = match input.data {
        Data::Enum(ref e) => &e.variants,
        _ => abort_call_site!("#[glib::flags] only supports enums"),
    };

    let crate_ident = crate_ident_new();

    let bitflags = gen_bitflags(name, visibility, enum_variants, &crate_ident);
    let (flags_values, nb_flags_values) = gen_flags_values(name, enum_variants);

    quote! {
        #bitflags

        impl #crate_ident::translate::IntoGlib for #name {
            type GlibType = u32;

            #[inline]
            fn into_glib(self) -> u32 {
                self.bits()
            }
        }

        impl #crate_ident::translate::FromGlib<u32> for #name {
            #[inline]
            unsafe fn from_glib(value: u32) -> Self {
                Self::from_bits_truncate(value)
            }
        }

        impl #crate_ident::value::ValueType for #name {
            type Type = Self;
        }

        unsafe impl<'a> #crate_ident::value::FromValue<'a> for #name {
            type Checker = #crate_ident::value::GenericValueTypeChecker<Self>;

            #[inline]
            unsafe fn from_value(value: &'a #crate_ident::value::Value) -> Self {
                #crate_ident::translate::from_glib(#crate_ident::gobject_ffi::g_value_get_flags(
                    #crate_ident::translate::ToGlibPtr::to_glib_none(value).0
                ))
            }
        }

        impl #crate_ident::value::ToValue for #name {
            #[inline]
            fn to_value(&self) -> #crate_ident::value::Value {
                let mut value = #crate_ident::value::Value::for_value_type::<Self>();
                unsafe {
                    #crate_ident::gobject_ffi::g_value_set_flags(
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

        impl #crate_ident::HasParamSpec for #name {
            type ParamSpec = #crate_ident::ParamSpecFlags;
            type SetValue = Self;
            type BuilderFn = fn(&str) -> #crate_ident::ParamSpecFlagsBuilder<Self>;

            fn param_spec_builder() -> Self::BuilderFn {
                |name| Self::ParamSpec::builder(name)
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
                    static mut VALUES: [#crate_ident::gobject_ffi::GFlagsValue; #nb_flags_values] = [
                        #flags_values
                        #crate_ident::gobject_ffi::GFlagsValue {
                            value: 0,
                            value_name: ::std::ptr::null(),
                            value_nick: ::std::ptr::null(),
                        },
                    ];

                    let name = ::std::ffi::CString::new(#gtype_name).expect("CString::new failed");
                    unsafe {
                        let type_ = #crate_ident::gobject_ffi::g_flags_register_static(name.as_ptr(), VALUES.as_ptr());
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
    }
}
