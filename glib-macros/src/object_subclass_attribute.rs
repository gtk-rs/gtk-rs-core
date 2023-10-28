// Take a look at the license at the top of the repository in the LICENSE file.

use proc_macro2::TokenStream;
use proc_macro_error::abort_call_site;
use quote::{quote, ToTokens};

pub const WRONG_EXPRESSION_MSG: &str =
    "This macro's attributes should be a sequence of assign expressions punctuated by comma";

pub const UNSUPPORTED_EXPRESSION_MSG: &str =
    "This macro's supported attributes are: `plugin_type = <subclass_of_glib::TypePlugin>, lazy_registration = true|false`";

pub const WRONG_PLACE_MSG: &str =
    "This macro should be used on `impl` block for `glib::ObjectSubclass` trait";

pub fn impl_object_subclass(input: &syn::ItemImpl) -> TokenStream {
    let crate_ident = crate::utils::crate_ident_new();
    let syn::ItemImpl { self_ty, .. } = &input;

    // registers the type on first use (lazy registration).
    let register_type = quote! {
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
    };

    impl_object_subclass_(register_type, input)
}

pub fn impl_dynamic_object_subclass(
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

    // The following implementations follows the lifecycle of plugins and of dynamic types (see [`TypePluginExt::unuse`]).
    // An object subclass can be reregistered as a dynamic type (see [`TypePluginExt::register_type`]).
    let register_type = if lazy_registration {
        // registers the object subclass as a dynamic type on the first use (lazy registration).
        // a weak reference on the plugin is stored and will be used later on the first use of the object subclass.
        // this implementation relies on a static storage of a weak reference on the plugin and of the glib type to know if the object subclass has been registered.
        quote! {
            impl #self_ty {
                /// Returns a mutable reference to the registration status: a tuple of the weak reference on the plugin and of the glib type.
                /// This is safe because the mutable reference guarantees that no other threads are concurrently accessing the data.
                #[inline]
                fn get_registration_status_ref_mut() -> &'static mut Option<(<#plugin_ty as #crate_ident::clone::Downgrade>::Weak, #crate_ident::Type)> {
                    static mut REGISTRATION_STATUS: ::std::sync::Mutex<Option<(<#plugin_ty as #crate_ident::clone::Downgrade>::Weak, #crate_ident::Type)>> = ::std::sync::Mutex::new(None);
                    unsafe { REGISTRATION_STATUS.get_mut().unwrap() }
                }

                /// Registers the object subclass as a dynamic type within the plugin only once.
                /// Plugin must have been used at least once.
                /// Do nothing if plugin has never been used or if the object subclass is already registered as a dynamic type.
                #[inline]
                fn register_type() {
                    let registration_status_ref_mut = Self::get_registration_status_ref_mut();
                    match registration_status_ref_mut {
                        // plugin has never been used, so the object subclass cannot be registered as a dynamic type.
                        None => (),
                        // plugin has been used and the object subclass has not been registered yet, so registers it as a dynamic type.
                        Some((type_plugin, type_)) if !type_.is_valid() => {
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
                    let registration_status_ref_mut = Self::get_registration_status_ref_mut();
                    match registration_status_ref_mut {
                        // plugin has never been used (this is the first time), so postpones registration of the object subclass as a dynamic type on the first use.
                        None => {
                            *registration_status_ref_mut = Some((#crate_ident::clone::Downgrade::downgrade(type_plugin), #crate_ident::Type::INVALID));
                            true
                        },
                        // plugin has been used at least one time and the object subclass has been registered as a dynamic type at least one time, so re-registers it.
                        Some((_, type_)) if type_.is_valid() => {
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
                    let registration_status_ref_mut = Self::get_registration_status_ref_mut();
                    match registration_status_ref_mut {
                        // plugin has never been used, so unload implementation is unexpected.
                        None => false,
                        // plugin has been used at least one time and the object subclass has been registered as a dynamic type at least one time.
                        Some((_, type_)) if type_.is_valid() => true,
                        // plugin has been used at least one time but the object subclass has not been registered yet as a dynamic type, so cancels the postponed registration.
                        Some(_) => {
                            *registration_status_ref_mut = None;
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
    };

    impl_object_subclass_(register_type, input)
}

fn impl_object_subclass_(register_type: TokenStream, input: &syn::ItemImpl) -> TokenStream {
    let mut has_new = false;
    let mut has_parent_type = false;
    let mut has_interfaces = false;
    let mut has_instance = false;
    let mut has_class = false;
    for item in &input.items {
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

        #register_type

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
    }
}
