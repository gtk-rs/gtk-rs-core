// Take a look at the license at the top of the repository in the LICENSE file.

use std::collections::HashSet;

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::spanned::Spanned;

mod boxed;
#[allow(dead_code)]
mod boxed_inline {
    #[derive(deluxe::ParseMetaItem)]
    pub struct BoxedInline {
        copy: Option<syn::Expr>,
        free: Option<syn::Expr>,
        init: Option<syn::Expr>,
        copy_into: Option<syn::Expr>,
        clear: Option<syn::Expr>,
        r#type: Option<syn::Expr>,
        #[deluxe(flatten)]
        args: super::WrapperArgs,
    }
}
#[allow(dead_code)]
mod shared {
    #[derive(deluxe::ParseMetaItem)]
    pub struct Shared {
        r#ref: syn::Expr,
        unref: syn::Expr,
        r#type: Option<syn::Expr>,
        #[deluxe(flatten)]
        args: super::WrapperArgs,
    }
}
#[allow(dead_code)]
mod object {
    #[derive(deluxe::ParseMetaItem)]
    pub struct Object {
        #[deluxe(append)]
        extends: Vec<syn::Path>,
        #[deluxe(append)]
        implements: Vec<syn::Path>,
        r#type: syn::Expr,
        #[deluxe(flatten)]
        args: super::WrapperArgs,
    }
}
#[allow(dead_code)]
mod object_subclass {
    #[derive(deluxe::ParseMetaItem)]
    pub struct ObjectSubclass {
        #[deluxe(append)]
        extends: Vec<syn::Path>,
        #[deluxe(append)]
        implements: Vec<syn::Path>,
        #[deluxe(flatten)]
        args: super::WrapperArgs,
    }
}
#[allow(dead_code)]
mod interface {
    #[derive(deluxe::ParseMetaItem)]
    pub struct Interface {
        #[deluxe(append)]
        requires: Vec<syn::Path>,
        r#type: syn::Expr,
        #[deluxe(flatten)]
        args: super::WrapperArgs,
    }
}
#[allow(dead_code)]
mod object_interface {
    #[derive(deluxe::ParseMetaItem)]
    pub struct ObjectInterface {
        #[deluxe(append)]
        requires: Vec<syn::Path>,
        #[deluxe(flatten)]
        args: super::WrapperArgs,
    }
}

#[derive(deluxe::ParseMetaItem)]
struct WrapperArgs {
    #[deluxe(
        append,
        rename = skip_trait,
        map = |s: HashSet<syn::Ident>| s.into_iter().map(|s| s.to_string()).collect(),
    )]
    skipped_traits: HashSet<String>,
}

enum Member<'a> {
    Named(&'a syn::Ident),
    Unnamed(syn::Index),
}

impl<'a> Member<'a> {
    fn from_fields(fields: &'a syn::Fields) -> impl Iterator<Item = Member<'a>> {
        fields
            .iter()
            .enumerate()
            .map(|(index, field)| match &field.ident {
                Some(ident) => Member::Named(ident),
                None => Member::Unnamed(syn::Index {
                    index: index as u32,
                    span: field.ty.span(),
                }),
            })
    }
}

impl<'a> ToTokens for Member<'a> {
    #[inline]
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Named(ident) => ident.to_tokens(tokens),
            Self::Unnamed(index) => index.to_tokens(tokens),
        }
    }
}

fn construct_stmt(
    fields: &syn::Fields,
    first_func: impl FnOnce(Member<'_>) -> TokenStream,
    mut rest_func: impl FnMut(Member<'_>) -> TokenStream,
) -> TokenStream {
    let mut first_func = Some(first_func);
    let members = fields.iter().enumerate().map(move |(index, f)| {
        let member = match &f.ident {
            Some(ident) => Member::Named(ident),
            None => Member::Unnamed(syn::Index {
                index: index as u32,
                span: f.ty.span(),
            }),
        };
        let expr: TokenStream = if index == 0 {
            (first_func.take().unwrap())(member)
        } else {
            rest_func(member)
        };
        match &f.ident {
            Some(ident) => quote::quote_spanned! { f.ty.span() => #ident: #expr },
            None => expr,
        }
    });
    match fields {
        syn::Fields::Named(_) => quote::quote! { Self { #(#members),* } },
        syn::Fields::Unnamed(_) => quote::quote! { Self(#(#members),*) },
        _ => unreachable!(),
    }
}

#[inline]
fn unique_lifetime(name: &str, generics: &syn::Generics) -> syn::Lifetime {
    let mut ident = String::from(name);
    while generics.lifetimes().any(|l| l.lifetime.ident == ident) {
        ident.push('_');
    }
    ident.insert(0, '\'');
    syn::Lifetime::new(&ident, proc_macro2::Span::mixed_site())
}

#[inline]
fn insert_lifetime(name: &str, generics: &mut syn::Generics) -> syn::Lifetime {
    let lt = unique_lifetime(name, generics);
    generics.params.insert(0, syn::parse_quote! { #lt });
    lt
}

fn get_first_field_type_param(fields: &syn::Fields, type_index: usize) -> syn::Result<&syn::Type> {
    fields
        .iter()
        .next()
        .ok_or_else(|| fields.span())
        .and_then(|f| match &f.ty {
            syn::Type::Path(tp) => tp
                .path
                .segments
                .last()
                .ok_or_else(|| tp.span())
                .and_then(|s| match &s.arguments {
                    syn::PathArguments::AngleBracketed(ga) => ga
                        .args
                        .iter()
                        .nth(type_index)
                        .ok_or_else(|| ga.span())
                        .and_then(|a| match a {
                            syn::GenericArgument::Type(t) => Ok(t),
                            t => Err(t.span()),
                        }),
                    a => Err(a.span()),
                }),
            ty => Err(ty.span()),
        })
        .map_err(|span| {
            syn::Error::new(
                span,
                format_args!("First field missing type argument {type_index}"),
            )
        })
}

#[inline]
fn get_type_name(ty: &syn::Type) -> Option<&syn::Ident> {
    match ty {
        syn::Type::Path(tp) => tp.path.segments.last().map(|s| &s.ident),
        _ => None,
    }
}

fn add_repr_transparent(item: &mut syn::ItemStruct) {
    if item.fields.iter().skip(1).all(|f| {
        get_type_name(&f.ty)
            .map(|i| i == "PhantomData")
            .unwrap_or(false)
    }) {
        item.attrs.extend(
            syn::parse::Parser::parse_str(syn::Attribute::parse_outer, "#[repr(transparent)]")
                .unwrap(),
        );
    }
}

pub fn impl_wrapper(attr: TokenStream, mut item: syn::ItemStruct) -> syn::Result<TokenStream> {
    let first_field = match item.fields.iter().next() {
        Some(f) => f,
        None => {
            return Err(syn::Error::new_spanned(
                item,
                "Wrapper struct must have at least one field",
            ));
        }
    };
    let type_name = match get_type_name(&first_field.ty) {
        Some(n) => n,
        None => {
            return Err(syn::Error::new_spanned(
                item,
                "First field must be a type path",
            ))
        }
    };

    let mut tokens = match type_name.to_string().as_str() {
        "Boxed" => deluxe::parse2::<boxed::Boxed>(attr)?.into_token_stream(&mut item)?,
        _ => return Err(syn::Error::new_spanned(type_name, "Unknown wrapper type")),
    };
    item.to_tokens(&mut tokens);
    Ok(tokens)
}
