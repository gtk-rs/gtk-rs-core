// Take a look at the license at the top of the repository in the LICENSE file.

use heck::ToKebabCase;
use proc_macro2::TokenStream;
use proc_macro_error::abort;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Fields, FieldsNamed, FieldsUnnamed, Generics, Ident, Type};

use crate::utils::crate_ident_new;

pub fn impl_variant(mut input: DeriveInput) -> TokenStream {
    match input.data {
        Data::Struct(data_struct) => {
            derive_variant_for_struct(input.ident, input.generics, data_struct)
        }
        Data::Enum(data_enum) => {
            let errors = deluxe::Errors::new();
            let mode = get_enum_mode(&mut input.attrs, &errors);
            let has_data = data_enum
                .variants
                .iter()
                .any(|v| !matches!(v.fields, syn::Fields::Unit));
            let tokens = if has_data {
                derive_variant_for_enum(input.ident, input.generics, data_enum, mode)
            } else {
                derive_variant_for_c_enum(input.ident, input.generics, data_enum, mode)
            };
            quote! { #errors #tokens }
        }
        Data::Union(_) => {
            abort!(
                input,
                "#[derive(glib::Variant)] is not available for unions."
            );
        }
    }
}

fn derive_variant_for_struct(
    ident: Ident,
    generics: Generics,
    data_struct: syn::DataStruct,
) -> TokenStream {
    let glib = crate_ident_new();
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();
    let (static_variant_type, to_variant, from_variant) = match data_struct.fields {
        Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
            let types = unnamed
                .into_pairs()
                .map(|pair| pair.into_value())
                .map(|field| field.ty)
                .collect::<Vec<_>>();

            let idents = (0..types.len()).map(syn::Index::from).collect::<Vec<_>>();
            let idents_len = idents.len();

            let static_variant_type = quote! {
                impl #impl_generics #glib::StaticVariantType for #ident #type_generics #where_clause {
                    #[inline]
                    fn static_variant_type() -> ::std::borrow::Cow<'static, #glib::VariantTy> {
                        static TYP: #glib::once_cell::sync::Lazy<#glib::VariantType> = #glib::once_cell::sync::Lazy::new(|| {

                            let mut builder = #glib::GStringBuilder::new("(");

                            #(
                                {
                                    let typ = <#types as #glib::StaticVariantType>::static_variant_type();
                                    builder.append(typ.as_str());
                                }
                            )*
                            builder.append_c(')');

                            #glib::VariantType::from_string(builder.into_string()).unwrap()
                        });

                        ::std::borrow::Cow::Borrowed(&*TYP)
                    }
                }
            };

            let to_variant = quote! {
                impl #impl_generics #glib::ToVariant for #ident #type_generics #where_clause {
                    fn to_variant(&self) -> #glib::Variant {
                        #glib::Variant::tuple_from_iter(::std::array::IntoIter::<#glib::Variant, #idents_len>::new([
                            #(
                                #glib::ToVariant::to_variant(&self.#idents)
                            ),*
                        ]))
                    }
                }

                impl #impl_generics ::std::convert::From<#ident #type_generics> for #glib::Variant #where_clause {
                    fn from(v: #ident #type_generics) -> #glib::Variant {
                        #glib::Variant::tuple_from_iter(::std::array::IntoIter::<#glib::Variant, #idents_len>::new([
                            #(
                                <#glib::Variant as ::std::convert::From<_>>::from(v.#idents)
                            ),*
                        ]))
                    }
                }
            };

            let from_variant = quote! {
                impl #impl_generics #glib::FromVariant for #ident #type_generics #where_clause {
                    fn from_variant(variant: &#glib::Variant) -> ::core::option::Option<Self> {
                        if !variant.is_container() {
                            return None;
                        }
                        Some(Self(
                            #(
                                match variant.try_child_get::<#types>(#idents) {
                                    Ok(Some(field)) => field,
                                    _ => return None,
                                }
                            ),*
                        ))
                    }
                }
            };

            (static_variant_type, to_variant, from_variant)
        }
        Fields::Named(FieldsNamed { named, .. }) => {
            let fields: Vec<(Ident, Type)> = named
                .into_pairs()
                .map(|pair| pair.into_value())
                .map(|field| (field.ident.expect("Field ident is specified"), field.ty))
                .collect();

            let idents: Vec<_> = fields.iter().map(|(ident, _ty)| ident).collect();
            let types: Vec<_> = fields.iter().map(|(_ident, ty)| ty).collect();
            let counts = (0..types.len()).map(syn::Index::from).collect::<Vec<_>>();

            let static_variant_type = quote! {
                impl #impl_generics #glib::StaticVariantType for #ident #type_generics #where_clause {
                    #[inline]
                    fn static_variant_type() -> ::std::borrow::Cow<'static, #glib::VariantTy> {
                        static TYP: #glib::once_cell::sync::Lazy<#glib::VariantType> = #glib::once_cell::sync::Lazy::new(|| unsafe {
                            let ptr = #glib::ffi::g_string_sized_new(16);
                            #glib::ffi::g_string_append_c(ptr, b'(' as _);

                            #(
                                {
                                    let typ = <#types as #glib::StaticVariantType>::static_variant_type();
                                    #glib::ffi::g_string_append_len(
                                        ptr,
                                        typ.as_str().as_ptr() as *const _,
                                        typ.as_str().len() as isize,
                                    );
                                }
                            )*
                            #glib::ffi::g_string_append_c(ptr, b')' as _);

                            #glib::translate::from_glib_full(
                                #glib::ffi::g_string_free(ptr, #glib::ffi::GFALSE) as *mut #glib::ffi::GVariantType
                            )
                        });

                        ::std::borrow::Cow::Borrowed(&*TYP)
                    }
                }
            };

            let to_variant = quote! {
                impl #impl_generics #glib::ToVariant for #ident #type_generics #where_clause {
                    fn to_variant(&self) -> #glib::Variant {
                        #glib::Variant::tuple_from_iter(::std::iter::IntoIterator::into_iter([
                            #(
                                #glib::ToVariant::to_variant(&self.#idents)
                            ),*
                        ]))
                    }
                }

                impl #impl_generics ::std::convert::From<#ident #type_generics> for #glib::Variant #where_clause {
                    fn from(v: #ident #type_generics) -> #glib::Variant {
                        #glib::Variant::tuple_from_iter(::std::iter::IntoIterator::into_iter([
                            #(
                                <#glib::Variant as ::std::convert::From<_>>::from(v.#idents)
                            ),*
                        ]))
                    }
                }
            };

            let from_variant = quote! {
                impl #impl_generics #glib::FromVariant for #ident #type_generics #where_clause {
                    fn from_variant(variant: &#glib::Variant) -> ::core::option::Option<Self> {
                        if !variant.is_container() {
                            return None;
                        }
                        Some(Self {
                            #(
                                #idents: match variant.try_child_get::<#types>(#counts) {
                                    Ok(Some(field)) => field,
                                    _ => return None,
                                }
                            ),*
                        })
                    }
                }
            };

            (static_variant_type, to_variant, from_variant)
        }
        Fields::Unit => {
            let static_variant_type = quote! {
                impl #impl_generics #glib::StaticVariantType for #ident #type_generics #where_clause {
                    #[inline]
                    fn static_variant_type() -> ::std::borrow::Cow<'static, #glib::VariantTy> {
                        ::std::borrow::Cow::Borrowed(#glib::VariantTy::UNIT)
                    }
                }
            };

            let to_variant = quote! {
                impl #impl_generics #glib::ToVariant for #ident #type_generics #where_clause {
                    #[inline]
                    fn to_variant(&self) -> #glib::Variant {
                        #glib::ToVariant::to_variant(&())
                    }
                }

                impl #impl_generics ::std::convert::From<#ident #type_generics> for #glib::Variant #where_clause {
                    #[inline]
                    fn from(v: #ident #type_generics) -> #glib::Variant {
                        #glib::ToVariant::to_variant(&())
                    }
                }
            };

            let from_variant = quote! {
                impl #impl_generics #glib::FromVariant for #ident #type_generics #where_clause {
                    fn from_variant(variant: &#glib::Variant) -> ::core::option::Option<Self> {
                        Some(Self)
                    }
                }
            };

            (static_variant_type, to_variant, from_variant)
        }
    };

    quote! {
        #static_variant_type

        #to_variant

        #from_variant
    }
}

