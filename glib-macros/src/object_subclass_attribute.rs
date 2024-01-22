// Take a look at the license at the top of the repository in the LICENSE file.

use heck::ToShoutySnakeCase;
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote, ToTokens};

use crate::utils::{parse_optional_nested_meta_items, NestedMetaItem};

pub const WRONG_PLACE_MSG: &str =
    "This macro should be used on `impl` block for `glib::ObjectSubclass` trait";

pub fn impl_object_subclass(input: &mut syn::ItemImpl) -> syn::Result<TokenStream> {
    let crate_ident = crate::utils::crate_ident_new();
    let syn::ItemImpl {
        attrs,
        generics,
        trait_,
        self_ty,
        items,
        ..
    } = input;

    let self_ty_as_ident = match &**self_ty {
        syn::Type::Path(syn::TypePath { path, .. }) => path.require_ident(),
        _ => Err(syn::Error::new(
            syn::spanned::Spanned::span(self_ty),
            "expected this path to be an identifier",
        )),
    }?;

    let mut plugin_type = NestedMetaItem::<syn::Path>::new("plugin_type").value_required();
    let mut lazy_registration =
        NestedMetaItem::<syn::LitBool>::new("lazy_registration").value_required();

    let found = parse_optional_nested_meta_items(
        &*attrs,
        "object_subclass_dynamic",
        &mut [&mut plugin_type, &mut lazy_registration],
    )?;

    let register_object_subclass = match found {
        None => register_object_subclass_as_static(&crate_ident, self_ty_as_ident),
        Some(_) => {
            // remove attribute 'object_subclass_dynamic' from the attribute list because it is not a real proc_macro_attribute
            attrs.retain(|attr| !attr.path().is_ident("object_subclass_dynamic"));
            let plugin_ty = plugin_type
                .value
                .map(|p| p.into_token_stream())
                .unwrap_or(quote!(#crate_ident::TypeModule));
            let lazy_registration = lazy_registration.value.map(|b| b.value).unwrap_or_default();
            register_object_subclass_as_dynamic(
                &crate_ident,
                self_ty_as_ident,
                plugin_ty,
                lazy_registration,
            )
        }
    };

    let mut has_new = false;
    let mut has_parent_type = false;
    let mut has_interfaces = false;
    let mut has_instance = false;
    let mut has_class = false;
    for item in items.iter() {
        match item {
            syn::ImplItem::Fn(method) => {
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
            #[inline]
            fn new() -> Self {
                ::std::default::Default::default()
            }
        }
    });

    let class_opt = (!has_class)
        .then(|| quote!(type Class = #crate_ident::subclass::basic::ClassStruct<Self>;));

    let instance_opt = (!has_instance)
        .then(|| quote!(type Instance = #crate_ident::subclass::basic::InstanceStruct<Self>;));

    let trait_path = &trait_
        .as_ref()
        .ok_or_else(|| syn::Error::new(Span::call_site(), WRONG_PLACE_MSG))?
        .1;

    Ok(quote! {
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
            #[inline]
            fn type_data() -> ::std::ptr::NonNull<#crate_ident::subclass::TypeData> {
                static mut DATA: #crate_ident::subclass::TypeData =
                    #crate_ident::subclass::types::INIT_TYPE_DATA;
                unsafe { ::std::ptr::NonNull::from(&mut DATA) }
            }

            #[inline]
            fn type_() -> #crate_ident::Type {
                Self::register_type();

                unsafe {
                    let data = Self::type_data();
                    let type_ = data.as_ref().type_();

                    type_
                }
            }
        }

        #register_object_subclass

        #[doc(hidden)]
        impl #crate_ident::subclass::types::FromObject for #self_ty {
            type FromObjectType = <Self as #crate_ident::subclass::types::ObjectSubclass>::Type;
            #[inline]
            fn from_object(obj: &Self::FromObjectType) -> &Self {
                <Self as #crate_ident::subclass::types::ObjectSubclassExt>::from_obj(obj)
            }
        }

        #[doc(hidden)]
        impl #crate_ident::clone::Downgrade for #self_ty {
            type Weak = #crate_ident::subclass::ObjectImplWeakRef<#self_ty>;

            #[inline]
            fn downgrade(&self) -> Self::Weak {
                let ref_counted = #crate_ident::subclass::prelude::ObjectSubclassExt::ref_counted(self);
                #crate_ident::clone::Downgrade::downgrade(&ref_counted)
            }
        }

        impl #self_ty {
            #[inline]
            pub fn downgrade(&self) -> <Self as #crate_ident::clone::Downgrade>::Weak {
                #crate_ident::clone::Downgrade::downgrade(self)
            }
        }

        #[doc(hidden)]
        impl ::std::borrow::ToOwned for #self_ty {
            type Owned = #crate_ident::subclass::ObjectImplRef<#self_ty>;

            #[inline]
            fn to_owned(&self) -> Self::Owned {
                #crate_ident::subclass::prelude::ObjectSubclassExt::ref_counted(self)
            }
        }

        #[doc(hidden)]
        impl ::std::borrow::Borrow<#self_ty> for #crate_ident::subclass::ObjectImplRef<#self_ty> {
            #[inline]
            fn borrow(&self) -> &#self_ty {
                self
            }
        }
    })
}

