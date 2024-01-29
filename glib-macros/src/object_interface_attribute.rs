// Take a look at the license at the top of the repository in the LICENSE file.

use heck::ToShoutySnakeCase;
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote, ToTokens};

use crate::utils::{parse_optional_nested_meta_items, NestedMetaItem};

pub const WRONG_PLACE_MSG: &str =
    "This macro should be used on `impl` block for `glib::ObjectInterface` trait";

pub fn impl_object_interface(input: &mut syn::ItemImpl) -> syn::Result<TokenStream> {
    let crate_ident = crate::utils::crate_ident_new();
    let syn::ItemImpl {
        attrs,
        generics,
        trait_,
        self_ty,
        unsafety,
        items,
        ..
    } = input;

    let self_ty_as_ident = match &**self_ty {
        syn::Type::Path(syn::TypePath { path, .. }) => path.require_ident(),
        _ => Err(syn::Error::new_spanned(
            &self_ty,
            "expected this path to be an identifier",
        )),
    }?;

    let mut plugin_type = NestedMetaItem::<syn::Path>::new("plugin_type").value_required();
    let mut lazy_registration =
        NestedMetaItem::<syn::LitBool>::new("lazy_registration").value_required();

    let found = parse_optional_nested_meta_items(
        &*attrs,
        "object_interface_dynamic",
        &mut [&mut plugin_type, &mut lazy_registration],
    )?;

    let register_object_interface = match found {
        None => register_object_interface_as_static(&crate_ident, self_ty_as_ident),
        Some(_) => {
            // remove attribute 'object_interface_dynamic' from the attribute list because it is not a real proc_macro_attribute
            attrs.retain(|attr| !attr.path().is_ident("object_interface_dynamic"));
            let plugin_ty = plugin_type
                .value
                .map(|p| p.into_token_stream())
                .unwrap_or(quote!(#crate_ident::TypeModule));
            let lazy_registration = lazy_registration.value.map(|b| b.value).unwrap_or_default();
            register_object_interface_as_dynamic(
                &crate_ident,
                self_ty_as_ident,
                plugin_ty,
                lazy_registration,
            )
        }
    };

    let mut has_prerequisites = false;
    for item in items.iter() {
        if let syn::ImplItem::Type(type_) = item {
            let name = type_.ident.to_string();
            if name == "Prerequisites" {
                has_prerequisites = true;
            }
        }
    }

    let prerequisites_opt = if has_prerequisites {
        None
    } else {
        Some(quote!(
            type Prerequisites = ();
        ))
    };

    let trait_path = &trait_
        .as_ref()
        .ok_or_else(|| syn::Error::new(Span::call_site(), WRONG_PLACE_MSG))?
        .1;

    Ok(quote! {
        #(#attrs)*
        #unsafety impl #generics #trait_path for #self_ty {
            #prerequisites_opt
            #(#items)*
        }

        unsafe impl #crate_ident::subclass::interface::ObjectInterfaceType for #self_ty {
            #[inline]
            fn type_() -> #crate_ident::Type {
                Self::register_interface()
            }
        }

        #register_object_interface
    })
}

// Registers the object interface as a static type.
fn register_object_interface_as_static(
    crate_ident: &TokenStream,
    self_ty: &syn::Ident,
) -> TokenStream {
    // registers the interface on first use (lazy registration).
    quote! {
        impl #self_ty {
            /// Registers the interface only once.
            #[inline]
            fn register_interface() -> #crate_ident::Type {
                static TYPE: ::std::sync::OnceLock<#crate_ident::Type> = ::std::sync::OnceLock::new();
                *TYPE.get_or_init(|| unsafe {
                    #crate_ident::subclass::register_interface::<Self>()
                })
            }
        }
    }
}

