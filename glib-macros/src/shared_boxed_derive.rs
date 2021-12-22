// Take a look at the license at the top of the repository in the LICENSE file.

use proc_macro2::{Ident, TokenStream};
use proc_macro_error::abort_call_site;
use quote::quote;

use crate::utils::{crate_ident_new, find_attribute_meta, find_nested_meta, parse_name};

fn gen_impl_to_value_optional(name: &Ident, crate_ident: &TokenStream) -> TokenStream {
    let refcounted_type_prefix = refcounted_type_prefix(name, crate_ident);

    quote! {
        impl #crate_ident::value::ToValueOptional for #name {
            fn to_value_optional(s: ::core::option::Option<&Self>) -> #crate_ident::Value {
                let mut value = #crate_ident::Value::for_value_type::<Self>();
                unsafe {
                    let ptr = match s {
                        Some(s) => #refcounted_type_prefix::into_raw(s.0.clone()),
                        None => ::std::ptr::null(),
                    };

                    #crate_ident::gobject_ffi::g_value_take_boxed(
                        #crate_ident::translate::ToGlibPtrMut::to_glib_none_mut(&mut value).0,
                        ptr as *mut _
                    );
                }

                value
            }
        }

        impl #crate_ident::value::ValueTypeOptional for #name { }
    }
}

fn gen_impl_from_value_optional(name: &Ident, crate_ident: &TokenStream) -> TokenStream {
    let refcounted_type_prefix = refcounted_type_prefix(name, crate_ident);

    quote! {
        unsafe impl<'a> #crate_ident::value::FromValue<'a> for #name {
            type Checker = #crate_ident::value::GenericValueTypeOrNoneChecker<Self>;

            unsafe fn from_value(value: &'a #crate_ident::Value) -> Self {
                let ptr = #crate_ident::gobject_ffi::g_value_dup_boxed(#crate_ident::translate::ToGlibPtr::to_glib_none(value).0);
                assert!(!ptr.is_null());
                #name(#refcounted_type_prefix::from_raw(ptr as *mut _))
            }
        }
    }
}

fn gen_impl_from_value(name: &Ident, crate_ident: &TokenStream) -> TokenStream {
    let refcounted_type_prefix = refcounted_type_prefix(name, crate_ident);

    quote! {
        unsafe impl<'a> #crate_ident::value::FromValue<'a> for #name {
            type Checker = #crate_ident::value::GenericValueTypeChecker<Self>;

            unsafe fn from_value(value: &'a #crate_ident::Value) -> Self {
                let ptr = #crate_ident::gobject_ffi::g_value_dup_boxed(#crate_ident::translate::ToGlibPtr::to_glib_none(value).0);
                assert!(!ptr.is_null());
                #name(#refcounted_type_prefix::from_raw(ptr as *mut _))
            }
        }
    }
}

fn refcounted_type(input: &syn::DeriveInput) -> Option<&syn::TypePath> {
    let fields = match &input.data {
        syn::Data::Struct(s) => &s.fields,
        _ => return None,
    };

    let unnamed = match fields {
        syn::Fields::Unnamed(u) if u.unnamed.len() == 1 => &u.unnamed[0],
        _ => return None,
    };

    let refcounted = match &unnamed.ty {
        syn::Type::Path(p) => p,
        _ => return None,
    };

    Some(refcounted)
}

fn refcounted_type_prefix(name: &Ident, crate_ident: &TokenStream) -> proc_macro2::TokenStream {
    quote! {
        <<#name as #crate_ident::subclass::shared::SharedType>::RefCountedType as #crate_ident::subclass::shared::RefCounted>
    }
}

