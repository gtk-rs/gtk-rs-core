// Take a look at the license at the top of the repository in the LICENSE file.

use crate::dbus_interface::attributes::{DBusItemAttributeMethod, DBusMethodArgumentAttribute};
use crate::dbus_interface::parse::is_deprecated;
use crate::utils::{ident_name, ident_name_as_lit_str};
use heck::ToPascalCase as _;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned as _;
use syn::{
    FnArg, Ident, LitStr, Pat, PatIdent, PatType, ReturnType, Token, TraitItemFn, Type, parse_quote,
};

#[derive(Clone)]
pub(crate) struct DBusMethod {
    pub(crate) item: TraitItemFn,
    pub(crate) dbus_name: LitStr,
    pub(crate) args: Vec<DBusMethodArg>,
    pub(crate) return_type: Box<Type>,
    pub(crate) out_arg_names: Option<Punctuated<LitStr, Token![,]>>,
    pub(crate) no_reply: bool,
    pub(crate) deprecated: bool,
}

#[derive(Clone)]
pub(crate) struct DBusMethodArg {
    pub(crate) syn: PatType,
    pub(crate) ident: PatIdent,
    pub(crate) dbus_name: LitStr,
}

impl DBusMethod {
    pub(super) fn parse(
        item: TraitItemFn,
        attr: DBusItemAttributeMethod,
    ) -> syn::Result<DBusMethod> {
        let dbus_name = attr
            .name
            .unwrap_or_else(|| default_method_name(&item.sig.ident));
        let args = item
            .sig
            .inputs
            .iter()
            .cloned()
            .filter_map(|arg| parse_method_arg(arg).transpose())
            .collect::<Result<_, _>>()?;
        let deprecated = is_deprecated(&item.attrs);
        let return_type = parse_return_type(&item.sig.output);
        Ok(DBusMethod {
            item,
            dbus_name,
            args,
            return_type,
            out_arg_names: attr.out_args,
            no_reply: attr.no_reply,
            deprecated,
        })
    }
}

fn parse_method_arg(arg: FnArg) -> syn::Result<Option<DBusMethodArg>> {
    let FnArg::Typed(typed) = arg else {
        return Ok(None);
    };
    let attr = DBusMethodArgumentAttribute::parse(&typed.attrs)?;
    let Pat::Ident(ident) = &*typed.pat else {
        return Err(syn::Error::new(
            typed.pat.span(),
            "only named arguments are allowed",
        ));
    };
    let dbus_name = attr
        .name
        .unwrap_or_else(|| default_parameter_name(&ident.ident));
    Ok(Some(DBusMethodArg {
        ident: ident.clone(),
        syn: typed,
        dbus_name,
    }))
}

pub(super) fn default_method_name(ident: &Ident) -> LitStr {
    let ident_name = ident_name(ident);
    LitStr::new(&ident_name.to_pascal_case(), ident.span())
}

fn default_parameter_name(ident: &Ident) -> LitStr {
    ident_name_as_lit_str(ident)
}

fn parse_return_type(return_: &ReturnType) -> Box<Type> {
    match return_ {
        ReturnType::Default => parse_quote!(()),
        ReturnType::Type(_, t) => t.clone(),
    }
}
