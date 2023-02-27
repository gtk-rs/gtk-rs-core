// Take a look at the license at the top of the repository in the LICENSE file.

use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;

#[derive(deluxe::ParseMetaItem)]
pub struct Boxed {
    copy: syn::Expr,
    free: syn::Expr,
    r#type: Option<syn::Expr>,
    #[deluxe(flatten)]
    args: super::WrapperArgs,
}

impl Boxed {
    pub fn into_token_stream(self, item: &mut syn::ItemStruct) -> syn::Result<TokenStream> {
        let crate_ = crate::utils::crate_ident_new();
        let ffi_name = super::get_first_field_type_param(&item.fields, 0)?.clone();
        if let Ok(second) = super::get_first_field_type_param(&item.fields, 1) {
            return Err(syn::Error::new_spanned(second, "unexpected tokens"));
        }
        super::add_repr_transparent(item);
        item.fields.iter_mut().next().unwrap().ty = syn::parse_quote! {
            #crate_::boxed::Boxed<#ffi_name, Self>
        };

        let Boxed {
            copy,
            free,
            r#type,
            args: super::WrapperArgs { skipped_traits },
        } = self;

        let struct_ident = &item.ident;
        let (impl_generics, type_generics, where_clause) = item.generics.split_for_impl();

        let mut lt_generics = item.generics.clone();
        let lt = super::insert_lifetime("a", &mut lt_generics);
        let (lt_impl_generics, _, _) = lt_generics.split_for_impl();

        let mut tokens = quote! {
            impl #impl_generics #struct_ident #type_generics #where_clause {
                #[doc = "Return the inner pointer to the underlying C value."]
                #[inline]
                pub fn as_ptr(&self) -> *mut #ffi_name {
                    unsafe { *(self as *const Self as *const *const #ffi_name) as *mut #ffi_name }
                }

                #[doc = "Borrows the underlying C value."]
                #[inline]
                pub unsafe fn from_glib_ptr_borrow<#lt>(ptr: *const *const #ffi_name) -> &#lt Self {
                    &*(ptr as *const Self)
                }

                #[doc = "Borrows the underlying C value mutably."]
                #[inline]
                pub unsafe fn from_glib_ptr_borrow_mut<#lt>(ptr: *mut *mut #ffi_name) -> &#lt mut Self {
                    &mut *(ptr as *mut Self)
                }
            }
        };

        let inner = super::Member::from_fields(&item.fields).next().unwrap();

        if !skipped_traits.contains("Clone") {
            let stmt = super::construct_stmt(
                &item.fields,
                |m| quote_spanned! { m.span() => ::std::clone::Clone::clone(&self.#m) },
                |m| quote_spanned! { m.span() => ::std::clone::Clone::clone(&self.#m) },
            );
            tokens.extend(quote! {
                impl #impl_generics ::std::clone::Clone for #struct_ident #type_generics #where_clause {
                    #[inline]
                    fn clone(&self) -> Self {
                        #stmt
                    }
                }
            });
        }
        if !skipped_traits.contains("GlibPtrDefault") {
            tokens.extend(quote! {
                #[doc(hidden)]
                impl #impl_generics #crate_::translate::GlibPtrDefault for #struct_ident #type_generics #where_clause {
                    type GlibType = *mut #ffi_name;
                }
            });
        }
        if !skipped_traits.contains("TransparentPtrType") {
            tokens.extend(quote! {
                #[doc(hidden)]
                unsafe impl #impl_generics #crate_::translate::TransparentPtrType for #struct_ident #type_generics #where_clause {}
            });
        }
        if !skipped_traits.contains("ToGlibPtr") {
            tokens.extend(quote! {
                #[doc(hidden)]
                impl #lt_impl_generics #crate_::translate::ToGlibPtr<#lt, *const #ffi_name> for #struct_ident #type_generics #where_clause {
                    type Storage = ::std::marker::PhantomData<&#lt #crate_::boxed::Boxed<#ffi_name, Self>>;

                    #[inline]
                    fn to_glib_none(&#lt self) -> #crate_::translate::Stash<#lt, *const #ffi_name, Self> {
                        let stash = #crate_::translate::ToGlibPtr::to_glib_none(&self.#inner);
                        #crate_::translate::Stash(stash.0, stash.1)
                    }
                    #[inline]
                    fn to_glib_full(&self) -> *const #ffi_name {
                        #crate_::translate::ToGlibPtr::to_glib_full(&self.#inner)
                    }
                }
                #[doc(hidden)]
                impl #lt_impl_generics #crate_::translate::ToGlibPtr<#lt, *mut #ffi_name> for #struct_ident #type_generics #where_clause {
                    type Storage = ::std::marker::PhantomData<&#lt #crate_::boxed::Boxed<#ffi_name, Self>>;

                    #[inline]
                    fn to_glib_none(&#lt self) -> #crate_::translate::Stash<#lt, *mut #ffi_name, Self> {
                        let stash = #crate_::translate::ToGlibPtr::to_glib_none(&self.#inner);
                        #crate_::translate::Stash(stash.0 as *mut _, stash.1)
                    }
                    #[inline]
                    fn to_glib_full(&self) -> *mut #ffi_name {
                        #crate_::translate::ToGlibPtr::to_glib_full(&self.#inner) as *mut _
                    }
                }
            });
        }
        if !skipped_traits.contains("ToGlibPtrMut") {
            tokens.extend(quote! {
                #[doc(hidden)]
                impl #lt_impl_generics #crate_::translate::ToGlibPtrMut<#lt, *mut #ffi_name> for #struct_ident #type_generics #where_clause {
                    type Storage = ::std::marker::PhantomData<&#lt mut #crate_::boxed::Boxed<#ffi_name, Self>>;

                    #[inline]
                    fn to_glib_none_mut(&#lt mut self) -> #crate_::translate::StashMut<#lt, *mut #ffi_name, Self> {
                        let stash = #crate_::translate::ToGlibPtrMut::to_glib_none_mut(&mut self.#inner);
                        #crate_::translate::StashMut(stash.0, stash.1)
                    }
                }
            });
        }
        if !skipped_traits.contains("ToGlibContainerFromSlice") {
            tokens.extend(quote! {
                #[doc(hidden)]
                impl #lt_impl_generics #crate_::translate::ToGlibContainerFromSlice<#lt, *mut *const #ffi_name> for #struct_ident #type_generics #where_clause {
                    type Storage = (::std::marker::PhantomData<&#lt [Self]>, ::std::option::Option<::std::vec::Vec<*const #ffi_name>>);

                    fn to_glib_none_from_slice(t: &#lt [Self]) -> (*mut *const #ffi_name, Self::Storage) {
                        let mut v_ptr = ::std::vec::Vec::with_capacity(t.len() + 1);
                        unsafe {
                            let ptr = v_ptr.as_mut_ptr();
                            ::std::ptr::copy_nonoverlapping(t.as_ptr() as *mut *const #ffi_name, ptr, t.len());
                            ::std::ptr::write(ptr.add(t.len()), ::std::ptr::null_mut());
                            v_ptr.set_len(t.len() + 1);
                        }

                        (v_ptr.as_ptr() as *mut *const #ffi_name, (::std::marker::PhantomData, Some(v_ptr)))
                    }
                    fn to_glib_container_from_slice(t: &#lt [Self]) -> (*mut *const #ffi_name, Self::Storage) {
                        let v_ptr = unsafe {
                            let v_ptr = #crate_::ffi::g_malloc(::std::mem::size_of::<*const #ffi_name>() * (t.len() + 1)) as *mut *const #ffi_name;

                            ::std::ptr::copy_nonoverlapping(t.as_ptr() as *mut *const #ffi_name, v_ptr, t.len());
                            ::std::ptr::write(v_ptr.add(t.len()), ::std::ptr::null_mut());

                            v_ptr
                        };

                        (v_ptr, (::std::marker::PhantomData, None))
                    }
                    fn to_glib_full_from_slice(t: &[Self]) -> *mut *const #ffi_name {
                        unsafe {
                            let v_ptr = #crate_::ffi::g_malloc(::std::mem::size_of::<*const #ffi_name>() * (t.len() + 1)) as *mut *const #ffi_name;

                            for (i, s) in t.iter().enumerate() {
                                ::std::ptr::write(v_ptr.add(i), #crate_::translate::ToGlibPtr::to_glib_full(s));
                            }
                            ::std::ptr::write(v_ptr.add(t.len()), ::std::ptr::null_mut());

                            v_ptr
                        }
                    }
                }
                #[doc(hidden)]
                impl #lt_impl_generics #crate_::translate::ToGlibContainerFromSlice<#lt, *const *const #ffi_name> for #struct_ident #type_generics #where_clause {
                    type Storage = (::std::marker::PhantomData<&#lt [Self]>, ::std::option::Option<::std::vec::Vec<*const #ffi_name>>);

                    #[inline]
                    fn to_glib_none_from_slice(t: &#lt [Self]) -> (*const *const #ffi_name, Self::Storage) {
                        let (ptr, stash) = #crate_::translate::ToGlibContainerFromSlice::<#lt, *mut *const #ffi_name>::to_glib_none_from_slice(t);
                        (ptr as *const *const #ffi_name, stash)
                    }
                    #[inline]
                    fn to_glib_container_from_slice(_: &#lt [Self]) -> (*const *const #ffi_name, Self::Storage) {
                        // Can't have consumer free a *const pointer
                        ::std::unimplemented!()
                    }
                    #[inline]
                    fn to_glib_full_from_slice(_: &[Self]) -> *const *const #ffi_name {
                        // Can't have consumer free a *const pointer
                        ::std::unimplemented!()
                    }
                }
            });
        }
        if !skipped_traits.contains("FromGlibPtrNone") {
            let stmt = super::construct_stmt(
                &item.fields,
                |m| quote_spanned! { m.span() => #crate_::translate::from_glib_none(ptr) },
                |m| quote_spanned! { m.span() => ::std::default::Default::default() },
            );
            tokens.extend(quote! {
                #[doc(hidden)]
                impl #impl_generics #crate_::translate::FromGlibPtrNone<*mut #ffi_name> for #struct_ident #type_generics #where_clause {
                    #[inline]
                    unsafe fn from_glib_none(ptr: *mut #ffi_name) -> Self {
                        #stmt
                    }
                }
                #[doc(hidden)]
                impl #impl_generics #crate_::translate::FromGlibPtrNone<*const #ffi_name> for #struct_ident #type_generics #where_clause {
                    #[inline]
                    unsafe fn from_glib_none(ptr: *const #ffi_name) -> Self {
                        #stmt
                    }
                }
            });
        }
        if !skipped_traits.contains("FromGlibPtrFull") {
            let stmt = super::construct_stmt(
                &item.fields,
                |m| quote_spanned! { m.span() => #crate_::translate::from_glib_full(ptr) },
                |m| quote_spanned! { m.span() => ::std::default::Default::default() },
            );
            tokens.extend(quote! {
                #[doc(hidden)]
                impl #impl_generics #crate_::translate::FromGlibPtrFull<*mut #ffi_name> for #struct_ident #type_generics #where_clause {
                    #[inline]
                    unsafe fn from_glib_full(ptr: *mut #ffi_name) -> Self {
                        #stmt
                    }
                }
                #[doc(hidden)]
                impl #impl_generics #crate_::translate::FromGlibPtrFull<*const #ffi_name> for #struct_ident #type_generics #where_clause {
                    #[inline]
                    unsafe fn from_glib_full(ptr: *const #ffi_name) -> Self {
                        #stmt
                    }
                }
            });
        }
        if !skipped_traits.contains("FromGlibPtrBorrow") {
            let stmt = super::construct_stmt(
                &item.fields,
                |m| quote_spanned! { m.span() => #crate_::translate::from_glib_borrow::<_, #crate_::boxed::Boxed<_, _>>(ptr).into_inner() },
                |m| quote_spanned! { m.span() => ::std::default::Default::default() },
            );
            tokens.extend(quote! {
                #[doc(hidden)]
                impl #impl_generics #crate_::translate::FromGlibPtrBorrow<*mut #ffi_name> for #struct_ident #type_generics #where_clause {
                    #[inline]
                    unsafe fn from_glib_borrow(ptr: *mut #ffi_name) -> #crate_::translate::Borrowed<Self> {
                        #crate_::translate::Borrowed::new(
                            #stmt
                        )
                    }
                }
                #[doc(hidden)]
                impl #impl_generics #crate_::translate::FromGlibPtrBorrow<*const #ffi_name> for #struct_ident #type_generics #where_clause {
                    #[inline]
                    unsafe fn from_glib_borrow(ptr: *const #ffi_name) -> #crate_::translate::Borrowed<Self> {
                        #crate_::translate::from_glib_borrow::<_, Self>(ptr as *mut #ffi_name)
                    }
                }
            });
        }
        if !skipped_traits.contains("FromGlibContainerAsVec") {
            tokens.extend(quote! {
                #[doc(hidden)]
                impl #impl_generics #crate_::translate::FromGlibContainerAsVec<*mut #ffi_name, *mut *mut #ffi_name> for #struct_ident #type_generics #where_clause {
                    unsafe fn from_glib_none_num_as_vec(ptr: *mut *mut #ffi_name, num: usize) -> ::std::vec::Vec<Self> {
                        if num == 0 || ptr.is_null() {
                            return ::std::vec::Vec::new();
                        }

                        let mut res = ::std::vec::Vec::<Self>::with_capacity(num);
                        let res_ptr = res.as_mut_ptr();
                        for i in 0..num {
                            ::std::ptr::write(res_ptr.add(i), #crate_::translate::from_glib_none(::std::ptr::read(ptr.add(i))));
                        }
                        res.set_len(num);
                        res
                    }
                    unsafe fn from_glib_container_num_as_vec(ptr: *mut *mut #ffi_name, num: usize) -> ::std::vec::Vec<Self> {
                        let res = #crate_::translate::FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr, num);
                        #crate_::ffi::g_free(ptr as *mut _);
                        res
                    }
                    unsafe fn from_glib_full_num_as_vec(ptr: *mut *mut #ffi_name, num: usize) -> ::std::vec::Vec<Self> {
                        if num == 0 || ptr.is_null() {
                            #crate_::ffi::g_free(ptr as *mut _);
                            return ::std::vec::Vec::new();
                        }

                        let mut res = ::std::vec::Vec::with_capacity(num);
                        let res_ptr = res.as_mut_ptr();
                        ::std::ptr::copy_nonoverlapping(ptr as *mut Self, res_ptr, num);
                        res.set_len(num);
                        #crate_::ffi::g_free(ptr as *mut _);
                        res
                    }
                }
            });
        }
        if !skipped_traits.contains("FromGlibPtrArrayContainerAsVec") {
            tokens.extend(quote! {
                #[doc(hidden)]
                impl #impl_generics #crate_::translate::FromGlibPtrArrayContainerAsVec<*mut #ffi_name, *mut *mut #ffi_name> for #struct_ident #type_generics #where_clause {
                    #[inline]
                    unsafe fn from_glib_none_as_vec(ptr: *mut *mut #ffi_name) -> ::std::vec::Vec<Self> {
                        #crate_::translate::FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr, #crate_::translate::c_ptr_array_len(ptr))
                    }
                    #[inline]
                    unsafe fn from_glib_container_as_vec(ptr: *mut *mut #ffi_name) -> ::std::vec::Vec<Self> {
                        #crate_::translate::FromGlibContainerAsVec::from_glib_container_num_as_vec(ptr, #crate_::translate::c_ptr_array_len(ptr))
                    }
                    #[inline]
                    unsafe fn from_glib_full_as_vec(ptr: *mut *mut #ffi_name) -> ::std::vec::Vec<Self> {
                        #crate_::translate::FromGlibContainerAsVec::from_glib_full_num_as_vec(ptr, #crate_::translate::c_ptr_array_len(ptr))
                    }
                }
            });
        }
        if !skipped_traits.contains("IntoGlibPtr") {
            tokens.extend(quote! {
                #[doc(hidden)]
                impl #impl_generics #crate_::translate::IntoGlibPtr<*mut #ffi_name> for #struct_ident #type_generics #where_clause {
                    #[inline]
                    unsafe fn into_glib_ptr(self) -> *mut #ffi_name {
                        let s = ::std::mem::ManuallyDrop::new(self);
                        #crate_::translate::ToGlibPtr::<*const #ffi_name>::to_glib_none(&*s).0 as *mut _
                    }
                }
                #[doc(hidden)]
                impl #impl_generics #crate_::translate::IntoGlibPtr<*const #ffi_name> for #struct_ident #type_generics #where_clause {
                    #[inline]
                    unsafe fn into_glib_ptr(self) -> *const #ffi_name {
                        let s = ::std::mem::ManuallyDrop::new(self);
                        #crate_::translate::ToGlibPtr::<*const #ffi_name>::to_glib_none(&*s).0 as *const _
                    }
                }
            });
        }
        if let Some(get_type_expr) = r#type {
            if !skipped_traits.contains("StaticType") {
                tokens.extend(quote! {
                    impl #impl_generics #crate_::types::StaticType for #struct_ident #type_generics #where_clause {
                        #[inline]
                        fn static_type() -> #crate_::types::Type {
                            #[allow(unused_unsafe)]
                            unsafe { #crate_::translate::from_glib((#get_type_expr)()) }
                        }
                    }
                });
            }
            if !skipped_traits.contains("ValueType") {
                tokens.extend(quote! {
                    #[doc(hidden)]
                    impl #impl_generics #crate_::value::ValueType for #struct_ident #type_generics #where_clause {
                        type Type = Self;
                    }
                });
            }
            if !skipped_traits.contains("ValueTypeOptional") {
                tokens.extend(quote! {
                    #[doc(hidden)]
                    impl #impl_generics #crate_::value::ValueTypeOptional for #struct_ident #type_generics #where_clause {}
                });
            }
            if !skipped_traits.contains("FromValue") {
                tokens.extend(quote! {
                    #[doc(hidden)]
                    unsafe impl #lt_impl_generics #crate_::value::FromValue<#lt> for #struct_ident #type_generics #where_clause {
                        type Checker = #crate_::value::GenericValueTypeOrNoneChecker<Self>;

                        #[inline]
                        unsafe fn from_value(value: &#lt #crate_::Value) -> Self {
                            let ptr = #crate_::gobject_ffi::g_value_dup_boxed(#crate_::translate::ToGlibPtr::to_glib_none(value).0);
                            debug_assert!(!ptr.is_null());
                            <Self as #crate_::translate::FromGlibPtrFull<*mut #ffi_name>>::from_glib_full(ptr as *mut #ffi_name)
                        }
                    }

                    #[doc(hidden)]
                    unsafe impl #lt_impl_generics #crate_::value::FromValue<#lt> for &#lt #struct_ident #type_generics #where_clause {
                        type Checker = #crate_::value::GenericValueTypeOrNoneChecker<Self>;

                        #[inline]
                        unsafe fn from_value(value: &#lt #crate_::Value) -> Self {
                            debug_assert_eq!(::std::mem::size_of::<Self>(), ::std::mem::size_of::<#crate_::ffi::gpointer>());
                            let value = &*(value as *const #crate_::Value as *const #crate_::gobject_ffi::GValue);
                            debug_assert!(!value.data[0].v_pointer.is_null());
                            <#struct_ident #type_generics>::from_glib_ptr_borrow(&value.data[0].v_pointer as *const #crate_::ffi::gpointer as *const *const #ffi_name)
                        }
                    }
                });
            }
            if !skipped_traits.contains("ToValue") {
                tokens.extend(quote! {
                    #[doc(hidden)]
                    impl #impl_generics #crate_::value::ToValue for #struct_ident #type_generics #where_clause {
                        #[inline]
                        fn to_value(&self) -> #crate_::Value {
                            unsafe {
                                let mut value = #crate_::Value::from_type_unchecked(<Self as #crate_::StaticType>::static_type());
                                #crate_::gobject_ffi::g_value_take_boxed(
                                    #crate_::translate::ToGlibPtrMut::to_glib_none_mut(&mut value).0,
                                    #crate_::translate::ToGlibPtr::<*const #ffi_name>::to_glib_full(self) as *mut _,
                                );
                                value
                            }
                        }

                        #[inline]
                        fn value_type(&self) -> #crate_::Type {
                            <Self as #crate_::StaticType>::static_type()
                        }
                    }
                });
            }
            if !skipped_traits.contains("From") {
                tokens.extend(quote! {
                    #[doc(hidden)]
                    impl #impl_generics ::std::convert::From<#struct_ident #type_generics> for #crate_::Value #where_clause {
                        #[inline]
                        fn from(o: #struct_ident #type_generics) -> Self {
                            unsafe {
                                let mut value = #crate_::Value::from_type_unchecked(
                                    <#struct_ident #type_generics as #crate_::StaticType>::static_type(),
                                );
                                #crate_::gobject_ffi::g_value_take_boxed(
                                    #crate_::translate::ToGlibPtrMut::to_glib_none_mut(&mut value).0,
                                    #crate_::translate::IntoGlibPtr::<*mut #ffi_name>::into_glib_ptr(o) as *mut _,
                                );
                                value
                            }
                        }
                    }
                });
            }
            if !skipped_traits.contains("ToValueOptional") {
                tokens.extend(quote! {
                    #[doc(hidden)]
                    impl #impl_generics #crate_::value::ToValueOptional for #struct_ident #type_generics #where_clause {
                        #[inline]
                        fn to_value_optional(s: Option<&Self>) -> #crate_::Value {
                            let mut value = #crate_::Value::for_value_type::<Self>();
                            unsafe {
                                #crate_::gobject_ffi::g_value_take_boxed(
                                    #crate_::translate::ToGlibPtrMut::to_glib_none_mut(&mut value).0,
                                    #crate_::translate::ToGlibPtr::<*const #ffi_name>::to_glib_full(&s) as *mut _,
                                );
                            }

                            value
                        }
                    }
                });
            }
            if !skipped_traits.contains("HasParamSpec") {
                tokens.extend(quote! {
                    impl #impl_generics #crate_::HasParamSpec for #struct_ident #type_generics #where_clause {
                        type ParamSpec = #crate_::ParamSpecBoxed;
                        type SetValue = Self;
                        type BuilderFn = fn(&str) -> #crate_::ParamSpecBoxedBuilder<Self>;

                        fn param_spec_builder() -> Self::BuilderFn {
                            |name| Self::ParamSpec::builder(name)
                        }
                    }
                });
            }
        }
        if !skipped_traits.contains("BoxedMemoryManager") {
            tokens.extend(quote! {
                #[doc(hidden)]
                    impl #impl_generics #crate_::boxed::BoxedMemoryManager<#ffi_name> for #struct_ident #type_generics #where_clause {
                    #[inline]
                    unsafe fn copy(ptr: *const #ffi_name) -> *mut #ffi_name {
                        (#copy)(ptr)
                    }

                    #[inline]
                    #[allow(clippy::no_effect)]
                    unsafe fn free(ptr: *mut #ffi_name) {
                        (#free)(ptr)
                    }
                }
            });
        }
        Ok(tokens)
    }
}
