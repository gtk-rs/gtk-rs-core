// Take a look at the license at the top of the repository in the LICENSE file.

use std::borrow::Cow;

use crate::dbus_interface::attributes::{DBusInterfaceAttribute, DBusPropertyAccess};
use crate::dbus_interface::parse::{DBusItem, DBusMethod, DBusMethodArg, DBusProperty};
use crate::utils::ident_name;
use heck::{ToKebabCase as _, ToPascalCase as _, ToSnakeCase as _};
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{Ident, ItemTrait, LitStr, Path, Type, Visibility};

pub(crate) fn emit_interface(
    input: &ItemTrait,
    items: &[DBusItem],
    attr: &DBusInterfaceAttribute,
    gio: &Path,
) -> TokenStream {
    let mod_prefix = Ident::new(&input.ident.to_string().to_snake_case(), input.ident.span());
    let ctx = Context {
        gio: gio.clone(),
        vis: input.vis.clone(),
        interface_ident: input.ident.clone(),
        ext_ident: format_ident!("{}Ext", &input.ident),
        impl_ident: format_ident!("{}Impl", &input.ident),
        impl_ext_ident: format_ident!("{}ImplExt", &input.ident),
        ffi_mod: format_ident!("{mod_prefix}_ffi"),
        iface_mod: format_ident!("{mod_prefix}_iface"),
        method_calls_mod: format_ident!("{mod_prefix}_method_calls"),
        glib_type_name: attr.type_name.clone(),
    };

    let methods: Vec<_> = items
        .iter()
        .filter_map(|item| item.method())
        .cloned()
        .collect();
    let properties: Vec<_> = items
        .iter()
        .filter_map(|item| item.property())
        .cloned()
        .collect();
    let errors = items
        .iter()
        .filter_map(|item| item.error())
        .map(|(_item, error)| error.to_compile_error());

    let ffi_mod = emit_ffi_types(&methods, &ctx);
    let iface_mod = emit_iface_types(&properties, &ctx);
    let wrapper = emit_wrapper(&ctx);
    let ext_trait = emit_ext_trait(&methods, &ctx);
    let impl_trait = emit_impl_trait(&methods, &ctx);
    let is_implementable_impl = emit_is_implementable_impl(&methods, &ctx);
    let method_calls = emit_method_calls(&methods, &ctx);

    quote! {
        #(#errors)*

        #ffi_mod
        #iface_mod
        #wrapper
        #ext_trait
        #impl_trait
        #is_implementable_impl
        #method_calls

        // #vis mod #mod_name {
        //     use #gio::glib::prelude::*;
        //     use #gio::glib::subclass::prelude::*;

        //     /// The `PurrableImplExt` trait contains non-overridable methods for subclasses to use.
        //     ///
        //     /// These are supposed to be called only from inside implementations of `Pet` subclasses.
        //     pub trait #impl_ext_ident: #impl_ident {
        //         // fn parent_is_purring(&self) -> bool {
        //         //     let data = Self::type_data();
        //         //     let parent_iface =
        //         //         unsafe { &*(data.as_ref().parent_interface::<Purrable>() as *const ffi::Interface) };
        //         //     let is_purring = parent_iface.is_purring;

        //         //     unsafe { is_purring(self.obj().unsafe_cast_ref()) }
        //         // }
        //     }

        //     /// The `PurrableImplExt` trait is implemented for all classes that implement [`Purrable`].
        //     impl<T: #impl_ident> #impl_ext_ident for T {}
    }
}

struct Context {
    gio: Path,
    vis: Visibility,
    interface_ident: Ident,
    ext_ident: Ident,
    impl_ident: Ident,
    impl_ext_ident: Ident,
    ffi_mod: Ident,
    iface_mod: Ident,
    method_calls_mod: Ident,
    glib_type_name: Option<LitStr>,
}