// The following implementations follows the lifecycle of plugins and of dynamic types (see [`TypePluginExt`] and [`TypeModuleExt`]).
// An object interface can be reregistered as a dynamic type.
fn register_object_interface_as_dynamic(
    crate_ident: &TokenStream,
    self_ty: &syn::Ident,
    plugin_ty: TokenStream,
    lazy_registration: bool,
) -> TokenStream {
    // The following implementations follows the lifecycle of plugins and of dynamic types (see [`TypePluginExt`] and [`TypeModuleExt`]).
    // An object interface can be reregistered as a dynamic type.
    if lazy_registration {
        // registers the object interface as a dynamic type on the first use (lazy registration).
        // a weak reference on the plugin is stored and will be used later on the first use of the object interface.
        // this implementation relies on a static storage of a weak reference on the plugin and of the GLib type to know if the object interface has been registered.

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
                /// Registers the object interface as a dynamic type within the plugin only once.
                /// Plugin must have been used at least once.
                /// Do nothing if plugin has never been used or if the object interface is already registered as a dynamic type.
                #[inline]
                fn register_interface() -> #crate_ident::Type {
                    let mut registration_status = #registration_status.lock().unwrap();
                    match ::std::ops::DerefMut::deref_mut(&mut registration_status) {
                        // plugin has never been used, so the object interface cannot be registered as a dynamic type.
                        None => #crate_ident::Type::INVALID,
                        // plugin has been used and the object interface has not been registered yet, so registers it as a dynamic type.
                        Some(#registration_status_type(type_plugin, type_)) if !type_.is_valid() => {
                            *type_ = #crate_ident::subclass::register_dynamic_interface::<#plugin_ty, Self>(&(type_plugin.upgrade().unwrap()));
                            *type_
                        },
                        // plugin has been used and the object interface has already been registered as a dynamic type.
                        Some(#registration_status_type(_, type_)) => *type_
                    }
                }

                /// Depending on the plugin lifecycle state and on the registration status of the object interface:
                /// If plugin is used (and has loaded the implementation) for the first time, postpones the registration and stores a weak reference on the plugin.
                /// If plugin is reused (and has reloaded the implementation) and the object interface has been already registered as a dynamic type, reregisters it.
                /// An object interface can be reregistered several times as a dynamic type.
                /// If plugin is reused (and has reloaded the implementation) and the object interface has not been registered yet as a dynamic type, do nothing.
                #[inline]
                pub fn on_implementation_load(type_plugin: &#plugin_ty) -> bool {
                    let mut registration_status = #registration_status.lock().unwrap();
                    match ::std::ops::DerefMut::deref_mut(&mut registration_status) {
                        // plugin has never been used (this is the first time), so postpones registration of the object interface as a dynamic type on the first use.
                        None => {
                            *registration_status = Some(#registration_status_type(#crate_ident::clone::Downgrade::downgrade(type_plugin), #crate_ident::Type::INVALID));
                            true
                        },
                        // plugin has been used at least one time and the object interface has been registered as a dynamic type at least one time, so re-registers it.
                        Some(#registration_status_type(_, type_)) if type_.is_valid() => {
                            *type_ = #crate_ident::subclass::register_dynamic_interface::<#plugin_ty, Self>(type_plugin);
                            type_.is_valid()
                        },
                        // plugin has been used at least one time but the object interface has not been registered yet as a dynamic type, so keeps postponed registration.
                        Some(_) => {
                            true
                        }
                    }
                }

                /// Depending on the plugin lifecycle state and on the registration status of the object interface:
                /// If plugin has been used (or reused) but the object interface has not been registered yet as a dynamic type, cancels the postponed registration by deleting the weak reference on the plugin.
                /// Else do nothing.
                #[inline]
                pub fn on_implementation_unload(type_plugin_: &#plugin_ty) -> bool {
                    let mut registration_status = #registration_status.lock().unwrap();
                    match ::std::ops::DerefMut::deref_mut(&mut registration_status) {
                        // plugin has never been used, so unload implementation is unexpected.
                        None => false,
                        // plugin has been used at least one time and the object interface has been registered as a dynamic type at least one time.
                        Some(#registration_status_type(_, type_)) if type_.is_valid() => true,
                        // plugin has been used at least one time but the object interface has not been registered yet as a dynamic type, so cancels the postponed registration.
                        Some(_) => {
                            *registration_status = None;
                            true
                        }
                    }
                }
            }
        }
    } else {
        // registers immediately the object interface as a dynamic type.

        // name of the static variable to store the GLib type.
        let gtype_status = format_ident!("{}_G_TYPE", self_ty.to_string().to_shouty_snake_case());

        quote! {
            /// The GLib type which can be safely shared between threads.
            static #gtype_status: ::std::sync::atomic::AtomicUsize = ::std::sync::atomic::AtomicUsize::new(#crate_ident::gobject_ffi::G_TYPE_INVALID);

            impl #self_ty {
                /// Do nothing as the object interface has been registered on implementation load.
                #[inline]
                fn register_interface() -> #crate_ident::Type {
                    let gtype = #gtype_status.load(::std::sync::atomic::Ordering::Acquire);
                    unsafe { <#crate_ident::Type as #crate_ident::translate::FromGlib<#crate_ident::ffi::GType>>::from_glib(gtype) }
                }

                /// Registers the object interface as a dynamic type within the plugin.
                /// The object interface can be registered several times as a dynamic type.
                #[inline]
                pub fn on_implementation_load(type_plugin: &#plugin_ty) -> bool {
                    let gtype = #crate_ident::translate::IntoGlib::into_glib(#crate_ident::subclass::register_dynamic_interface::<#plugin_ty, Self>(type_plugin));
                    #gtype_status.store(gtype, ::std::sync::atomic::Ordering::Release);
                    gtype != #crate_ident::gobject_ffi::G_TYPE_INVALID
                }

                /// Do nothing as object interfaces registered as dynamic types are never unregistered.
                #[inline]
                pub fn on_implementation_unload(type_plugin_: &#plugin_ty) -> bool {
                    true
                }
            }
        }
    }
}
