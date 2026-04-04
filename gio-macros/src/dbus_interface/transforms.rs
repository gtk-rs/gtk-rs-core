// Take a look at the license at the top of the repository in the LICENSE file.

use crate::dbus_interface::attributes::ATTRIBUTE_NAME;
use syn::{Attribute, FnArg, ItemTrait, TraitItem, TraitItemFn};

pub(crate) fn remove_dbus_attribute_from_trait(trait_: &mut ItemTrait) {
    for trait_item in &mut trait_.items {
        remove_dbus_attribute_from_trait_item(trait_item);
    }
}

pub(crate) fn remove_dbus_attribute_from_trait_item(item: &mut TraitItem) {
    if let TraitItem::Fn(trait_item_fn) = item {
        remove_dbus_attribute_from_trait_item_fn(trait_item_fn)
    }
}

pub(crate) fn remove_dbus_attribute_from_trait_item_fn(item: &mut TraitItemFn) {
    remove_dbus_attribute_from_list(&mut item.attrs);
    for arg in &mut item.sig.inputs {
        remove_dbus_attribute_from_fn_arg(arg);
    }
}

fn remove_dbus_attribute_from_list(attributes: &mut Vec<Attribute>) {
    attributes.retain(|attr| !attr.meta.path().is_ident(ATTRIBUTE_NAME));
}

fn remove_dbus_attribute_from_fn_arg(arg: &mut FnArg) {
    match arg {
        FnArg::Receiver(_receiver) => {} // No attributes supported on the receiver arg.
        FnArg::Typed(pat_type) => remove_dbus_attribute_from_list(&mut pat_type.attrs),
    }
}
