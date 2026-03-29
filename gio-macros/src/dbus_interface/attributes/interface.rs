// Take a look at the license at the top of the repository in the LICENSE file.

use crate::dbus_interface::attributes::EmitsChangedSignal;
use proc_macro2::Span;
use syn::{Attribute, LitStr, Path};

/// The `#[dbus_interface(...)]` attribute.
pub(crate) struct DBusInterfaceAttribute {
    pub(crate) name: LitStr,
    pub(crate) crate_: Option<Path>,
    pub(crate) emits_changed_signal: Option<EmitsChangedSignal>,
}

impl DBusInterfaceAttribute {
    pub(crate) fn parse(attrs: &[Attribute], span: Span) -> syn::Result<Self> {
        let mut name: Option<LitStr> = None;
        let mut crate_: Option<Path> = None;
        let mut emits_changed_signal: Option<EmitsChangedSignal> = None;
        for attr in attrs {
            if attr.path().is_ident("dbus_interface") {
                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("name") {
                        name = Some(meta.value()?.parse()?);
                        Ok(())
                    } else if meta.path.is_ident("crate") {
                        crate_ = Some(meta.value()?.parse()?);
                        Ok(())
                    } else if meta.path.is_ident("emits_changed_signal") {
                        emits_changed_signal = Some(meta.value()?.parse()?);
                        Ok(())
                    } else {
                        Err(meta.error(format!(
                            "unknown attribute `{}`. Possible attributes are `name`, `crate`, `emits_changed_signal`",
                            meta.path.get_ident().unwrap(),
                        )))
                    }
                })?;
            }
        }

        let Some(name) = name else {
            return Err(syn::Error::new(
                span,
                "attribute `#[dbus_interface(name = ...)]` must be specified",
            ));
        };
        Ok(Self {
            name,
            crate_,
            emits_changed_signal,
        })
    }
}
