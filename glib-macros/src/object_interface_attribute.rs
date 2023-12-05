// Take a look at the license at the top of the repository in the LICENSE file.

use proc_macro2::TokenStream;
use proc_macro_error::abort_call_site;
use quote::{quote, ToTokens};

pub const WRONG_EXPRESSION_MSG: &str =
    "This macro's attributes should be a sequence of assign expressions punctuated by comma";

pub const UNSUPPORTED_EXPRESSION_MSG: &str =
    "This macro's supported attributes are: `plugin_type = <subclass_of_glib::TypePlugin>, lazy_registration = true|false`";

pub const WRONG_PLACE_MSG: &str =
    "This macro should be used on `impl` block for `glib::ObjectInterface` trait";

pub fn impl_object_interface(input: &syn::ItemImpl) -> TokenStream {
    let crate_ident = crate::utils::crate_ident_new();
    let syn::ItemImpl { self_ty, .. } = &input;

    // registers the interface on first use (lazy registration).
    let register_interface = quote! {
        impl #self_ty {
            /// Registers the interface only once.
            #[inline]
            fn register_interface() -> #crate_ident::Type {
                static ONCE: ::std::sync::Once = ::std::sync::Once::new();
                static mut TYPE: #crate_ident::Type = #crate_ident::Type::INVALID;

                ONCE.call_once(|| unsafe {
                    TYPE = #crate_ident::subclass::register_interface::<Self>();
                });

                unsafe {
                    TYPE
                }
            }
        }
    };

    impl_object_interface_(register_interface, input)
}

