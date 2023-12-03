// Take a look at the license at the top of the repository in the LICENSE file.

use heck::{ToKebabCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use proc_macro_error::abort_call_site;
use quote::{quote, quote_spanned, ToTokens};
use syn::{
    parse_quote, punctuated::Punctuated, spanned::Spanned, token::Comma, Data, ExprArray, Ident,
    Variant,
};

use crate::utils::{crate_ident_new, gen_enum_from_glib, parse_nested_meta_items, NestedMetaItem};

// generates glib::gobject_ffi::GEnumValue structs mapping the enum such as:
//     glib::gobject_ffi::GEnumValue {
//         value: Animal::Goat as i32,
//         value_name: "Goat\0" as *const _ as *const _,
//         value_nick: "goat\0" as *const _ as *const _,
//     },
fn gen_enum_values(
    enum_name: &Ident,
    enum_variants: &Punctuated<Variant, Comma>,
) -> (TokenStream, usize) {
    let crate_ident = crate_ident_new();

    // starts at one as GEnumValue array is null-terminated.
    let mut n = 1;
    let recurse = enum_variants.iter().map(|v| {
        let name = &v.ident;
        let mut value_name = name.to_string().to_upper_camel_case();
        let mut value_nick = name.to_string().to_kebab_case();

        let mut name_attr = NestedMetaItem::<syn::LitStr>::new("name").value_required();
        let mut nick = NestedMetaItem::<syn::LitStr>::new("nick").value_required();

        let found =
            parse_nested_meta_items(&v.attrs, "enum_value", &mut [&mut name_attr, &mut nick]);
        if let Err(e) = found {
            return e.to_compile_error();
        }

        value_name = name_attr.value.map(|s| s.value()).unwrap_or(value_name);
        value_nick = nick.value.map(|s| s.value()).unwrap_or(value_nick);

        let value_name = format!("{value_name}\0");
        let value_nick = format!("{value_nick}\0");

        n += 1;
        // generates a glib::gobject_ffi::GEnumValue.
        quote_spanned! {v.span()=>
            #crate_ident::gobject_ffi::GEnumValue {
                value: #enum_name::#name as i32,
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

pub fn impl_enum(input: &syn::DeriveInput) -> TokenStream {
    let name = &input.ident;

    let enum_variants = match input.data {
        Data::Enum(ref e) => &e.variants,
        _ => abort_call_site!("#[derive(glib::Enum)] only supports enums"),
    };

    let mut gtype_name = NestedMetaItem::<syn::LitStr>::new("name")
        .required()
        .value_required();
    let found = parse_nested_meta_items(&input.attrs, "enum_type", &mut [&mut gtype_name]);

    match found {
        Ok(None) => {
            abort_call_site!("#[derive(glib::Enum)] requires #[enum_type(name = \"EnumTypeName\")]")
        }
        Err(e) => return e.to_compile_error(),
        Ok(attr) => attr,
    };
    let gtype_name = gtype_name.value.unwrap();
    let from_glib = gen_enum_from_glib(name, enum_variants);
    let (enum_values, nb_enum_values) = gen_enum_values(name, enum_variants);

    let crate_ident = crate_ident_new();

    // registers the enum on first use (lazy registration).
    let register_enum = quote! {
        impl #name {
            /// Registers the enum only once.
            #[inline]
            fn register_enum() -> #crate_ident::Type {
                static ONCE: ::std::sync::Once = ::std::sync::Once::new();
                static mut TYPE: #crate_ident::Type = #crate_ident::Type::INVALID;

                ONCE.call_once(|| {
                    static mut VALUES: [#crate_ident::gobject_ffi::GEnumValue; #nb_enum_values] = [
                        #enum_values
                        #crate_ident::gobject_ffi::GEnumValue {
                            value: 0,
                            value_name: ::std::ptr::null(),
                            value_nick: ::std::ptr::null(),
                        },
                    ];
                    let name = ::std::ffi::CString::new(#gtype_name).expect("CString::new failed");
                    unsafe {
                        let type_ = #crate_ident::gobject_ffi::g_enum_register_static(name.as_ptr(), VALUES.as_ptr());
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
    };

    impl_enum_(name, from_glib, register_enum)
}

pub fn impl_dynamic_enum(input: &syn::DeriveInput) -> TokenStream {
    let name = &input.ident;

    let enum_variants = match input.data {
        Data::Enum(ref e) => &e.variants,
        _ => abort_call_site!("#[derive(glib::Enum)] only supports enums"),
    };

    let mut gtype_name = NestedMetaItem::<syn::LitStr>::new("name")
        .required()
        .value_required();
    let mut plugin_type = NestedMetaItem::<syn::Path>::new("plugin_type").value_required();
    let mut lazy_registration =
        NestedMetaItem::<syn::LitBool>::new("lazy_registration").value_required();
    let found = parse_nested_meta_items(
        &input.attrs,
        "enum_type",
        &mut [&mut gtype_name, &mut plugin_type, &mut lazy_registration],
    );

    match found {
        Ok(None) => {
            abort_call_site!("#[derive(glib::DynamicEnum)] requires #[enum_type(name = \"EnumTypeName\"[, plugin_type =  <subclass_of_glib::TypePlugin>][, lazy_registration = true|false])]")
        }
        Err(e) => return e.to_compile_error(),
        Ok(attr) => attr,
    };

    let crate_ident = crate_ident_new();

    let gtype_name = gtype_name.value.unwrap();
    let plugin_ty = plugin_type
        .value
        .map(|p| p.into_token_stream())
        .unwrap_or(quote!(#crate_ident::TypeModule));
    let lazy_registration = lazy_registration.value.map(|b| b.value).unwrap_or_default();

    let from_glib = gen_enum_from_glib(name, enum_variants);
    let (g_enum_values, nb_enum_values) = gen_enum_values(name, enum_variants);

    // Wrap each GEnumValue to EnumValue
    let g_enum_values_expr: ExprArray = parse_quote! { [#g_enum_values] };
    let enum_values_iter = g_enum_values_expr.elems.iter().map(|v| {
        quote_spanned! {v.span()=>
            #crate_ident::EnumValue::new(#v),
        }
    });

    let enum_values = quote! {
        [#crate_ident::EnumValue; #nb_enum_values] = [
            #(#enum_values_iter)*
            #crate_ident::EnumValue::new(#crate_ident::gobject_ffi::GEnumValue {
                value: 0,
                value_name: ::std::ptr::null(),
                value_nick: ::std::ptr::null(),
            }),
        ]
    };

    // The following implementations follows the lifecycle of plugins and of dynamic types (see [`TypePluginExt`] and [`TypeModuleExt`]).
    // An enum can be reregistered as a dynamic type.
    let register_enum_impl = if lazy_registration {
        // registers the enum as a dynamic type on the first use (lazy registration).
        // a weak reference on the plugin is stored and will be used later on the first use of the enum.
        // this implementation relies on a static storage of a weak reference on the plugin and of the glib type to know if the enum has been registered.
        quote! {
            impl #name {
                /// Returns a mutable reference to the registration status: a tuple of the weak reference on the plugin and of the glib type.
                /// This is safe because the mutable reference guarantees that no other threads are concurrently accessing the data.
                #[inline]
                fn get_registration_status_ref_mut() -> &'static mut Option<(<#plugin_ty as #crate_ident::clone::Downgrade>::Weak, #crate_ident::Type)> {
                    static mut REGISTRATION_STATUS: ::std::sync::Mutex<Option<(<#plugin_ty as #crate_ident::clone::Downgrade>::Weak, #crate_ident::Type)>> = ::std::sync::Mutex::new(None);
                    unsafe { REGISTRATION_STATUS.get_mut().unwrap() }
                }

                /// Registers the enum as a dynamic type within the plugin only once.
                /// Plugin must have been used at least once.
                /// Do nothing if plugin has never been used or if the enum is already registered as a dynamic type.
                #[inline]
                fn register_enum() -> #crate_ident::Type {
                    let registration_status_ref_mut = Self::get_registration_status_ref_mut();
                    match registration_status_ref_mut {
                        // plugin has never been used, so the enum cannot be registered as a dynamic type.
                        None => #crate_ident::Type::INVALID,
                        // plugin has been used and the enum has not been registered yet, so registers it as a dynamic type.
                        Some((type_plugin, type_)) if !type_.is_valid() => {
                            static mut VALUES: #enum_values;
                            *type_ = <#plugin_ty as glib::prelude::DynamicObjectRegisterExt>::register_dynamic_enum(type_plugin.upgrade().unwrap().as_ref(), #gtype_name, unsafe { &VALUES } );
                            *type_
                        },
                        // plugin has been used and the enum has already been registered as a dynamic type.
                        Some((_, type_)) => *type_
                    }
                }

                /// Depending on the plugin lifecycle state and on the registration status of the enum:
                /// If plugin is used (and has loaded the implementation) for the first time, postpones the registration and stores a weak reference on the plugin.
                /// If plugin is reused (and has reloaded the implementation) and the enum has been already registered as a dynamic type, reregisters it.
                /// An enum can be reregistered several times as a dynamic type.
                /// If plugin is reused (and has reloaded the implementation) and the enum has not been registered yet as a dynamic type, do nothing.
                #[inline]
                pub fn on_implementation_load(type_plugin: &#plugin_ty) -> bool {
                    let registration_status_ref_mut = Self::get_registration_status_ref_mut();
                    match registration_status_ref_mut {
                        // plugin has never been used (this is the first time), so postpones registration of the enum as a dynamic type on the first use.
                        None => {
                            *registration_status_ref_mut = Some((#crate_ident::clone::Downgrade::downgrade(type_plugin), #crate_ident::Type::INVALID));
                            true
                        },
                        // plugin has been used at least one time and the enum has been registered as a dynamic type at least one time, so re-registers it.
                        Some((_, type_)) if type_.is_valid() => {
                            static mut VALUES: #enum_values;
                            *type_ = <#plugin_ty as glib::prelude::DynamicObjectRegisterExt>::register_dynamic_enum(type_plugin, #gtype_name, unsafe { &VALUES } );
                            type_.is_valid()
                        },
                        // plugin has been used at least one time but the enum has not been registered yet as a dynamic type, so keeps postponed registration.
                        Some(_) => {
                            true
                        }
                    }
                }

                /// Depending on the plugin lifecycle state and on the registration status of the enum:
                /// If plugin has been used (or reused) but the enum has not been registered yet as a dynamic type, cancels the postponed registration by deleting the weak reference on the plugin.
                /// Else do nothing.
                #[inline]
                pub fn on_implementation_unload(type_plugin_: &#plugin_ty) -> bool {
                    let registration_status_ref_mut = Self::get_registration_status_ref_mut();
                    match registration_status_ref_mut {
                        // plugin has never been used, so unload implementation is unexpected.
                        None => false,
                        // plugin has been used at least one time and the enum has been registered as a dynamic type at least one time.
                        Some((_, type_)) if type_.is_valid() => true,
                        // plugin has been used at least one time but the enum has not been registered yet as a dynamic type, so cancels the postponed registration.
                        Some(_) => {
                            *registration_status_ref_mut = None;
                            true
                        }
                    }
                }
            }
        }
    } else {
        // registers immediately the enum as a dynamic type.
        quote! {
            impl #name {
                /// Returns a mutable reference to the glib type.
                /// This is safe because the mutable reference guarantees that no other threads are concurrently accessing the atomic data.
                #[inline]
                fn get_type_mut() -> &'static mut #crate_ident::ffi::GType {
                    static mut TYPE: ::std::sync::atomic::AtomicUsize = ::std::sync::atomic::AtomicUsize::new(#crate_ident::gobject_ffi::G_TYPE_INVALID);
                    unsafe { TYPE.get_mut() }
                }

                /// Do nothing as the enum has been registered on implementation load.
                #[inline]
                fn register_enum() -> #crate_ident::Type {
                    unsafe { <#crate_ident::Type as #crate_ident::translate::FromGlib<#crate_ident::ffi::GType>>::from_glib(*Self::get_type_mut()) }
                }

                /// Registers the enum as a dynamic type within the plugin.
                /// The enum can be registered several times as a dynamic type.
                #[inline]
                pub fn on_implementation_load(type_plugin: &#plugin_ty) -> bool {
                    let type_mut = Self::get_type_mut();
                    static mut VALUES: #enum_values;
                    *type_mut = #crate_ident::translate::IntoGlib::into_glib(<#plugin_ty as glib::prelude::DynamicObjectRegisterExt>::register_dynamic_enum(type_plugin, #gtype_name, unsafe { &VALUES } ));
                    *type_mut != #crate_ident::gobject_ffi::G_TYPE_INVALID
                }

                /// Do nothing as enums registered as dynamic types are never unregistered.
                #[inline]
                pub fn on_implementation_unload(type_plugin_: &#plugin_ty) -> bool {
                    true
                }
            }
        }
    };

    impl_enum_(name, from_glib, register_enum_impl)
}

pub fn impl_enum_(
    name: &syn::Ident,
    from_glib: TokenStream,
    register_enum: TokenStream,
) -> TokenStream {
    let crate_ident = crate_ident_new();

    quote! {
        impl #crate_ident::translate::IntoGlib for #name {
            type GlibType = i32;

            #[inline]
            fn into_glib(self) -> i32 {
                self as i32
            }
        }

        impl #crate_ident::translate::TryFromGlib<i32> for #name {
            type Error = i32;

            #[inline]
            unsafe fn try_from_glib(value: i32) -> ::core::result::Result<Self, i32> {
                let from_glib = || {
                    #from_glib
                };

                from_glib().ok_or(value)
            }
        }

        impl #crate_ident::translate::FromGlib<i32> for #name {
            #[inline]
            unsafe fn from_glib(value: i32) -> Self {
                use #crate_ident::translate::TryFromGlib;

                Self::try_from_glib(value).unwrap()
            }
        }

        impl #crate_ident::value::ValueType for #name {
            type Type = Self;
        }

        unsafe impl<'a> #crate_ident::value::FromValue<'a> for #name {
            type Checker = #crate_ident::value::GenericValueTypeChecker<Self>;

            #[inline]
            unsafe fn from_value(value: &'a #crate_ident::value::Value) -> Self {
                #crate_ident::translate::from_glib(#crate_ident::gobject_ffi::g_value_get_enum(
                    #crate_ident::translate::ToGlibPtr::to_glib_none(value).0
                ))
            }
        }

        impl #crate_ident::value::ToValue for #name {
            #[inline]
            fn to_value(&self) -> #crate_ident::value::Value {
                let mut value = #crate_ident::value::Value::for_value_type::<Self>();
                unsafe {
                    #crate_ident::gobject_ffi::g_value_set_enum(
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

        impl ::std::convert::From<#name> for #crate_ident::Value {
            #[inline]
            fn from(v: #name) -> Self {
                #crate_ident::value::ToValue::to_value(&v)
            }
        }

        impl #crate_ident::StaticType for #name {
            #[inline]
            fn static_type() -> #crate_ident::Type {
                Self::register_enum()
            }
        }

        #register_enum

        impl #crate_ident::HasParamSpec for #name {
            type ParamSpec = #crate_ident::ParamSpecEnum;
            type SetValue = Self;
            type BuilderFn = fn(&::core::primitive::str, Self) -> #crate_ident::ParamSpecEnumBuilder<Self>;

            fn param_spec_builder() -> Self::BuilderFn {
                |name, default_value| Self::ParamSpec::builder_with_default(name, default_value)
            }
        }
    }
}