// Registers the object subclass as a static type.
fn register_object_subclass_as_static(
    crate_ident: &TokenStream,
    self_ty: &syn::Ident,
) -> TokenStream {
    // registers the object subclass on first use (lazy registration).
    quote! {
        impl #self_ty {
            /// Registers the type only once.
            #[inline]
            fn register_type() {
                static ONCE: ::std::sync::Once = ::std::sync::Once::new();

                ONCE.call_once(|| {
                    #crate_ident::subclass::register_type::<Self>();
                })
            }
        }
    }
}

// The following implementations follows the lifecycle of plugins and of dynamic types (see [`TypePluginExt`] and [`TypeModuleExt`]).
// An object subclass can be reregistered as a dynamic type.
fn register_object_subclass_as_dynamic(
    crate_ident: &TokenStream,
    self_ty: &syn::Ident,
    plugin_ty: TokenStream,
    lazy_registration: bool,
) -> TokenStream {
    // The following implementations follows the lifecycle of plugins and of dynamic types (see [`TypePluginExt`] and [`TypeModuleExt`]).
    // An object subclass can be reregistered as a dynamic type.
    if lazy_registration {
        // registers the object subclass as a dynamic type on the first use (lazy registration).
        // a weak reference on the plugin is stored and will be used later on the first use of the object subclass.
        // this implementation relies on a static storage of a weak reference on the plugin and of the GLib type to know if the object subclass has been registered.

        // the registration status type.
        let registration_status_type = format_ident!("{}RegistrationStatus", self_ty);
        // name of the static variable to store the registration status.
        let registration_status = format_ident!(
            "{}",
            registration_status_type.to_string().to_shouty_snake_case()
        );

        quote! {
            /// The registration status type: a tuple of the weak reference on the plugin and of the GLib type.
            struct #registration_status_type(<#plugin_ty as #crate_ident::clone::Downgrade>::Weak, #crate_ident::Type);
            unsafe impl Send for #registration_status_type {}

            /// The registration status protected by a mutex guarantees so that no other threads are concurrently accessing the data.
            static #registration_status: ::std::sync::Mutex<Option<#registration_status_type>> = ::std::sync::Mutex::new(None);

            impl #self_ty {
                /// Registers the object subclass as a dynamic type within the plugin only once.
                /// Plugin must have been used at least once.
                /// Do nothing if plugin has never been used or if the object subclass is already registered as a dynamic type.
                #[inline]
                fn register_type() {
                    let mut registration_status = #registration_status.lock().unwrap();
                    match ::std::ops::DerefMut::deref_mut(&mut registration_status) {
                        // plugin has never been used, so the object subclass cannot be registered as a dynamic type.
                        None => (),
                        // plugin has been used and the object subclass has not been registered yet, so registers it as a dynamic type.
                        Some(#registration_status_type(type_plugin, type_)) if !type_.is_valid() => {
                            *type_ = #crate_ident::subclass::register_dynamic_type::<#plugin_ty, Self>(&(type_plugin.upgrade().unwrap()));
                        },
                        // plugin has been used and the object subclass has already been registered as a dynamic type.
                        Some(_) => ()
                    }
                }

                /// Depending on the plugin lifecycle state and on the registration status of the object subclass:
                /// If plugin is used (and has loaded the implementation) for the first time, postpones the registration and stores a weak reference on the plugin.
                /// If plugin is reused (and has reloaded the implementation) and the object subclass has been already registered as a dynamic type, reregisters it.
                /// An object subclass can be reregistered several times as a dynamic type.
                /// If plugin is reused (and has reloaded the implementation) and the object subclass has not been registered yet as a dynamic type, do nothing.
                #[inline]
                pub fn on_implementation_load(type_plugin: &#plugin_ty) -> bool {
                    let mut registration_status = #registration_status.lock().unwrap();
                    match ::std::ops::DerefMut::deref_mut(&mut registration_status) {
                        // plugin has never been used (this is the first time), so postpones registration of the object subclass as a dynamic type on the first use.
                        None => {
                            *registration_status = Some(#registration_status_type(#crate_ident::clone::Downgrade::downgrade(type_plugin), #crate_ident::Type::INVALID));
                            true
                        },
                        // plugin has been used at least one time and the object subclass has been registered as a dynamic type at least one time, so re-registers it.
                        Some(#registration_status_type(_, type_)) if type_.is_valid() => {
                            *type_ = #crate_ident::subclass::register_dynamic_type::<#plugin_ty, Self>(type_plugin);
                            type_.is_valid()
                        },
                        // plugin has been used at least one time but the object subclass has not been registered yet as a dynamic type, so keeps postponed registration.
                        Some(_) => {
                            true
                        }
                    }
                }

                /// Depending on the plugin lifecycle state and on the registration status of the object subclass:
                /// If plugin has been used (or reused) but the object subclass has not been registered yet as a dynamic type, cancels the postponed registration by deleting the weak reference on the plugin.
                /// Else do nothing.
                #[inline]
                pub fn on_implementation_unload(type_plugin_: &#plugin_ty) -> bool {
                    let mut registration_status = #registration_status.lock().unwrap();
                    match ::std::ops::DerefMut::deref_mut(&mut registration_status) {
                        // plugin has never been used, so unload implementation is unexpected.
                        None => false,
                        // plugin has been used at least one time and the object subclass has been registered as a dynamic type at least one time.
                        Some(#registration_status_type(_, type_)) if type_.is_valid() => true,
                        // plugin has been used at least one time but the object subclass has not been registered yet as a dynamic type, so cancels the postponed registration.
                        Some(_) => {
                            *registration_status = None;
                            true
                        }
                    }
                }
            }
        }
    } else {
        // registers immediately the object subclass as a dynamic type.

        quote! {
            impl #self_ty {
                /// Do nothing as the object subclass has been registered on implementation load.
                #[inline]
                fn register_type() { }

                /// Registers the object subclass as a dynamic type within the plugin.
                /// The object subclass can be registered several times as a dynamic type.
                #[inline]
                pub fn on_implementation_load(type_plugin: &#plugin_ty) -> bool {
                    let type_ = #crate_ident::subclass::register_dynamic_type::<#plugin_ty, Self>(type_plugin);
                    type_ != #crate_ident::Type::INVALID
                }

                /// Do nothing as object subclasses registered as dynamic types are never unregistered.
                #[inline]
                pub fn on_implementation_unload(type_plugin_: &#plugin_ty) -> bool {
                    true
                }
            }
        }
    }
}
