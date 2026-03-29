// Take a look at the license at the top of the repository in the LICENSE file.

use crate::dbus_interface::attributes::ATTRIBUTE_NAME;
use syn::punctuated::Punctuated;
use syn::{Attribute, LitStr, Meta, Token, Type, parenthesized};

/// The attribute placed on an `fn` item in the `#[dbus_interface]` impl block.
pub(crate) enum DBusItemAttribute {
    /// `#[dbus(...)]`
    Method(DBusItemAttributeMethod),
    /// `#[dbus(signal, ...)]`
    Signal(DBusItemAttributeSignal),
}

pub(crate) struct DBusItemAttributeMethod {
    pub(crate) name: Option<LitStr>,
    pub(crate) out_args: Option<Punctuated<LitStr, Token![,]>>,
    pub(crate) manual_return: Option<Box<Type>>,
}

pub(crate) struct DBusItemAttributeSignal {
    pub(crate) name: Option<LitStr>,
}

impl DBusItemAttribute {
    pub(crate) fn parse(attrs: &[Attribute]) -> syn::Result<Self> {
        let tag = DBusItemAttributeTag::parse(attrs)?;
        match tag {
            None => DBusItemAttributeMethod::parse(attrs).map(Self::Method),
            Some(DBusItemAttributeTag::Signal) => {
                DBusItemAttributeSignal::parse(attrs).map(Self::Signal)
            }
        }
    }
}

impl DBusItemAttributeMethod {
    fn parse(attrs: &[Attribute]) -> syn::Result<Self> {
        let mut name: Option<LitStr> = None;
        let mut out_args: Option<Punctuated<LitStr, Token![,]>> = None;
        let mut manual_return: Option<Box<Type>> = None;
        for attr in attrs {
            if attr.path().is_ident(ATTRIBUTE_NAME) {
                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("name") {
                        name = Some(meta.value()?.parse()?);
                        Ok(())
                    } else if meta.path.is_ident("out_args") {
                        let content;
                        parenthesized!(content in meta.input);
                        out_args = Some(content.call(Punctuated::parse_terminated)?);
                        Ok(())
                    } else if meta.path.is_ident("manual_return") {
                        let content;
                        parenthesized!(content in meta.input);
                        manual_return = Some(Box::new(content.parse()?));
                        Ok(())
                    } else {
                        Err(meta.error(format!(
                            "unknown attribute `{}`. Possible attributes are `name`, `out_args`, `manual_return`",
                            meta.path.get_ident().unwrap(),
                        )))
                    }
                })?;
            }
        }
        Ok(Self {
            name,
            out_args,
            manual_return,
        })
    }
}

impl DBusItemAttributeSignal {
    fn parse(attrs: &[Attribute]) -> syn::Result<Self> {
        let mut name: Option<LitStr> = None;
        for attr in attrs {
            if attr.path().is_ident(ATTRIBUTE_NAME) {
                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("signal") {
                        Ok(())
                    } else if meta.path.is_ident("name") {
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

enum DBusItemAttributeTag {
    Signal,
}

impl DBusItemAttributeTag {
    pub(crate) fn parse(attrs: &[Attribute]) -> syn::Result<Option<Self>> {
        let mut tag = None;
        for attr in attrs {
            if attr.path().is_ident(ATTRIBUTE_NAME) {
                let nested =
                    attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;
                if let Some(first_nested) = nested.first() {
                    if first_nested.path().is_ident("signal") {
                        tag = Some(Self::Signal);
                        first_nested.require_path_only()?;
                    }
                }
            }
        }
        Ok(tag)
    }
}
