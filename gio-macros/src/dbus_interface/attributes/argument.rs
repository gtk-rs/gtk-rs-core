// Take a look at the license at the top of the repository in the LICENSE file.

use crate::dbus_interface::attributes::ATTRIBUTE_NAME;
use crate::dbus_interface::parse::DBusMethodArgumentProvider;
use syn::{Attribute, LitStr};

pub(crate) struct DBusMethodArgumentAttribute {
    pub(crate) name: Option<LitStr>,
    pub(crate) provider: Option<DBusMethodArgumentProvider>,
}

impl DBusMethodArgumentAttribute {
    pub(crate) fn parse(attrs: &[Attribute]) -> syn::Result<Self> {
        let mut provider = None;
        let mut name: Option<LitStr> = None;
        for attr in attrs {
            if attr.path().is_ident(ATTRIBUTE_NAME) {
                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("connection") {
                        provider = Some(DBusMethodArgumentProvider::Connection);
                        Ok(())
                    }
                    else if meta.path.is_ident("invocation") {
                        provider = Some(DBusMethodArgumentProvider::Invocation);
                        Ok(())
                    }
                    else if meta.path.is_ident("name") {
                        name = Some(meta.value()?.parse()?);
                        Ok(())
                    } else {
                        Err(meta.error(format!(
                            "unknown attribute `{}`. Possible attributes are `connection`, `invocation`, `signal_emitter`, `name`",
                            meta.path.get_ident().unwrap(),
                        )))
                    }
                })?;
            }
        }
        Ok(Self { name, provider })
    }
}