pub fn impl_shared_boxed(input: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let name = &input.ident;

    let refcounted_type = match refcounted_type(input) {
        Some(p) => p,
        _ => {
            abort_call_site!("#[derive(glib::SharedBoxed)] requires struct MyStruct(T: RefCounted)")
        }
    };

    let gtype_name = match parse_name(input, "shared_boxed_type") {
        Ok(name) => name,
        Err(e) => abort_call_site!(
            "{}: #[derive(glib::SharedBoxed)] requires #[shared_boxed_type(name = \"SharedBoxedTypeName\")]",
            e
        ),
    };

    let meta = find_attribute_meta(&input.attrs, "shared_boxed_type")
        .unwrap()
        .unwrap();
    let nullable = find_nested_meta(&meta, "nullable").is_some();

    let crate_ident = crate_ident_new();
    let refcounted_type_prefix = refcounted_type_prefix(name, &crate_ident);

    let impl_from_value = if !nullable {
        gen_impl_from_value(name, &crate_ident)
    } else {
        gen_impl_from_value_optional(name, &crate_ident)
    };

    let impl_to_value_optional = if nullable {
        gen_impl_to_value_optional(name, &crate_ident)
    } else {
        quote! {}
    };

    quote! {
        impl #crate_ident::subclass::shared::SharedType for #name {
            const NAME: &'static str = #gtype_name;

            type RefCountedType = #refcounted_type;

            fn from_refcounted(this: Self::RefCountedType) -> Self {
                Self(this)
            }

            fn into_refcounted(self) -> Self::RefCountedType {
                self.0
            }
        }

        impl #crate_ident::StaticType for #name {
            fn static_type() -> #crate_ident::Type {
                static ONCE: ::std::sync::Once = ::std::sync::Once::new();
                static mut TYPE_: #crate_ident::Type = #crate_ident::Type::INVALID;

                ONCE.call_once(|| {
                    let type_ = #crate_ident::subclass::shared::register_shared_type::<#name>();
                    unsafe {
                        TYPE_ = type_;
                    }
                });

                unsafe { TYPE_ }
            }
        }

        impl #crate_ident::value::ValueType for #name {
            type Type = #name;
        }

        impl #crate_ident::value::ToValue for #name {
            fn to_value(&self) -> #crate_ident::Value {
                unsafe {
                    let ptr = #refcounted_type_prefix::into_raw(self.0.clone());
                    let mut value = #crate_ident::Value::from_type(<#name as #crate_ident::StaticType>::static_type());
                    #crate_ident::gobject_ffi::g_value_take_boxed(
                        #crate_ident::translate::ToGlibPtrMut::to_glib_none_mut(&mut value).0,
                        ptr as *mut _
                    );
                    value
                }
            }

            fn value_type(&self) -> #crate_ident::Type {
                <#name as #crate_ident::StaticType>::static_type()
            }
        }

        #impl_to_value_optional

        #impl_from_value

        impl #crate_ident::translate::FromGlibPtrNone<*const #refcounted_type_prefix::InnerType> for #name {
            #[inline]
            unsafe fn from_glib_none(ptr: *const #refcounted_type_prefix::InnerType) -> Self {
                let ptr = #refcounted_type_prefix::ref_(ptr);
                #name(#refcounted_type_prefix::from_raw(ptr))
            }
        }

        impl #crate_ident::translate::FromGlibPtrNone<*mut #refcounted_type_prefix::InnerType> for #name {
            #[inline]
            unsafe fn from_glib_none(ptr: *mut #refcounted_type_prefix::InnerType) -> Self {
                #crate_ident::translate::FromGlibPtrNone::from_glib_none(ptr as *const _)
            }
        }

        impl #crate_ident::translate::FromGlibPtrFull<*mut #refcounted_type_prefix::InnerType> for #name {
            #[inline]
            unsafe fn from_glib_full(ptr: *mut #refcounted_type_prefix::InnerType) -> Self {
                #name(#refcounted_type_prefix::from_raw(ptr))
            }
        }

        impl<'a> #crate_ident::translate::ToGlibPtr<'a, *const #refcounted_type_prefix::InnerType> for #name {
            type Storage = &'a Self;

            #[inline]
            fn to_glib_none(&'a self) -> #crate_ident::translate::Stash<'a, *const #refcounted_type_prefix::InnerType, Self> {
                unsafe {
                    #crate_ident::translate::Stash(#refcounted_type_prefix::as_ptr(&self.0), self)
                }
            }

            #[inline]
            fn to_glib_full(&self) -> *const #refcounted_type_prefix::InnerType {
                let r = <#name as #crate_ident::subclass::shared::SharedType>::into_refcounted(self.clone());
                unsafe {
                    #refcounted_type_prefix::into_raw(r)
                }
            }
        }

        impl<'a> #crate_ident::translate::ToGlibPtr<'a, *mut #refcounted_type_prefix::InnerType> for #name {
            type Storage = &'a Self;

            #[inline]
            fn to_glib_none(&'a self) -> #crate_ident::translate::Stash<'a, *mut #refcounted_type_prefix::InnerType, Self> {
                unsafe {
                    #crate_ident::translate::Stash(#refcounted_type_prefix::as_ptr(&self.0) as *mut _, self)
                }
            }

            #[inline]
            fn to_glib_full(&self) -> *mut #refcounted_type_prefix::InnerType {
                let r = <#name as #crate_ident::subclass::shared::SharedType>::into_refcounted(self.clone());
                unsafe {
                    #refcounted_type_prefix::into_raw(r) as *mut _
                }
            }
        }
    }
}
