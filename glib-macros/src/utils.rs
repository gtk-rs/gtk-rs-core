// Take a look at the license at the top of the repository in the LICENSE file.

use anyhow::{bail, Result};
use proc_macro2::{Ident, Span, TokenStream};
use proc_macro_crate::crate_name;
use quote::{quote, quote_spanned};
use syn::{
    punctuated::Punctuated, spanned::Spanned, token::Comma, Attribute, DeriveInput, Lit, Meta,
    MetaList, NestedMeta, Variant,
};

// find the #[@attr_name] attribute in @attrs
pub fn find_attribute_meta(attrs: &[Attribute], attr_name: &str) -> Result<Option<MetaList>> {
    let meta = match attrs.iter().find(|a| a.path.is_ident(attr_name)) {
        Some(a) => a.parse_meta(),
        _ => return Ok(None),
    };
    match meta? {
        Meta::List(n) => Ok(Some(n)),
        _ => bail!("wrong meta type"),
    }
}

// parse a single meta like: ident = "value"
fn parse_attribute(meta: &NestedMeta) -> Result<(String, String)> {
    let meta = match &meta {
        NestedMeta::Meta(m) => m,
        _ => bail!("wrong meta type"),
    };
    let meta = match meta {
        Meta::NameValue(n) => n,
        _ => bail!("wrong meta type"),
    };
    let value = match &meta.lit {
        Lit::Str(s) => s.value(),
        _ => bail!("wrong meta type"),
    };

    let ident = match meta.path.get_ident() {
        None => bail!("missing ident"),
        Some(ident) => ident,
    };

    Ok((ident.to_string(), value))
}

pub fn find_nested_meta<'a>(meta: &'a MetaList, name: &str) -> Option<&'a NestedMeta> {
    meta.nested.iter().find(|n| match n {
        NestedMeta::Meta(m) => m.path().is_ident(name),
        _ => false,
    })
}

pub fn parse_name_attribute(meta: &NestedMeta) -> Result<String> {
    let (ident, v) = parse_attribute(meta)?;

    match ident.as_ref() {
        "name" => Ok(v),
        s => bail!("Unknown meta {}", s),
    }
}

// Parse attribute such as:
// #[enum_type(name = "TestAnimalType")]
pub fn parse_name(input: &DeriveInput, attr_name: &str) -> Result<String> {
    let meta = match find_attribute_meta(&input.attrs, attr_name)? {
        Some(meta) => meta,
        _ => bail!("Missing '{}' attribute", attr_name),
    };

    let meta = match find_nested_meta(&meta, "name") {
        Some(meta) => meta,
        _ => bail!("Missing meta 'name'"),
    };

    parse_name_attribute(meta)
}

#[derive(Debug)]
pub enum ItemAttribute {
    Name(String),
    Nick(String),
}

fn parse_item_attribute(meta: &NestedMeta) -> Result<ItemAttribute> {
    let (ident, v) = parse_attribute(meta)?;

    match ident.as_ref() {
        "name" => Ok(ItemAttribute::Name(v)),
        "nick" => Ok(ItemAttribute::Nick(v)),
        s => bail!("Unknown item meta {}", s),
    }
}

// Parse optional enum item attributes such as:
// #[enum_value(name = "My Name", nick = "my-nick")]
pub fn parse_item_attributes(attr_name: &str, attrs: &[Attribute]) -> Result<Vec<ItemAttribute>> {
    let meta = find_attribute_meta(attrs, attr_name)?;

    let v = match meta {
        Some(meta) => meta
            .nested
            .iter()
            .map(parse_item_attribute)
            .collect::<Result<Vec<_>, _>>()?,
        None => Vec::new(),
    };

    Ok(v)
}

pub fn crate_ident_new() -> TokenStream {
    use proc_macro_crate::FoundCrate;

    match crate_name("glib") {
        Ok(FoundCrate::Name(name)) => Some(name),
        Ok(FoundCrate::Itself) => Some("glib".to_string()),
        Err(_) => None,
    }
    .map(|s| {
        let glib = Ident::new(&s, Span::call_site());
        quote!(#glib)
    })
    .unwrap_or_else(|| {
        // We couldn't find the glib crate (renamed or not) so let's just hope it's in scope!
        //
        // We will be able to have this information once this code is stable:
        //
        // ```
        // let span = Span::call_site();
        // let source = span.source_file();
        // let file_path = source.path();
        // ```
        //
        // Then we can use proc_macro to parse the file and check if glib is imported somehow.
        let glib = Ident::new("glib", Span::call_site());
        quote!(#glib)
    })
}

// Generate i32 to enum mapping, used to implement
// glib::translate::TryFromGlib<i32>, such as:
//
//   if value == Animal::Goat as i32 {
//       return Some(Animal::Goat);
//   }
pub fn gen_enum_from_glib(
    enum_name: &Ident,
    enum_variants: &Punctuated<Variant, Comma>,
) -> TokenStream {
    // FIXME: can we express this with a match()?
    let recurse = enum_variants.iter().map(|v| {
        let name = &v.ident;
        quote_spanned! { v.span() =>
            if value == #enum_name::#name as i32 {
                return Some(#enum_name::#name);
            }
        }
    });
    quote! {
        #(#recurse)*
        None
    }
}