pub fn impl_dynamic_object_interface(
    attrs: &syn::punctuated::Punctuated<syn::Expr, syn::Token![,]>,
    input: &syn::ItemImpl,
) -> TokenStream {
    let crate_ident = crate::utils::crate_ident_new();
    let syn::ItemImpl { self_ty, .. } = &input;

    let mut plugin_type_opt: Option<syn::Path> = None;
    let mut lazy_registration_opt: Option<bool> = None;

    for attr in attrs {
        match attr {
            // attribute must be one of supported assign expressions.
            syn::Expr::Assign(syn::ExprAssign { left, right, .. }) => {
                match (*left.to_owned(), *right.to_owned()) {
                    // `plugin_type = <subclass_of_TypePlugin>`
                    (
                        syn::Expr::Path(syn::ExprPath { path: path1, .. }),
                        syn::Expr::Path(syn::ExprPath { path: path2, .. }),
                    ) if path1.is_ident(&"plugin_type") => plugin_type_opt = Some(path2),
                    // `lazy_registration = true|false`
                    (
                        syn::Expr::Path(syn::ExprPath { path, .. }),
                        syn::Expr::Lit(syn::ExprLit {
                            lit: syn::Lit::Bool(syn::LitBool { value, .. }),
                            ..
                        }),
                    ) if path.is_ident(&"lazy_registration") => lazy_registration_opt = Some(value),
                    _ => abort_call_site!(UNSUPPORTED_EXPRESSION_MSG),
                };
            }
            _ => abort_call_site!(WRONG_EXPRESSION_MSG),
        };
    }

    let (plugin_ty, lazy_registration) = match (plugin_type_opt, lazy_registration_opt) {
        (Some(type_plugin), lazy_registration_opt) => (
            type_plugin.into_token_stream(),
            lazy_registration_opt.unwrap_or_default(),
        ),
        (None, lazy_registration_opt) => (
            quote!(#crate_ident::TypeModule),
            lazy_registration_opt.unwrap_or_default(),
        ),
    };

    // The following implementations follows the lifecycle of plugins and of dynamic types (see [`TypePluginExt`] and [`TypeModuleExt`]).
    // An object interface can be reregistered as a dynamic type.
    let register_interface = if lazy_registration {
        // registers the object interface as a dynamic type on the first use (lazy registration).
        // a weak reference on the plugin is stored and will be used later on the first use of the object interface.
        // this implementation relies on a static storage of a weak reference on the plugin and of the GLib type to know if the object interface has been registered.
        quote! {
            impl #self_ty {
                /// Returns a mutable reference to the registration status: a tuple of the weak reference on the plugin and of the GLib type.
                /// This is safe because the mutable reference guarantees that no other threads are concurrently accessing the data.
                #[inline]
                fn get_registration_status_ref_mut() -> &'static mut Option<(<#plugin_ty as #crate_ident::clone::Downgrade>::Weak, #crate_ident::Type)> {
                    static mut REGISTRATION_STATUS: ::std::sync::Mutex<Option<(<#plugin_ty as #crate_ident::clone::Downgrade>::Weak, #crate_ident::Type)>> = ::std::sync::Mutex::new(None);
                    unsafe { REGISTRATION_STATUS.get_mut().unwrap() }
                }

                /// Registers the object interface as a dynamic type within the plugin only once.
                /// Plugin must have been used at least once.
                /// Do nothing if plugin has never been used or if the object interface is already registered as a dynamic type.
                #[inline]
                fn register_interface() -> #crate_ident::Type {
                    let registration_status_ref_mut = Self::get_registration_status_ref_mut();
                    match registration_status_ref_mut {
                        // plugin has never been used, so the object interface cannot be registered as a dynamic type.
                        None => #crate_ident::Type::INVALID,
                        // plugin has been used and the object interface has not been registered yet, so registers it as a dynamic type.
                        Some((type_plugin, type_)) if !type_.is_valid() => {
                            *type_ = #crate_ident::subclass::register_dynamic_interface::<#plugin_ty, Self>(&(type_plugin.upgrade().unwrap()));
                            *type_
                        },
                        // plugin has been used and the object interface has already been registered as a dynamic type.
                        Some((_, type_)) => *type_
                    }
                }

                /// Depending on the plugin lifecycle state and on the registration status of the object interface:
                /// If plugin is used (and has loaded the implementation) for the first time, postpones the registration and stores a weak reference on the plugin.
                /// If plugin is reused (and has reloaded the implementation) and the object interface has been already registered as a dynamic type, reregisters it.
                /// An object interface can be reregistered several times as a dynamic type.
                /// If plugin is reused (and has reloaded the implementation) and the object interface has not been registered yet as a dynamic type, do nothing.
                #[inline]
                pub fn on_implementation_load(type_plugin: &#plugin_ty) -> bool {
                    let registration_status_ref_mut = Self::get_registration_status_ref_mut();
                    match registration_status_ref_mut {
                        // plugin has never been used (this is the first time), so postpones registration of the object interface as a dynamic type on the first use.
                        None => {
                            *registration_status_ref_mut = Some((#crate_ident::clone::Downgrade::downgrade(type_plugin), #crate_ident::Type::INVALID));
                            true
                        },
                        // plugin has been used at least one time and the object interface has been registered as a dynamic type at least one time, so re-registers it.
                        Some((_, type_)) if type_.is_valid() => {
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
                    let registration_status_ref_mut = Self::get_registration_status_ref_mut();
                    match registration_status_ref_mut {
                        // plugin has never been used, so unload implementation is unexpected.
                        None => false,
                        // plugin has been used at least one time and the object interface has been registered as a dynamic type at least one time.
                        Some((_, type_)) if type_.is_valid() => true,
                        // plugin has been used at least one time but the object interface has not been registered yet as a dynamic type, so cancels the postponed registration.
                        Some(_) => {
                            *registration_status_ref_mut = None;
                            true
                        }
                    }
                }
            }
        }
    } else {
        // registers immediately the object interface as a dynamic type.
        quote! {
            impl #self_ty {
                /// Returns a mutable reference to the GLib type.
                /// This is safe because the mutable reference guarantees that no other threads are concurrently accessing the atomic data.
                #[inline]
                fn get_type_mut() -> &'static mut #crate_ident::ffi::GType {
                    static mut TYPE: ::std::sync::atomic::AtomicUsize = ::std::sync::atomic::AtomicUsize::new(#crate_ident::gobject_ffi::G_TYPE_INVALID);
                    unsafe { TYPE.get_mut() }
                }

                /// Do nothing as the object interface has been registered on implementation load.
                #[inline]
                fn register_interface() -> #crate_ident::Type {
                    unsafe { <#crate_ident::Type as #crate_ident::translate::FromGlib<#crate_ident::ffi::GType>>::from_glib(*Self::get_type_mut()) }
                }

                /// Registers the object interface as a dynamic type within the plugin.
                /// The object interface can be registered several times as a dynamic type.
                #[inline]
                pub fn on_implementation_load(type_plugin: &#plugin_ty) -> bool {
                    let type_mut = Self::get_type_mut();
                    *type_mut = #crate_ident::translate::IntoGlib::into_glib(#crate_ident::subclass::register_dynamic_interface::<#plugin_ty, Self>(type_plugin));
                    *type_mut != #crate_ident::gobject_ffi::G_TYPE_INVALID
                }

                /// Do nothing as object interfaces registered as dynamic types are never unregistered.
                #[inline]
                pub fn on_implementation_unload(type_plugin_: &#plugin_ty) -> bool {
                    true
                }
            }
        }
    };

    impl_object_interface_(register_interface, input)
}

pub fn impl_object_interface_(
    register_interface: TokenStream,
    input: &syn::ItemImpl,
) -> TokenStream {
    let mut has_prerequisites = false;
    for item in &input.items {
        if let syn::ImplItem::Type(type_) = item {
            let name = type_.ident.to_string();
            if name == "Prerequisites" {
                has_prerequisites = true;
            }
        }
    }

    let syn::ItemImpl {
        attrs,
        generics,
        trait_,
        self_ty,
        unsafety,
        items,
        ..
    } = &input;

    let prerequisites_opt = if has_prerequisites {
        None
    } else {
        Some(quote!(
            type Prerequisites = ();
        ))
    };

    let crate_ident = crate::utils::crate_ident_new();

    let trait_path = match &trait_ {
        Some(path) => &path.1,
        None => abort_call_site!(WRONG_PLACE_MSG),
    };

    quote! {
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

        #register_interface
    }
}