/// Emits the ffi interface struct
fn emit_ffi_types(
    methods: &[DBusMethod],
    Context {
        iface_mod,
        interface_ident,
        ffi_mod,
        vis,
        gio,
        ..
    }: &Context,
) -> TokenStream {
    let async_vfuncs = methods.iter()
        .map(|method| {
            let arg_types = method.args.iter().map(|arg| &arg.syn.ty);
            let fn_ident = &method.item.sig.ident;
            let return_type = vfunc_return_type(vfunc_output_type(&method.return_type, gio));
            let auto_args = quote!(
                #gio::DBusCallFlags,
                ::std::option::Option<::std::time::Duration>,
                ::std::option::Option<#gio::Cancellable,
            );
            quote!(pub(super) #fn_ident: fn(&super::#interface_ident, #(#arg_types,)* #auto_args>) -> #return_type)
        });

    quote! {
        #vis mod #ffi_mod {
            // Needed in case the vfuncs use types available in the parent scope only
            use super::*;

            #[repr(C)]
            pub struct Instance(::std::ffi::c_void);

            #[derive(Copy, Clone)]
            #[repr(C)]
            pub struct Interface {
                __interface_parent_type: #gio::glib::gobject_ffi::GTypeInterface,
                #(#async_vfuncs,)*
            }

            // Safety: This impl is unsafe because it requires the struct to be `repr(C)` and
            // the first field must be [`glib::gobject_ffi::GTypeInterface`].
            unsafe impl #gio::glib::subclass::types::InterfaceStruct for Interface {
                type Type = super::#iface_mod::#interface_ident;
            }
        }
    }
}

/// Emits the `glib::wrapper!` struct for the interface
fn emit_wrapper(
    Context {
        gio,
        interface_ident,
        iface_mod,
        ..
    }: &Context,
) -> TokenStream {
    quote! {
        #gio::glib::wrapper! {
            pub struct #interface_ident(ObjectInterface<#iface_mod::#interface_ident>);
        }
    }
}

/// Emits the `ObjectInterface` impl
fn emit_iface_types(
    properties: &[DBusProperty],
    ctx @ Context {
        interface_ident,
        iface_mod,
        ffi_mod,
        gio,
        ..
    }: &Context,
) -> TokenStream {
    let name = ctx
        .glib_type_name
        .clone()
        .unwrap_or_else(|| LitStr::new(&interface_ident.to_string(), interface_ident.span()));
    let properties_fn = emit_properties_fn(properties, ctx);
    quote! {
        mod #iface_mod {
            pub enum #interface_ident {}

            // [TODO] pass crate path to `object_interface` macro (crate = #gio::glib)
            #[#gio::glib::object_interface]
            impl #gio::glib::subclass::interface::ObjectInterface for #interface_ident {
                const NAME: &'static str = #name;

                type Instance = super::#ffi_mod::Instance;
                type Interface = super::#ffi_mod::Interface;

                #properties_fn
            }
        }
    }
}

