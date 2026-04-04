// Take a look at the license at the top of the repository in the LICENSE file.

use crate::dbus_interface::attributes::{ATTRIBUTE_NAME, EmitsChangedSignal};
use proc_macro2::Span;
use syn::punctuated::Punctuated;
use syn::{Attribute, LitStr, Meta, Token, parenthesized};

/// The attribute placed on an `fn` item in the `#[dbus_interface]` trait.
pub(crate) enum DBusItemAttribute {
    /// `#[dbus(...)]`
    Method(DBusItemAttributeMethod),
    /// `#[dbus(signal, ...)]`
    Signal(DBusItemAttributeSignal),
    /// `#[dbus(property, ...)]
    Property(DBusItemAttributeProperty),
}

pub(crate) struct DBusItemAttributeMethod {
    pub(crate) name: Option<LitStr>,
    pub(crate) out_args: Option<Punctuated<LitStr, Token![,]>>,
    pub(crate) no_reply: bool,
}

pub(crate) struct DBusItemAttributeSignal {
    pub(crate) name: Option<LitStr>,
}

pub(crate) struct DBusItemAttributeProperty {
    pub(crate) name: Option<LitStr>,
    pub(crate) access: DBusPropertyAccess,
    pub(crate) emits_changed_signal: Option<EmitsChangedSignal>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DBusPropertyAccess {
    Read,
    Write,
    ReadWrite,
}

impl DBusPropertyAccess {
    pub(crate) fn or(self, other: DBusPropertyAccess) -> Self {
        if self == other {
            self
        } else {
            DBusPropertyAccess::ReadWrite
        }
    }
}

impl DBusItemAttribute {
    pub(crate) fn parse(attrs: &[Attribute], span: Span) -> syn::Result<Self> {
        let tag = DBusItemAttributeTag::parse(attrs)?;
        match tag {
            None => DBusItemAttributeMethod::parse(attrs).map(Self::Method),
            Some(DBusItemAttributeTag::Signal) => {
                DBusItemAttributeSignal::parse(attrs).map(Self::Signal)
            }
            Some(DBusItemAttributeTag::Property) => {
                DBusItemAttributeProperty::parse(attrs, span).map(Self::Property)
            }
        }
    }
}

impl DBusItemAttributeMethod {
    fn parse(attrs: &[Attribute]) -> syn::Result<Self> {
        let mut name: Option<LitStr> = None;
        let mut out_args: Option<Punctuated<LitStr, Token![,]>> = None;
        let mut no_reply = false;
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
                    } else if meta.path.is_ident("no_reply") {
                        no_reply = true;
                        Ok(())
                    } else {
                        Err(meta.error(format!(
                            "unknown attribute `{}`. Possible attributes are `name`, `out_args`, `no_reply`",
                            meta.path.get_ident().unwrap(),
                        )))
                    }
                })?;
            }
        }
        Ok(Self {
            name,
            out_args,
            no_reply,
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

impl DBusItemAttributeProperty {
    fn parse(attrs: &[Attribute], span: Span) -> syn::Result<Self> {
        let mut name: Option<LitStr> = None;
        let mut access: Option<DBusPropertyAccess> = None;
        let mut emits_changed_signal: Option<EmitsChangedSignal> = None;
        for attr in attrs {
            if attr.path().is_ident(ATTRIBUTE_NAME) {
                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("property") {
                        Ok(())
                    } else if meta.path.is_ident("name") {
                        name = Some(meta.value()?.parse()?);
                        Ok(())
                    } else if meta.path.is_ident("get") {
                        access = Some(access
                            .unwrap_or(DBusPropertyAccess::Read)
                            .or(DBusPropertyAccess::Read));
                        Ok(())
                    } else if meta.path.is_ident("set") {
                        access = Some(access
                            .unwrap_or(DBusPropertyAccess::Write)
                            .or(DBusPropertyAccess::Write));
                        Ok(())
                    } else if meta.path.is_ident("emits_changed_signal") {
                        emits_changed_signal = Some(meta.value()?.parse()?);
                        Ok(())
                    } else {
                        Err(meta.error(format!(
                            "unknown attribute `{}`. Possible attributes are `name`, `get`, `set`, `emits_changed_signal`",
                            meta.path.get_ident().unwrap(),
                        )))
                    }
                })?;
            }
        }
        let Some(access) = access else {
            return Err(syn::Error::new(
                span,
                "either the attribute `get` or `set` attribute (or both) must be specified",
            ));
        };
        Ok(Self {
            name,
            access,
            emits_changed_signal,
        })
    }
}

enum DBusItemAttributeTag {
    Signal,
    Property,
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
                    } else if first_nested.path().is_ident("property") {
                        tag = Some(Self::Property);
                        first_nested.require_path_only()?;
                    }
                }
            }
        }
        Ok(tag)
    }
}
