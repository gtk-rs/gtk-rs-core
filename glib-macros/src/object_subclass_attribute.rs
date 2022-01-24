// Take a look at the license at the top of the repository in the LICENSE file.

use proc_macro2::TokenStream;
use proc_macro_error::abort_call_site;
use quote::quote;

pub const WRONG_PLACE_MSG: &str =
    "This macro should be used on `impl` block for `glib::ObjectSubclass` trait";

pub fn impl_object_subclass(input: &syn::ItemImpl) -> TokenStream {
    let mut has_new = false;
    let mut has_parent_type = false;
    let mut has_interfaces = false;
    let mut has_instance = false;
    let mut has_class = false;
    for item in &input.items {
        match item {
            syn::ImplItem::Method(method) => {
                let name = &method.sig.ident;
                if name == "new" || name == "with_class" {
                    has_new = true;
                }
            }
            syn::ImplItem::Type(type_) => {
                let name = &type_.ident;
                if name == "ParentType" {
                    has_parent_type = true;
                } else if name == "Interfaces" {
                    has_interfaces = true;
                } else if name == "Instance" {
                    has_instance = true;
                } else if name == "Class" {
                    has_class = true;
                }
            }
            _ => {}
        }
    }

    let syn::ItemImpl {
        attrs,
        generics,
        trait_,
        self_ty,
        items,
        ..
    } = &input;

    let crate_ident = crate::utils::crate_ident_new();

    let parent_type_opt = (!has_parent_type).then(|| {
        quote!(
            type ParentType = #crate_ident::Object;
        )
    });

    let interfaces_opt = (!has_interfaces).then(|| {
        quote!(
            type Interfaces = ();
        )
    });

    let new_opt = (!has_new).then(|| {
        quote! {
            fn new() -> Self {
                ::std::default::Default::default()
            }
        }
    });

    let class_opt = (!has_class)
        .then(|| quote!(type Class = #crate_ident::subclass::basic::ClassStruct<Self>;));

    let instance_opt = (!has_instance)
        .then(|| quote!(type Instance = #crate_ident::subclass::basic::InstanceStruct<Self>;));

    let trait_path = match &trait_ {
        Some(path) => &path.1,
        None => abort_call_site!(WRONG_PLACE_MSG),
    };

    quote! {
        #(#attrs)*
        impl #generics #trait_path for #self_ty {
            #parent_type_opt
            #interfaces_opt
            #class_opt
            #instance_opt
            #new_opt
            #(#items)*
        }

        unsafe impl #crate_ident::subclass::types::ObjectSubclassType for #self_ty {
            fn type_data() -> ::std::ptr::NonNull<#crate_ident::subclass::TypeData> {
                static mut DATA: #crate_ident::subclass::TypeData =
                    #crate_ident::subclass::types::INIT_TYPE_DATA;
                unsafe { ::std::ptr::NonNull::new_unchecked(&mut DATA) }
            }

            fn type_() -> #crate_ident::Type {
                static ONCE: ::std::sync::Once = ::std::sync::Once::new();

                ONCE.call_once(|| {
                    #crate_ident::subclass::register_type::<Self>();
                });

                unsafe {
                    let data = Self::type_data();
                    let type_ = data.as_ref().type_();
                    assert!(type_.is_valid());

                    type_
                }
            }
        }

        #[doc(hidden)]
        impl #crate_ident::subclass::types::FromObject for #self_ty {
            type FromObjectType = <Self as #crate_ident::subclass::types::ObjectSubclass>::Type;
            fn from_object(obj: &Self::FromObjectType) -> &Self {
                <Self as #crate_ident::subclass::types::ObjectSubclassExt>::from_instance(obj)
            }
        }
    }
}
