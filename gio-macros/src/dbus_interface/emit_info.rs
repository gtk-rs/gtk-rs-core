// Take a look at the license at the top of the repository in the LICENSE file.

use crate::dbus_interface::attributes::DBusInterfaceAttribute;
use crate::dbus_interface::parse::{DBusMethod, DBusMethodArgument, DBusMethodArgumentProvider};
use proc_macro2::TokenStream;
use quote::quote;
use std::borrow::Cow;
use syn::punctuated::Punctuated;
use syn::{Ident, LitStr, ReturnType, Token, Type, TypeTraitObject, TypeTuple, token};

pub(crate) fn emit_interface_info(
    attr: &DBusInterfaceAttribute,
    ident: &Ident,
    gio: &TokenStream,
) -> syn::Result<TokenStream> {
    let name = &attr.name;
    let helpers = quote!(#gio::__macro_helpers::dbus_interface_skeleton);
    Ok(quote! {
        #gio::DBusInterfaceInfo::builder()
            .name(#name)
            .methods(<#ident as #helpers::DBusMethods>::method_infos())
            .build()
    })
}

pub(crate) fn emit_method_info(method: &DBusMethod, gio: &TokenStream) -> syn::Result<TokenStream> {
    let name = &method.dbus_name;
    let in_args = method
        .args
        .iter()
        .filter_map(|arg| emit_in_arg_info(arg, gio));
    let return_type = method_return_type(method);
    let out_args = emit_out_arg_infos(&return_type, method.out_arg_names.as_ref(), gio)?;
    let annotations = method
        .deprecated
        .then_some(("org.freedesktop.DBus.Deprecated", "true"))
        .into_iter();
    let annotations = annotations.map(|(key, value)| emit_annotation(key, value, gio));
    Ok(quote! {
        #gio::DBusMethodInfo::builder()
            .in_args([#(#in_args,)*])
            .out_args(#out_args)
            .annotations([#(#annotations,)*])
            .name(#name)
            .build()
    })
}

fn method_return_type(method: &DBusMethod) -> Type {
    if let Some(ty) = &method.manual_return {
        return ty.as_ref().clone();
    };
    match &method.item.sig.output {
        ReturnType::Type(_, type_) => (**type_).clone(),
        ReturnType::Default => Type::Tuple(TypeTuple {
            paren_token: token::Paren::default(),
            elems: Punctuated::new(),
        }),
    }
}

fn emit_in_arg_info(argument: &DBusMethodArgument, gio: &TokenStream) -> Option<TokenStream> {
    if !matches!(
        argument.provider,
        DBusMethodArgumentProvider::Parameters { .. }
    ) {
        return None;
    }

    let name = &argument.dbus_name;
    let signature = emit_signature_str(&argument.arg.ty, gio);
    Some(quote! {
        #gio::DBusArgInfo::builder()
            .name(#name)
            .signature(#signature)
            .build()
    })
}

// The out args are fundamentally different from the in args.
//
// Each function argument maps neatly to an in arg so we know the count and their names at generation time.
//
// The out args come from the return type for which we know how many tuple elements it has only at runtime
// We could try do some matching at compile time but this would break as soon as type aliases are involved.
fn emit_out_arg_infos(
    return_type: &Type,
    out_arg_names: Option<&Punctuated<LitStr, Token![,]>>,
    gio: &TokenStream,
) -> syn::Result<TokenStream> {
    let out_arg_variant_type = emit_out_arg_variant_type(return_type, gio);
    let out_arg_names = out_arg_names.cloned().unwrap_or_default();
    let out_arg_info = emit_out_arg_info(
        quote!(variant_type),
        quote!(out_arg_names),
        quote!(index),
        gio,
    );
    Ok(quote! {
        {
            let out_arg_names: [&str; _] = [#out_arg_names];
            let out_args_tuple_type = #out_arg_variant_type;
            let mut out_args: Vec<#gio::DBusArgInfo> = Vec::new();
            for (index, variant_type) in out_args_tuple_type.tuple_types().enumerate() {
                out_args.push(#out_arg_info);
            }
            out_args
        }
    })
}

fn emit_out_arg_info(
    variant_type_expr: TokenStream,
    out_arg_names_expr: TokenStream,
    index_expr: TokenStream,
    gio: &TokenStream,
) -> TokenStream {
    quote! {
        #gio::DBusArgInfo::builder()
            .name(#out_arg_names_expr.get(#index_expr).map(|s| *s))
            .signature(#variant_type_expr.as_str())
            .build()
    }
}

fn emit_out_arg_variant_type(type_: &Type, gio: &TokenStream) -> TokenStream {
    let type_ = replace_impl_trait_with_dyn_trait(type_);
    let helpers = quote!(#gio::__macro_helpers::dbus_interface_skeleton);
    quote! {
        {
            use #helpers::static_return_type::*;
            let variant_type = (&&&type_of::<#type_>()).static_variant_type();
            #helpers::variant_type::ensure_tuple(variant_type)
        }
    }
}

fn replace_impl_trait_with_dyn_trait(type_: &Type) -> Cow<'_, Type> {
    if let Type::ImplTrait(type_impl_trait) = type_ {
        Cow::Owned(Type::TraitObject(TypeTraitObject {
            dyn_token: Some(Token![dyn](type_impl_trait.impl_token.span)),
            bounds: type_impl_trait.bounds.clone(),
        }))
    } else {
        Cow::Borrowed(type_)
    }
}

fn emit_signature_str(type_: &Type, gio: &TokenStream) -> TokenStream {
    quote! {
        <#type_ as #gio::glib::variant::StaticVariantType>::static_variant_type().as_str()
    }
}

fn emit_annotation(key: &str, value: &str, gio: &TokenStream) -> TokenStream {
    quote! {
        #gio::DBusAnnotationInfo::builder()
            .key(#key)
            .value(#value)
            .build()
    }
}