#[derive(deluxe::ParseMetaItem, Default)]
#[deluxe(default)]
enum EnumMode {
    #[default]
    #[deluxe(skip)]
    String,
    #[deluxe(skip)]
    Repr(Ident),
    Enum {
        #[deluxe(skip)]
        repr: bool,
    },
    Flags {
        #[deluxe(skip)]
        repr: bool,
    },
}

impl EnumMode {
    fn tag_type(&self) -> char {
        match self {
            EnumMode::String => 's',
            EnumMode::Repr(repr) => match repr.to_string().as_str() {
                "i8" | "i16" => 'n',
                "i32" => 'i',
                "i64" => 'x',
                "u8" => 'y',
                "u16" => 'q',
                "u32" => 'u',
                "u64" => 't',
                _ => unimplemented!(),
            },
            EnumMode::Enum { repr } => {
                if *repr {
                    'i'
                } else {
                    's'
                }
            }
            EnumMode::Flags { repr } => {
                if *repr {
                    'u'
                } else {
                    's'
                }
            }
        }
    }
}

fn derive_variant_for_enum(
    ident: Ident,
    generics: Generics,
    data_enum: syn::DataEnum,
    mode: EnumMode,
) -> TokenStream {
    let glib = crate_ident_new();
    let static_variant_type = format!("({}v)", mode.tag_type());
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    let to = data_enum.variants.iter().enumerate().map(|(index, v)| {
        let ident = &v.ident;
        let tag = match &mode {
            EnumMode::String => {
                let nick = ToKebabCase::to_kebab_case(ident.to_string().as_str());
                quote! { #nick }
            },
            EnumMode::Repr(repr) => quote! { #index as #repr },
            _ => unimplemented!(),
        };
        if !matches!(v.fields, syn::Fields::Unit) {
            match &mode {
                EnumMode::Enum { .. } =>
                    abort!(v, "#[variant_enum(enum) only allowed with C-style enums using #[derive(glib::Enum)]"),
                EnumMode::Flags { .. } =>
                    abort!(v, "#[variant_enum(flags) only allowed with bitflags using #[glib::flags]"),
                _ => (),
            }
        }
        match &v.fields {
            syn::Fields::Named(FieldsNamed { named, .. }) => {
                let field_names = named.iter().map(|f| f.ident.as_ref().unwrap());
                let field_names2 = field_names.clone();
                quote! {
                    Self::#ident { #(#field_names),* } => #glib::ToVariant::to_variant(&(
                        #tag,
                        #glib::Variant::tuple_from_iter(::std::iter::IntoIterator::into_iter([
                            #(#glib::ToVariant::to_variant(&#field_names2)),*
                        ]))
                    ))
                }
            },
            syn::Fields::Unnamed(FieldsUnnamed  { unnamed, .. }) => {
                let field_names = unnamed.iter().enumerate().map(|(i, _)| {
                    format_ident!("field{}", i)
                });
                let field_names2 = field_names.clone();
                quote! {
                    Self::#ident(#(#field_names),*) => #glib::ToVariant::to_variant(&(
                        #tag,
                        #glib::Variant::tuple_from_iter(::std::iter::IntoIterator::into_iter([
                            #(#glib::ToVariant::to_variant(&#field_names2)),*
                        ]))
                    ))
                }
            },
            syn::Fields::Unit => {
                quote! {
                    Self::#ident => #glib::ToVariant::to_variant(&(
                        #tag,
                        #glib::ToVariant::to_variant(&())
                    ))
                }
            },
        }
    });
    let into = data_enum.variants.iter().enumerate().map(|(index, v)| {
        let field_ident = &v.ident;
        let tag = match &mode {
            EnumMode::String => {
                let nick = ToKebabCase::to_kebab_case(field_ident.to_string().as_str());
                quote! { #nick }
            }
            EnumMode::Repr(repr) => quote! { #index as #repr },
            _ => unimplemented!(),
        };
        match &v.fields {
            syn::Fields::Named(FieldsNamed { named, .. }) => {
                let field_names = named.iter().map(|f| f.ident.as_ref().unwrap());
                let field_names2 = field_names.clone();
                quote! {
                    #ident::#field_ident { #(#field_names),* } => #glib::ToVariant::to_variant(&(
                        #tag,
                        #glib::Variant::tuple_from_iter(::std::iter::IntoIterator::into_iter([
                            #(<#glib::Variant as ::std::convert::From<_>>::from(#field_names2)),*
                        ]))
                    ))
                }
            }
            syn::Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                let field_names = unnamed
                    .iter()
                    .enumerate()
                    .map(|(i, _)| format_ident!("field{}", i));
                let field_names2 = field_names.clone();
                quote! {
                    #ident::#field_ident(#(#field_names),*) => #glib::ToVariant::to_variant(&(
                        #tag,
                        #glib::Variant::tuple_from_iter(::std::iter::IntoIterator::into_iter([
                            #(<#glib::Variant as ::std::convert::From<_>>::from(#field_names2)),*
                        ]))
                    ))
                }
            }
            syn::Fields::Unit => {
                quote! {
                    #ident::#field_ident => #glib::ToVariant::to_variant(&(
                        #tag,
                        #glib::ToVariant::to_variant(&())
                    ))
                }
            }
        }
    });
    let from = data_enum.variants.iter().enumerate().map(|(index, v)| {
        let ident = &v.ident;
        let tag = match &mode {
            EnumMode::String => {
                let nick = ToKebabCase::to_kebab_case(ident.to_string().as_str());
                quote! { #nick }
            }
            EnumMode::Repr(_) => quote! { #index },
            _ => unimplemented!(),
        };
        match &v.fields {
            syn::Fields::Named(FieldsNamed { named, .. }) => {
                let fields = named.iter().enumerate().map(|(index, f)| {
                    let name = f.ident.as_ref().unwrap();
                    let repr = &f.ty;
                    quote! {
                        #name: <#repr as #glib::FromVariant>::from_variant(
                            &#glib::Variant::try_child_value(&value, #index)?
                        )?
                    }
                });
                quote! { #tag => ::std::option::Option::Some(Self::#ident { #(#fields),* }), }
            }
            syn::Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                let indices = 0..unnamed.iter().count();
                let repr = unnamed.iter().map(|f| &f.ty);
                quote! {
                    #tag => ::std::option::Option::Some(Self::#ident(
                        #(
                            <#repr as #glib::FromVariant>::from_variant(
                                &#glib::Variant::try_child_value(&value, #indices)?
                            )?
                        ),*
                    )),
                }
            }
            syn::Fields::Unit => {
                quote! { #tag => ::std::option::Option::Some(Self::#ident), }
            }
        }
    });

    let (repr, tag_match) = match &mode {
        EnumMode::String => (quote! { String }, quote! { tag.as_str() }),
        EnumMode::Repr(repr) => (quote! { #repr }, quote! { tag as usize }),
        _ => unimplemented!(),
    };

    quote! {
        impl #impl_generics #glib::StaticVariantType for #ident #type_generics #where_clause {
            #[inline]
            fn static_variant_type() -> ::std::borrow::Cow<'static, #glib::VariantTy> {
                ::std::borrow::Cow::Borrowed(
                    unsafe {
                        #glib::VariantTy::from_str_unchecked(#static_variant_type)
                    }
                )
            }
        }

        impl #impl_generics #glib::ToVariant for #ident #type_generics #where_clause {
            fn to_variant(&self) -> #glib::Variant {
                match self {
                    #(#to),*
                }
            }
        }

        impl #impl_generics ::std::convert::From<#ident #type_generics> for #glib::Variant #where_clause {
            fn from(v: #ident #type_generics) -> #glib::Variant {
                match v {
                    #(#into),*
                }
            }
        }

        impl #impl_generics #glib::FromVariant for #ident #type_generics #where_clause {
            fn from_variant(variant: &#glib::Variant) -> ::std::option::Option<Self> {
                let (tag, value) = <(#repr, #glib::Variant) as #glib::FromVariant>::from_variant(&variant)?;
                if !#glib::VariantTy::is_tuple(#glib::Variant::type_(&value)) {
                    return ::std::option::Option::None;
                }
                match #tag_match {
                    #(#from)*
                    _ => ::std::option::Option::None
                }
            }
        }
    }
}

