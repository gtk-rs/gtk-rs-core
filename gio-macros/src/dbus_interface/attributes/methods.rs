// Take a look at the license at the top of the repository in the LICENSE file.

use proc_macro2::TokenStream;
use syn::Path;

/// The `#[dbus_methods(...)]` attribute.
pub(crate) struct DBusMethodsAttribute {
    pub(crate) crate_: Option<Path>,
}

impl DBusMethodsAttribute {
    pub(crate) fn parse(attr: TokenStream) -> syn::Result<Self> {
        let mut crate_: Option<Path> = None;
        let p = syn::meta::parser(|meta| {
            if meta.path.is_ident("crate") {
                crate_ = Some(meta.value()?.parse()?);
                Ok(())
            } else {
                Err(meta.error(format!(
                    "unknown attribute `{}`. Possible attributes are `name`, `crate`, `emits_changed_signal`",
                    meta.path.get_ident().unwrap(),
                )))
            }
        });
        syn::parse::Parser::parse(p, attr.into())?;

        Ok(Self { crate_ })
    }
}
