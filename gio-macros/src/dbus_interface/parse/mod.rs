// Take a look at the license at the top of the repository in the LICENSE file.

use crate::dbus_interface::attributes::DBusItemAttribute;
use proc_macro2::Span;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned as _;
use syn::{Attribute, FnArg, Token, TraitItem, TraitItemFn};

mod method;
pub(crate) use method::*;
mod signal;
pub(crate) use signal::*;
mod property;
pub(crate) use property::*;

pub(crate) enum DBusItem {
    Method(DBusMethod),
    Signal(DBusSignal),
    Property(DBusProperty),
    Error(TraitItem, syn::Error),
}

impl DBusItem {
    pub(crate) fn method(&self) -> Option<&DBusMethod> {
        if let Self::Method(method) = self {
            Some(method)
        } else {
            None
        }
    }

    pub(crate) fn signal(&self) -> Option<&DBusSignal> {
        if let Self::Signal(signal) = self {
            Some(signal)
        } else {
            None
        }
    }

    pub(crate) fn property(&self) -> Option<&DBusProperty> {
        if let Self::Property(property) = self {
            Some(property)
        } else {
            None
        }
    }

    pub(crate) fn error(&self) -> Option<(&TraitItem, &syn::Error)> {
        if let Self::Error(item, error) = self {
            Some((item, error))
        } else {
            None
        }
    }
}

pub(crate) fn parse_trait_items(items: &[TraitItem]) -> Vec<DBusItem> {
    items.iter().cloned().map(parse_trait_item).collect()
}

fn parse_trait_item(item: TraitItem) -> DBusItem {
    match item {
        TraitItem::Fn(ref impl_item_fn) => match parse_trait_item_fn(impl_item_fn.clone()) {
            Ok(v) => v,
            Err(error) => DBusItem::Error(item, error),
        },
        _ => {
            let span = item.span();
            DBusItem::Error(
                item,
                syn::Error::new(span, "unsupported item, only `fn`s are supported"),
            )
        }
    }
}

fn parse_trait_item_fn(item: TraitItemFn) -> Result<DBusItem, syn::Error> {
    let attr = DBusItemAttribute::parse(&item.attrs, item.span())?;
    first_arg_is_self_by_ref_or_err(&item.sig.inputs, item.sig.ident.span())?;
    match attr {
        DBusItemAttribute::Method(attr) => Ok(DBusItem::Method(DBusMethod::parse(item, attr)?)),
        DBusItemAttribute::Signal(attr) => Ok(DBusItem::Signal(DBusSignal::parse(item, attr)?)),
        DBusItemAttribute::Property(attr) => {
            Ok(DBusItem::Property(DBusProperty::parse(item, attr)?))
        }
    }
}

fn is_deprecated(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|attr| attr.path().is_ident("deprecated"))
}

fn first_arg_is_self_by_ref_or_err(
    inputs: &Punctuated<FnArg, Token![,]>,
    signature_span: Span,
) -> Result<(), syn::Error> {
    const ERROR_MESSAGE: &str = "the first parameter must be `&self`";
    let Some(first_arg) = inputs.first() else {
        return Err(syn::Error::new(signature_span, ERROR_MESSAGE));
    };
    let FnArg::Receiver(receiver) = first_arg else {
        return Err(syn::Error::new(first_arg.span(), ERROR_MESSAGE));
    };
    if receiver.colon_token.is_some() {
        return Err(syn::Error::new(receiver.ty.span(), ERROR_MESSAGE));
    }
    if let Some(mut_) = &receiver.mutability {
        return Err(syn::Error::new(mut_.span(), ERROR_MESSAGE));
    }
    Ok(())
}