fn derive_variant_for_c_enum(
    ident: Ident,
    generics: Generics,
    data_enum: syn::DataEnum,
    mode: EnumMode,
) -> TokenStream {
    let glib = crate_ident_new();
    let static_variant_type = mode.tag_type().to_string();
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    let (to_variant, from_variant) = match mode {
        EnumMode::String => {
            let idents = data_enum.variants.iter().map(|v| &v.ident);
            let nicks = data_enum.variants.iter().map(|v| {
                let nick = ToKebabCase::to_kebab_case(v.ident.to_string().as_str());
                quote! { #nick }
            });
            let idents2 = idents.clone();
            let nicks2 = nicks.clone();
            (
                quote! {
                    #glib::ToVariant::to_variant(match self {
                        #(Self::#idents => #nicks),*
                    })
                },
                quote! {
                    let tag = #glib::Variant::str(&variant)?;
                    match tag {
                        #(#nicks2 => ::std::option::Option::Some(Self::#idents2),)*
                        _ => ::std::option::Option::None
                    }
                },
            )
        }
        EnumMode::Repr(repr) => {
            let idents = data_enum.variants.iter().map(|v| &v.ident);
            (
                quote! {
                    #glib::ToVariant::to_variant(&(*self as #repr))
                },
                quote! {
                    let value = <#repr as #glib::FromVariant>::from_variant(&variant)?;
                    #(if value == Self::#idents as #repr {
                        return Some(Self::#idents);
                    })*
                    None
                },
            )
        }
        EnumMode::Enum { repr: true } => (
            quote! {
                #glib::ToVariant::to_variant(&(*self as i32))
            },
            quote! {
                let value = <i32 as #glib::FromVariant>::from_variant(&variant)?;
                unsafe { #glib::translate::try_from_glib(value) }.ok()
            },
        ),
        EnumMode::Enum { repr: false } => (
            quote! {
                let ty = <Self as #glib::StaticType>::static_type();
                let enum_class = #glib::EnumClass::new(ty);
                let enum_class = ::std::option::Option::unwrap(enum_class);
                let value = <Self as #glib::translate::IntoGlib>::into_glib(*self);
                let value = #glib::EnumClass::value(&enum_class, value);
                let value = ::std::option::Option::unwrap(value);
                let nick = #glib::EnumValue::nick(&value);
                #glib::ToVariant::to_variant(nick)
            },
            quote! {
                let ty = <Self as #glib::StaticType>::static_type();
                let enum_class = #glib::EnumClass::new(ty)?;
                let tag = #glib::Variant::str(&variant)?;
                let value = #glib::EnumClass::value_by_nick(&enum_class, tag)?;
                let value = #glib::EnumValue::value(&value);
                unsafe { #glib::translate::try_from_glib(value) }.ok()
            },
        ),
        EnumMode::Flags { repr: true } => (
            quote! {
                #glib::ToVariant::to_variant(&self.bits())
            },
            quote! {
                let value = <u32 as #glib::FromVariant>::from_variant(&variant)?;
                Self::from_bits(value)
            },
        ),
        EnumMode::Flags { repr: false } => (
            quote! {
                let ty = <Self as #glib::StaticType>::static_type();
                let flags_class = #glib::FlagsClass::new(ty);
                let flags_class = ::std::option::Option::unwrap(flags_class);
                let value = <Self as #glib::translate::IntoGlib>::into_glib(*self);
                let s = #glib::FlagsClass::to_nick_string(&flags_class, value);
                #glib::ToVariant::to_variant(&s)
            },
            quote! {
                let ty = <Self as #glib::StaticType>::static_type();
                let flags_class = #glib::FlagsClass::new(ty)?;
                let s = #glib::Variant::str(&variant)?;
                let value = #glib::FlagsClass::from_nick_string(&flags_class, s).ok()?;
                Some(unsafe { #glib::translate::from_glib(value) })
            },
        ),
    };

    quote! {
        impl #impl_generics #glib::StaticVariantType for #ident #type_generics #where_clause {
            #[inline]
            fn static_variant_type() -> ::std::borrow::Cow<'static, #glib::VariantTy> {
                ::std::borrow::Cow::Borrowed(
                    unsafe {
                        #glib::VariantTy::from_str_unchecked(#static_variant_type)
                    }
                )
            }
        }

        impl #impl_generics #glib::ToVariant for #ident #type_generics #where_clause {
            fn to_variant(&self) -> #glib::Variant {
                #to_variant
            }
        }

        impl #impl_generics ::std::convert::From<#ident #type_generics> for #glib::Variant #where_clause {
            #[inline]
            fn from(v: #ident #type_generics) -> #glib::Variant {
                <#ident #type_generics as #glib::ToVariant>::to_variant(&v)
            }
        }

        impl #impl_generics #glib::FromVariant for #ident #type_generics #where_clause {
            fn from_variant(variant: &#glib::Variant) -> ::std::option::Option<Self> {
                #from_variant
            }
        }
    }
}

