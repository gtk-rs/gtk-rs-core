// Take a look at the license at the top of the repository in the LICENSE file.

use crate::dbus_interface::attributes::EmitsChangedSignal;
use proc_macro2::TokenStream;
use syn::spanned::Spanned;
use syn::{LitStr, Path};

/// The `#[dbus_interface(...)]` attribute.
pub(crate) struct DBusInterfaceAttribute {
    pub(crate) name: LitStr,
    pub(crate) type_name: Option<LitStr>,
    pub(crate) crate_: Option<Path>,
    pub(crate) emits_changed_signal: Option<EmitsChangedSignal>,
    pub(crate) generate_proxy: bool,
    pub(crate) generate_skeleton: bool,
}

impl DBusInterfaceAttribute {
    pub(crate) fn parse(attr: TokenStream) -> syn::Result<Self> {
        let span = attr.span();
        let mut name: Option<LitStr> = None;
        let mut type_name: Option<LitStr> = None;
        let mut crate_: Option<Path> = None;
        let mut emits_changed_signal: Option<EmitsChangedSignal> = None;
        let mut generate_proxy = false;
        let mut generate_skeleton = false;
        let p = syn::meta::parser(|meta| {
            if meta.path.is_ident("name") {
                name = Some(meta.value()?.parse()?);
                Ok(())
            } else if meta.path.is_ident("type_name") {
                type_name = Some(meta.value()?.parse()?);
                Ok(())
            } else if meta.path.is_ident("crate") {
                crate_ = Some(meta.value()?.parse()?);
                Ok(())
            } else if meta.path.is_ident("emits_changed_signal") {
                emits_changed_signal = Some(meta.value()?.parse()?);
                Ok(())
            } else if meta.path.is_ident("proxy") {
                generate_proxy = true;
                Ok(())
            } else if meta.path.is_ident("skeleton") {
                generate_skeleton = true;
                Ok(())
            } else {
                Err(meta.error(format!(
                    "unknown attribute `{}`. Possible attributes are `name`, `type_name`, `crate`, `emits_changed_signal`",
                    meta.path.get_ident().unwrap(),
                )))
            }
        });
        syn::parse::Parser::parse2(p, attr)?;

        let Some(name) = name else {
            return Err(syn::Error::new(
                span,
                "attribute `#[dbus_interface(name = ...)]` must be specified",
            ));
        };
        Ok(Self {
            name,
            type_name,
            crate_,
            emits_changed_signal,
            generate_proxy,
            generate_skeleton,
        })
    }
}
