// Take a look at the license at the top of the repository in the LICENSE file.

use crate::utils::crate_ident_new;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput};
use syn::{Fields, FieldsNamed, FieldsUnnamed, Generics, Ident, Type};

pub fn impl_variant(input: DeriveInput) -> TokenStream {
    match input.data {
        Data::Struct(data_struct) => {
            derive_variant_for_struct(input.ident, input.generics, data_struct)
        }
        Data::Enum(_) => {
            panic!("#[derive(glib::Variant)] is not available for enums.");
        }
        Data::Union(..) => {
            panic!("#[derive(glib::Variant)] is not available for unions.");
        }
    }
}

pub fn derive_variant_for_struct(
    ident: Ident,
    generics: Generics,
    data_struct: syn::DataStruct,
) -> TokenStream {
    let glib = crate_ident_new();
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
                impl #generics #glib::StaticVariantType for #ident #generics {
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
                impl #generics #glib::ToVariant for #ident #generics {
                    fn to_variant(&self) -> #glib::Variant {
                        #glib::Variant::tuple_from_iter(::std::array::IntoIter::<#glib::Variant, #idents_len>::new([
                            #(
                                #glib::ToVariant::to_variant(&self.#idents)
                            ),*
                        ]))
                    }
                }
            };

            let from_variant = quote! {
                impl #generics #glib::FromVariant for #ident #generics {
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
                impl #generics #glib::StaticVariantType for #ident #generics {
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
                impl #generics #glib::ToVariant for #ident #generics {
                    fn to_variant(&self) -> #glib::Variant {
                        #glib::Variant::tuple_from_iter(::std::array::IntoIter::new([
                            #(
                                #glib::ToVariant::to_variant(&self.#idents)
                            ),*
                        ]))
                    }
                }
            };

            let from_variant = quote! {
                impl #generics #glib::FromVariant for #ident #generics {
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
                impl #generics #glib::StaticVariantType for #ident #generics {
                    fn static_variant_type() -> ::std::borrow::Cow<'static, #glib::VariantTy> {
                        ::std::borrow::Cow::Borrowed(#glib::VariantTy::UNIT)
                    }
                }
            };

            let to_variant = quote! {
                impl #generics #glib::ToVariant for #ident #generics {
                    fn to_variant(&self) -> #glib::Variant {
                        #glib::ToVariant::to_variant(&())
                    }
                }
            };

            let from_variant = quote! {
                impl #generics #glib::FromVariant for #ident #generics {
                    fn from_variant(variant: &#glib::Variant) -> ::core::option::Option<Self> {
                        Some(Self)
                    }
                }
            };

            (static_variant_type, to_variant, from_variant)
        }
    };

    let derived = quote! {
        #static_variant_type

        #to_variant

        #from_variant
    };

    derived.into()
}