fn emit_properties_fn(
    properties: &[DBusProperty],
    ctx @ Context { gio, .. }: &Context,
) -> TokenStream {
    if properties.is_empty() {
        return TokenStream::default();
    }

    let properties = properties
        .iter()
        .map(|property| emit_property(property, ctx));

    quote! {
        fn properties() -> &'static [#gio::glib::ParamSpec] {
            static PROPERTIES: ::std::sync::OnceLock<::std::vec::Vec<#gio::glib::ParamSpec>> = ::std::sync::OnceLock::new();
            PROPERTIES.get_or_init(|| vec![#(#properties,)*])
        }
    }
}

fn emit_property(property: &DBusProperty, ctx @ Context { gio, .. }: &Context) -> TokenStream {
    let name = property.dbus_name.value().to_kebab_case();
    let type_ = &property.type_;
    let access_flags = emit_property_access_flags(property.access, ctx);
    let deprecated_flag = if property.deprecated {
        Some(quote!(| #gio::glib::ParamFlags::DEPRECATED))
    } else {
        None
    };
    quote! {
        #gio::glib::ParamSpecBuilderExt::flags(
            (<#type_ as #gio::glib::HasParamSpec>::param_spec_builder())(#name),
            #access_flags #deprecated_flag
        )
        .build()
    }
}

fn emit_property_access_flags(
    access: DBusPropertyAccess,
    Context { gio, .. }: &Context,
) -> TokenStream {
    let access = match access {
        DBusPropertyAccess::Read => quote!(READABLE),
        DBusPropertyAccess::Write => quote!(WRITABLE),
        DBusPropertyAccess::ReadWrite => quote!(READWRITE),
    };
    quote!(#gio::glib::ParamFlags::#access)
}

/// Emits the `<Interface>Ext` trait which is the interface's Rust API.
fn emit_ext_trait(
    methods: &[DBusMethod],
    Context {
        interface_ident,
        ext_ident,
        method_calls_mod,
        gio,
        ..
    }: &Context,
) -> TokenStream {
    #[allow(non_snake_case)]
    let IsA = quote!(#gio::glib::object::IsA);

    let trait_methods = methods.iter().map(|method| {
        let ident = &method.item.sig.ident;
        let receiver_arg = method.item.sig.inputs.first();
        let args = method.args.iter().map(|arg| {
            let arg = &arg.syn;
            quote!(#arg)
        });
        let arg_names = method.args.iter().map(|arg| &arg.ident.ident);
        let method_call = method_call_ident(ident);
        quote! {
            fn #ident(#receiver_arg, #(#args,)*) -> #method_calls_mod::#method_call<'_> {
                let __this = #gio::glib::prelude::Cast::upcast_ref::<#interface_ident>(self);
                #method_calls_mod::#method_call::new(
                    __this,
                    #(#arg_names,)*
                )
            }
        }
    });

    quote! {
        pub trait #ext_ident: #IsA<#interface_ident> {
            #(#trait_methods)*
        }

        impl<T: #IsA<#interface_ident>> #ext_ident for T {}
    }
}

/// Emits the `<Interface>Impl` trait which is the trait that classes implementing
/// the interface will have to implement.
fn emit_impl_trait(
    methods: &[DBusMethod],
    Context {
        interface_ident,
        impl_ident,
        vis,
        gio,
        ..
    }: &Context,
) -> TokenStream {
    #[allow(non_snake_case)]
    let IsA = quote!(#gio::glib::object::IsA);
    let object_impl_trait = quote!(#gio::glib::subclass::object::ObjectImpl);
    let object_subclass_bound =
        quote!(#gio::glib::subclass::types::ObjectSubclass<Type: #IsA<#interface_ident>>);

    let trait_methods = methods.iter().map(|method| {
        let ident = &method.item.sig.ident;
        let receiver_arg = method.item.sig.inputs.first();
        let args = method.args.iter().map(|arg| {
            let arg = &arg.syn;
            quote!(#arg)
        });
        let call_flags_ident = disambiguated_arg("call_flags", &method.args);
        let timeout_ident = disambiguated_arg("timeout", &method.args);
        let cancellable_ident = disambiguated_arg("cancellable", &method.args);
        let auto_args = quote!(
            #call_flags_ident: #gio::DBusCallFlags,
            #timeout_ident: ::std::option::Option<::std::time::Duration>,
            #cancellable_ident: ::std::option::Option<#gio::Cancellable>,
        );
        let return_type = vfunc_return_type(vfunc_output_type(&method.return_type, gio));
        quote! {
            fn #ident(#receiver_arg, #(#args,)* #auto_args) -> #return_type;
        }
    });

    quote! {
        #vis trait #impl_ident: #object_impl_trait + #object_subclass_bound {
            #(#trait_methods)*
        }
    }
}

/// Emits an implementation of the `IsImplementable` trait.
/// This ties together the vfuncs and the `<Interface>Impl` trait.
fn emit_is_implementable_impl(
    methods: &[DBusMethod],
    Context {
        interface_ident,
        impl_ident,
        gio,
        ..
    }: &Context,
) -> TokenStream {
    let object_subclass_trait = quote!(#gio::glib::subclass::types::ObjectSubclass);
    let object_subclass_is_ext_trait = quote!(gio::glib::subclass::types::ObjectSubclassIsExt);
    let cast_trait = quote!(#gio::glib::object::Cast);
    let is_implementable_trait = quote!(#gio::glib::subclass::types::IsImplementable<Obj>);

    let vfunc_initializers = methods.iter().map(|method| {
        let ident = &method.item.sig.ident;
        let arg_names: Vec<_> = method
            .args
            .iter()
            .map(|arg| format_ident!("arg_{}", &arg.ident.ident))
            .collect();
        let auto_arg_names = quote!(call_flags_ident, timeout_ident, cancellable_ident);
        quote! {
            klass.#ident = |obj, #(#arg_names,)* #auto_arg_names| {
                let this = unsafe {
                    #object_subclass_is_ext_trait::imp(
                        #cast_trait::unsafe_cast_ref::<<Obj as #object_subclass_trait>::Type>(
                            obj
                        )
                    )
                };
                #impl_ident::#ident(this, #(#arg_names,)* #auto_arg_names)
            };
        }
    });

    quote! {
        unsafe impl<Obj: #impl_ident> #is_implementable_trait for #interface_ident {
            fn interface_init(iface: &mut #gio::glib::Interface<Self>) {
                let klass = iface.as_mut();
                #(#vfunc_initializers)*
            }
        }
    }
}

fn emit_method_calls(
    methods: &[DBusMethod],
    ctx @ Context {
        vis,
        method_calls_mod,
        ..
    }: &Context,
) -> TokenStream {
    let method_calls = methods.iter().map(|method| emit_method_call(method, ctx));
    quote! {
        #vis mod #method_calls_mod {
            // Needed in case the vfuncs use types available in the parent scope only
            use super::*;

            #(#method_calls)*
        }
    }
}

fn emit_method_call(
    method: &DBusMethod,
    Context {
        interface_ident,
        gio,
        ..
    }: &Context,
) -> TokenStream {
    let method_ident = &method.item.sig.ident;
    let ident = method_call_ident(method_ident);
    let fields: TokenStream = method
        .args
        .iter()
        .map(|arg| {
            let ident = &arg.ident.ident;
            let ty = &arg.syn.ty;
            quote!(#ident: #ty,)
        })
        .collect();
    let field_names: TokenStream = method
        .args
        .iter()
        .map(|arg| {
            let ident = &arg.ident.ident;
            quote!(#ident,)
        })
        .collect();
    let field_accessors = method.args.iter().map(|arg| {
        let ident = &arg.ident.ident;
        quote!(self.#ident)
    });

    let output_type = vfunc_output_type(&method.return_type, gio);
    let future_type = vfunc_return_type(quote!(Self::Output));
    quote! {
        #[non_exhaustive]
        pub struct #ident<'a> {
            #fields
            __this: &'a super::#interface_ident,
            __cancellable: ::std::option::Option<#gio::Cancellable>,
            __flags: #gio::DBusCallFlags,
            __timeout: ::std::option::Option<::std::time::Duration>,
        }

        impl<'a> #ident<'a> {
            pub(super) fn new(__this: &'a super::#interface_ident, #fields) -> Self {
                Self {
                    #field_names
                    __this,
                    __cancellable: ::std::option::Option::None,
                    __flags: #gio::DBusCallFlags::NONE,
                    __timeout: ::std::option::Option::None,
                }
            }

            pub fn cancellable(mut self, cancellable: #gio::Cancellable) -> Self {
                self.__cancellable = ::std::option::Option::Some(cancellable);
                self
            }

            pub fn flags(mut self, flags: #gio::DBusCallFlags) -> Self {
                self.__flags = flags;
                self
            }

            pub fn timeout(mut self, timeout: ::std::time::Duration) -> Self {
                self.__timeout = ::std::option::Option::Some(timeout);
                self
            }
        }

        impl ::std::future::IntoFuture for #ident<'_> {
            type Output = #output_type;
            type IntoFuture = #future_type;

            fn into_future(self) -> Self::IntoFuture {
                let Self { __this, __flags, __timeout, __cancellable, .. } = self;
                let iface = #gio::glib::prelude::ObjectExt::interface::<#interface_ident>(__this).unwrap();
                (iface.as_ref().#method_ident)(__this, #(#field_accessors,)* __flags, __timeout, __cancellable)
            }
        }
    }
}

fn vfunc_output_type(return_type: &Type, gio: &Path) -> TokenStream {
    quote!(::std::result::Result<#return_type, #gio::glib::Error>)
}

fn vfunc_return_type(output_type: TokenStream) -> TokenStream {
    quote!(::std::pin::Pin<::std::boxed::Box<dyn ::std::future::Future<Output = #output_type>>>)
}

fn method_call_ident(method_ident: &Ident) -> Ident {
    format_ident!(
        "{}MethodCall",
        ident_name(method_ident).to_pascal_case(),
        span = method_ident.span()
    )
}

fn disambiguated_arg<'a>(ident: &'a str, args: &[DBusMethodArg]) -> Ident {
    let mut new_ident = Cow::Borrowed(ident);
    let mut counter = 1;
    while args.iter().any(|m| m.ident.ident == new_ident) {
        new_ident = Cow::Owned(format!("{ident}_{counter}"));
        counter += 1;
    }
    Ident::new(&new_ident, Span::call_site())
}