#[derive(deluxe::ExtractAttributes, Default)]
#[deluxe(attributes(variant_enum))]
struct VariantEnum {
    #[deluxe(flatten)]
    mode: EnumMode,
    repr: deluxe::Flag,
}

fn get_enum_mode(attrs: &mut Vec<syn::Attribute>, errors: &deluxe::Errors) -> EnumMode {
    let VariantEnum { mode, repr } = deluxe::extract_attributes_optional(attrs, errors);
    match mode {
        EnumMode::String if repr.is_set() => {
            let repr = get_repr(attrs).unwrap_or_else(|| {
                errors.push(
                    syn::spanned::Spanned::span(&repr),
                    "#[derive(glib::Variant)] can only have #[repr(...)] with one of i8, i16, i32, i64, u8, u16, u32, u64"
                );
                format_ident!("i32")
            });
            EnumMode::Repr(repr)
        }
        EnumMode::Enum { .. } => EnumMode::Enum {
            repr: repr.is_set(),
        },
        EnumMode::Flags { .. } => EnumMode::Flags {
            repr: repr.is_set(),
        },
        e => e,
    }
}

fn get_repr(attrs: &mut Vec<syn::Attribute>) -> Option<Ident> {
    #[derive(deluxe::ExtractAttributes)]
    #[deluxe(attributes(repr))]
    struct Repr(Ident);

    let Repr(repr) = deluxe::extract_attributes(attrs).ok()?;
    match repr.to_string().as_str() {
        "i8" | "i16" | "i32" | "i64" | "u8" | "u16" | "u32" | "u64" => Some(repr),
        _ => None,
    }
}
