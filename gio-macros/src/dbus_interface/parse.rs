// Take a look at the license at the top of the repository in the LICENSE file.

use crate::dbus_interface::attributes::{
    DBusItemAttribute, DBusItemAttributeMethod, DBusItemAttributeSignal,
    DBusMethodArgumentAttribute,
};
use crate::dbus_interface::transforms::{
    remove_dbus_attribute_from_impl_item, remove_dbus_attribute_from_item_fn,
};
use crate::utils::{ident_name, ident_name_as_lit_str};
use heck::ToPascalCase as _;
use std::collections::BTreeMap;
use std::collections::btree_map::Entry;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{Attribute, FnArg, Ident, ImplItem, ImplItemFn, LitStr, Pat, PatType, Token, Type};

#[derive(Default)]
pub(crate) struct DBusItems {
    pub(crate) methods: BTreeMap<String, DBusMethod>,
    pub(crate) signals: BTreeMap<String, DBusSignal>,
    pub(crate) errors: Vec<(ImplItem, syn::Error)>,
}

enum DBusItem {
    Method(DBusMethod),
    Signal(DBusSignal),
    Error(ImplItem, syn::Error),
}

pub(crate) struct DBusMethod {
    pub(crate) item: ImplItemFn,
    pub(crate) dbus_name: LitStr,
    pub(crate) args: Vec<DBusMethodArgument>,
    pub(crate) out_arg_names: Option<Punctuated<LitStr, Token![,]>>,
    pub(crate) manual_return: Option<Box<Type>>,
    pub(crate) deprecated: bool,
}

pub(crate) struct DBusMethodArgument {
    pub(crate) arg: PatType,
    pub(crate) dbus_name: LitStr,
    pub(crate) provider: DBusMethodArgumentProvider,
}

pub(crate) enum DBusMethodArgumentProvider {
    Parameters { index: usize },
    Connection,
    Invocation,
}

pub(crate) struct DBusSignal {
    pub(crate) item: ImplItemFn,
    pub(crate) dbus_name: LitStr,
    pub(crate) deprecated: bool,
}

pub(crate) fn parse_impl_items(items: Vec<ImplItem>) -> DBusItems {
    let mut output = DBusItems::default();
    for item in items {
        match parse_impl_item(item) {
            DBusItem::Method(mut method) => {
                remove_dbus_attribute_from_item_fn(&mut method.item);
                if let Entry::Vacant(entry) = output.methods.entry(method.dbus_name.value()) {
                    entry.insert(method);
                } else {
                    let error = syn::Error::new(
                        method.item.span(),
                        "a method with this name is already defined",
                    );
                    output.errors.push((ImplItem::Fn(method.item), error));
                }
            }
            DBusItem::Signal(dbus_signal) => {
                let mut impl_item = ImplItem::Fn(dbus_signal.item);
                remove_dbus_attribute_from_impl_item(&mut impl_item);
                let span = impl_item.span();
                output
                    .errors
                    .push((impl_item, syn::Error::new(span, "[TODO]")));
            }
            DBusItem::Error(mut impl_item, error) => {
                remove_dbus_attribute_from_impl_item(&mut impl_item);
                output.errors.push((impl_item, error));
            }
        }
    }
    output
}

fn parse_impl_item(item: ImplItem) -> DBusItem {
    match item {
        ImplItem::Fn(ref impl_item_fn) => match parse_impl_item_fn(impl_item_fn.clone()) {
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

fn parse_impl_item_fn(item: ImplItemFn) -> Result<DBusItem, syn::Error> {
    let attr = DBusItemAttribute::parse(&item.attrs)?;
    match attr {
        DBusItemAttribute::Signal(attr) => {
            Ok(DBusItem::Signal(parse_impl_item_fn_for_signal(item, attr)?))
        }
        DBusItemAttribute::Method(attr) => {
            Ok(DBusItem::Method(parse_impl_item_fn_for_method(item, attr)?))
        }
    }
}

fn parse_impl_item_fn_for_method(
    item: ImplItemFn,
    attr: DBusItemAttributeMethod,
) -> syn::Result<DBusMethod> {
    let dbus_name = attr
        .name
        .unwrap_or_else(|| default_method_name(&item.sig.ident));
    let mut parameter_index = 0;
    let args = item
        .sig
        .inputs
        .iter()
        .filter_map(|input| parse_method_arg(input.clone(), &mut parameter_index).transpose())
        .collect::<Result<_, _>>()?;
    let deprecated = is_deprecated(&item.attrs);
    Ok(DBusMethod {
        item,
        dbus_name,
        args,
        out_arg_names: attr.out_args,
        manual_return: attr.manual_return,
        deprecated,
    })
}

fn parse_impl_item_fn_for_signal(
    item: ImplItemFn,
    attr: DBusItemAttributeSignal,
) -> syn::Result<DBusSignal> {
    let dbus_name = attr
        .name
        .unwrap_or_else(|| default_method_name(&item.sig.ident));
    let deprecated = is_deprecated(&item.attrs);

    if !item.block.stmts.is_empty() {
        return Err(syn::Error::new(
            item.block.stmts.first().span(),
            "signal body must be left empty; it is implemented automatically for you",
        ));
    }

    Ok(DBusSignal {
        item,
        dbus_name,
        deprecated,
    })
}

fn parse_method_arg(
    arg: FnArg,
    parameter_index: &mut usize,
) -> syn::Result<Option<DBusMethodArgument>> {
    let FnArg::Typed(typed) = arg else {
        return Ok(None);
    };
    let attr = DBusMethodArgumentAttribute::parse(&typed.attrs)?;
    let dbus_name = attr
        .name
        .map(Ok)
        .unwrap_or_else(|| default_parameter_name(&typed.pat))?;
    Ok(Some(DBusMethodArgument {
        arg: typed,
        dbus_name,
        provider: attr.provider.unwrap_or_else(|| {
            let index = *parameter_index;
            *parameter_index += 1;
            DBusMethodArgumentProvider::Parameters { index }
        }),
    }))
}

fn is_deprecated(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|attr| attr.path().is_ident("deprecated"))
}

fn default_method_name(ident: &Ident) -> LitStr {
    let ident_name = ident_name(ident);
    LitStr::new(&ident_name.to_pascal_case(), ident.span())
}

fn default_parameter_name(pat: &Pat) -> syn::Result<LitStr> {
    if let Pat::Ident(pat_ident) = pat {
        Ok(ident_name_as_lit_str(&pat_ident.ident))
    } else {
        Err(syn::Error::new(
            pat.span(),
            "unable to determine parameter name, specify one using `#[dbus(name = \"...\")]`",
        ))
    }
}
