// Take a look at the license at the top of the repository in the LICENSE file.

use crate::dbus_interface::attributes::DBusItemAttributeSignal;
use crate::dbus_interface::parse::{default_method_name, is_deprecated};
use syn::spanned::Spanned as _;
use syn::{Ident, LitStr, TraitItemFn};

pub(crate) struct DBusSignal {
    pub(crate) item: TraitItemFn,
    pub(crate) dbus_name: LitStr,
    pub(crate) deprecated: bool,
}

impl DBusSignal {
    pub(super) fn parse(
        item: TraitItemFn,
        attr: DBusItemAttributeSignal,
    ) -> syn::Result<DBusSignal> {
        let dbus_name = attr
            .name
            .unwrap_or_else(|| default_signal_name(&item.sig.ident));
        let deprecated = is_deprecated(&item.attrs);

        if matches!(item.sig.output, syn::ReturnType::Type(..)) {
            return Err(syn::Error::new(
                item.sig.output.span(),
                "signal must not specify a return type",
            ));
        }

        Ok(DBusSignal {
            item,
            dbus_name,
            deprecated,
        })
    }
}

fn default_signal_name(ident: &Ident) -> LitStr {
    default_method_name(ident)
}
