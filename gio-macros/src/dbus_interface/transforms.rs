// Take a look at the license at the top of the repository in the LICENSE file.

use crate::dbus_interface::attributes::ATTRIBUTE_NAME;
use syn::{Attribute, FnArg, ImplItem, ImplItemFn, ItemImpl};

pub(crate) fn remove_dbus_attribute_from_impl(item: &mut ItemImpl) {
    for impl_item in &mut item.items {
        remove_dbus_attribute_from_impl_item(impl_item);
    }
}

pub(crate) fn remove_dbus_attribute_from_impl_item(item: &mut ImplItem) {
    if let ImplItem::Fn(impl_item_fn) = item {
        remove_dbus_attribute_from_item_fn(impl_item_fn)
    }
}

pub(crate) fn remove_dbus_attribute_from_item_fn(item: &mut ImplItemFn) {
    remove_dbus_attribute_from_list(&mut item.attrs);
    for arg in &mut item.sig.inputs {
        remove_dbus_attribute_from_fn_arg(arg);
    }
}

fn remove_dbus_attribute_from_fn_arg(arg: &mut FnArg) {
    match arg {
        FnArg::Receiver(receiver) => remove_dbus_attribute_from_list(&mut receiver.attrs),
        FnArg::Typed(pat_type) => remove_dbus_attribute_from_list(&mut pat_type.attrs),
    }
}

fn remove_dbus_attribute_from_list(attributes: &mut Vec<Attribute>) {
    attributes.retain(|attr| !attr.meta.path().is_ident(ATTRIBUTE_NAME));
}
