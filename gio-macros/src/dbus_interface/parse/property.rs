// Take a look at the license at the top of the repository in the LICENSE file.

use crate::dbus_interface::attributes::{
    DBusItemAttributeProperty, DBusPropertyAccess, EmitsChangedSignal,
};
use crate::dbus_interface::parse::{default_method_name, is_deprecated};
use syn::spanned::Spanned as _;
use syn::{Ident, LitStr, ReturnType, TraitItemFn, Type};

#[derive(Clone)]
pub(crate) struct DBusProperty {
    pub(crate) item: TraitItemFn,
    pub(crate) dbus_name: LitStr,
    pub(crate) type_: Box<Type>,
    pub(crate) access: DBusPropertyAccess,
    pub(crate) emits_changed_signal: Option<EmitsChangedSignal>,
    pub(crate) deprecated: bool,
}

impl DBusProperty {
    pub(super) fn parse(
        item: TraitItemFn,
        attr: DBusItemAttributeProperty,
    ) -> syn::Result<DBusProperty> {
        let dbus_name = attr
            .name
            .unwrap_or_else(|| default_property_name(&item.sig.ident));
        let deprecated = is_deprecated(&item.attrs);

        if item.sig.inputs.len() > 1 {
            return Err(syn::Error::new(
                item.sig.inputs.iter().nth(1).span(),
                "property must not have any parameters",
            ));
        }

        let ReturnType::Type(_, type_) = &item.sig.output else {
            return Err(syn::Error::new(
                item.sig.ident.span(),
                "property must have a return type",
            ));
        };

        Ok(DBusProperty {
            type_: type_.clone(),
            item,
            dbus_name,
            access: attr.access,
            emits_changed_signal: attr.emits_changed_signal,
            deprecated,
        })
    }
}

fn default_property_name(ident: &Ident) -> LitStr {
    default_method_name(ident)
}
