// Take a look at the license at the top of the repository in the LICENSE file.

use crate::dbus_interface::attributes::ATTRIBUTE_NAME;
use syn::{Attribute, LitStr};

pub(crate) struct DBusMethodArgumentAttribute {
    pub(crate) name: Option<LitStr>,
}

impl DBusMethodArgumentAttribute {
    pub(crate) fn parse(attrs: &[Attribute]) -> syn::Result<Self> {
        let mut name: Option<LitStr> = None;
        for attr in attrs {
            if attr.path().is_ident(ATTRIBUTE_NAME) {
                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("name") {
                        name = Some(meta.value()?.parse()?);
                        Ok(())
                    } else {
                        Err(meta.error(format!(
                            "unknown attribute `{}`. Possible attributes are `name`",
                            meta.path.get_ident().unwrap(),
                        )))
                    }
                })?;
            }
        }
        Ok(Self { name })
    }
}
