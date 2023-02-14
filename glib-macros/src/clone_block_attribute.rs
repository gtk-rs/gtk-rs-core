// Take a look at the license at the top of the repository in the LICENSE file.

use deluxe::{Errors, HasAttributes};
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote, quote_spanned, ToTokens};
use std::collections::HashSet;
use std::rc::Rc;
use syn::{
    parse::{ParseStream, Parser},
    parse_quote, parse_quote_spanned,
    spanned::Spanned,
    visit_mut::VisitMut,
};

enum Capture {
    Strong {
        span: Span,
        ident: Option<syn::Ident>,
        from: Option<syn::Expr>,
    },
    Weak {
        span: Span,
        ident: Option<syn::Ident>,
        from: Option<syn::Expr>,
        or: Option<Rc<UpgradeFailAction>>,
    },
    Watch {
        span: Span,
        ident: Option<syn::Ident>,
        from: Option<syn::Expr>,
    },
}

#[derive(Clone, deluxe::ParseMetaItem, Default)]
#[deluxe(default)]
enum UpgradeFailAction {
    #[default]
    #[deluxe(skip)]
    Unspecified,
    #[deluxe(skip)]
    Default(syn::Expr),
    #[deluxe(rename = default_allow_none)]
    AllowNone,
    #[deluxe(rename = default_panic)]
    Panic,
    #[deluxe(rename = default_return, transparent)]
    Return(Option<syn::Expr>),
}

impl UpgradeFailAction {
    #[inline]
    fn is_unspecified(&self) -> bool {
        matches!(self, UpgradeFailAction::Unspecified)
    }
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
enum Mode {
    Clone,
    Closure,
    ClosureAsync,
}

fn is_simple_expr(mut expr: &syn::Expr) -> bool {
    // try to do some looping to avoid blowing up the stack
    loop {
        match expr {
            syn::Expr::Cast(e) => expr = &e.expr,
            syn::Expr::Field(e) => expr = &e.base,
            syn::Expr::Index(e) => return is_simple_expr(&e.expr) && is_simple_expr(&e.index),
            syn::Expr::Lit(_) => return true,
            syn::Expr::Paren(e) => expr = &e.expr,
            syn::Expr::Path(_) => return true,
            syn::Expr::Reference(e) => expr = &e.expr,
            syn::Expr::Type(e) => expr = &e.expr,
            _ => return false,
        }
    }
}

impl Capture {
    fn ident(&self) -> Option<&syn::Ident> {
        match self {
            Self::Strong { ident, .. } => ident.as_ref(),
            Self::Weak { ident, .. } => ident.as_ref(),
            Self::Watch { ident, .. } => ident.as_ref(),
        }
    }
    fn set_default_fail(&mut self, action: &Rc<UpgradeFailAction>) {
        if let Self::Weak { or, .. } = self {
            if or.is_none() {
                *or = Some(action.clone());
            }
        }
    }
    fn outer_tokens(&self, index: usize, glib: &syn::Path) -> Option<TokenStream> {
        Some(match self {
            Self::Strong { ident, from, .. } => {
                let target = format_ident!("____strong{}", index, span = Span::mixed_site());
                let input = from
                    .as_ref()
                    .map(|f| f.to_token_stream())
                    .or_else(|| Some(ident.as_ref()?.to_token_stream()))?;
                quote! { let #target = ::std::clone::Clone::clone(&#input); }
            }
            Self::Weak { ident, from, .. } => {
                let target = format_ident!("____weak{}", index, span = Span::mixed_site());
                let input = from
                    .as_ref()
                    .map(|f| f.to_token_stream())
                    .or_else(|| Some(ident.as_ref()?.to_token_stream()))?;
                quote! { let #target = #glib::clone::Downgrade::downgrade(&#input); }
            }
            Self::Watch { ident, from, .. } => {
                let target = format_ident!("____watch{}", index, span = Span::mixed_site());
                let input = from
                    .as_ref()
                    .map(|f| f.to_token_stream())
                    .or_else(|| Some(ident.as_ref()?.to_token_stream()))?;
                if from.as_ref().map(is_simple_expr).unwrap_or(true) {
                    quote! {
                        let #target = #glib::object::Watchable::watched_object(&#input);
                    }
                } else {
                    let watch_ident = syn::Ident::new("____watch", Span::mixed_site());
                    quote! {
                        let #watch_ident = ::std::clone::Clone::clone(&#input);
                        let #target = #glib::object::Watchable::watched_object(&#watch_ident);
                    }
                }
            }
        })
    }
    fn rename_tokens(&self, index: usize) -> Option<TokenStream> {
        Some(match self {
            Self::Strong { ident, .. } => {
                let ident = ident.as_ref()?;
                let input = format_ident!("____strong{}", index, span = Span::mixed_site());
                quote! { let #ident = #input; }
            }
            _ => return None,
        })
    }
    fn inner_tokens(&self, index: usize, mode: Mode, glib: &syn::Path) -> Option<TokenStream> {
        Some(match self {
            Self::Strong { .. } => return None,
            Self::Weak { ident, or, .. } => {
                let ident = ident.as_ref()?;
                let input = format_ident!("____weak{}", index, span = Span::mixed_site());
                let upgrade = quote! { #glib::clone::Upgrade::upgrade(&#input) };
                let upgrade = match or.as_ref().map(|or| or.as_ref()) {
                    None | Some(UpgradeFailAction::AllowNone) => upgrade,
                    Some(or) => {
                        let action = match or {
                            UpgradeFailAction::Panic => {
                                let name = ident.to_string();
                                quote! { ::std::panic!("Failed to upgrade `{}`", #name) }
                            }
                            UpgradeFailAction::Default(expr) => expr.to_token_stream(),
                            UpgradeFailAction::Return(expr) => {
                                if mode != Mode::Clone {
                                    quote! {
                                        return #glib::closure::ToClosureReturnValue::to_closure_return_value(
                                            &#expr
                                        )
                                    }
                                } else {
                                    quote! { return #expr }
                                }
                            }
                            _ => unreachable!(),
                        };
                        quote_spanned! { Span::mixed_site() =>
                            match #upgrade {
                                ::std::option::Option::Some(v) => v,
                                ::std::option::Option::None => #action
                            }
                        }
                    }
                };
                quote! { let #ident = #upgrade;  }
            }
            Self::Watch { ident, .. } => {
                if mode == Mode::ClosureAsync {
                    return None;
                }
                let ident = ident.as_ref()?;
                let input = format_ident!("____watch{}", index, span = Span::mixed_site());
                quote! {
                    let #ident = unsafe { #input.borrow() };
                    let #ident = ::std::convert::AsRef::as_ref(&#ident);
                }
            }
        })
    }
    fn async_inner_tokens(&self, index: usize) -> Option<TokenStream> {
        Some(match self {
            Self::Strong { ident, .. } => {
                ident.as_ref()?;
                quote! { let #ident = ::std::clone::Clone::clone(&#ident); }
            }
            Self::Weak { ident, .. } => {
                ident.as_ref()?;
                let input = format_ident!("____weak{}", index, span = Span::mixed_site());
                quote! { let #input = ::std::clone::Clone::clone(&#input); }
            }
            Self::Watch { ident, .. } => {
                let ident = ident.as_ref()?;
                let input = format_ident!("____watch{}", index, span = Span::mixed_site());
                quote! {
                    let #ident = ::std::clone::Clone::clone(unsafe { &*#input.borrow() });
                }
            }
        })
    }
    fn after_tokens(&self, glib: &syn::Path) -> Option<TokenStream> {
        Some(match self {
            Self::Watch { ident, from, .. } if ident.is_some() || from.is_some() => {
                let closure_ident = syn::Ident::new("____closure", Span::mixed_site());
                if from.as_ref().map(is_simple_expr).unwrap_or(true) {
                    let input = from
                        .as_ref()
                        .map(|f| f.to_token_stream())
                        .or_else(|| Some(ident.as_ref()?.to_token_stream()))?;
                    quote! {
                        #glib::object::Watchable::watch_closure(&#input, &#closure_ident);
                    }
                } else {
                    let watch_ident = syn::Ident::new("____watch", Span::mixed_site());
                    quote! {
                        #glib::object::ObjectExt::watch_closure(&#watch_ident, &#closure_ident);
                    }
                }
            }
            _ => return None,
        })
    }
}

impl Spanned for Capture {
    fn span(&self) -> Span {
        match self {
            Self::Strong { span, .. } => *span,
            Self::Weak { span, .. } => *span,
            Self::Watch { span, .. } => *span,
        }
    }
}

impl deluxe::ParseMetaItem for Capture {
    fn parse_meta_item(_input: ParseStream, _mode: deluxe::ParseMode) -> deluxe::Result<Self> {
        unreachable!()
    }
    fn parse_meta_item_named(input: ParseStream, name: &str, span: Span) -> deluxe::Result<Self> {
        #[inline]
        fn parse_capture<T: Default>(
            input: ParseStream,
            func: impl FnOnce(ParseStream) -> syn::Result<T>,
        ) -> syn::Result<(T, Option<syn::Ident>)> {
            let t = input
                .peek(syn::token::Paren)
                .then(|| func(input))
                .transpose()?
                .unwrap_or_default();
            let ident = if input.peek(syn::Token![_]) {
                input.parse::<syn::Token![_]>()?;
                None
            } else {
                Some(input.parse()?)
            };
            Ok((t, ident))
        }
        match name {
            "strong" => {
                let (from, ident) = parse_capture(input, parse_strong)?;
                Ok(Capture::Strong { span, ident, from })
            }
            "weak" => {
                let ((from, or), ident) = parse_capture(input, parse_weak)?;
                Ok(Capture::Weak {
                    span,
                    ident,
                    from,
                    or: or.map(Rc::new),
                })
            }
            "watch" => {
                let (from, ident) = parse_capture(input, parse_strong)?;
                Ok(Capture::Watch { span, ident, from })
            }
            _ => unreachable!(),
        }
    }
}

fn extract_idents<'p>(pat: &'p syn::Pat, idents: &mut HashSet<&'p syn::Ident>) {
    use syn::Pat::*;
    match pat {
        Box(p) => extract_idents(&p.pat, idents),
        Ident(p) => {
            idents.insert(&p.ident);
        }
        Or(p) => p.cases.iter().for_each(|p| extract_idents(p, idents)),
        Reference(p) => extract_idents(&p.pat, idents),
        Slice(p) => p.elems.iter().for_each(|p| extract_idents(p, idents)),
        Struct(p) => p.fields.iter().for_each(|p| extract_idents(&p.pat, idents)),
        Tuple(p) => p.elems.iter().for_each(|p| extract_idents(p, idents)),
        TupleStruct(p) => p.pat.elems.iter().for_each(|p| extract_idents(p, idents)),
        Type(p) => extract_idents(&p.pat, idents),
        _ => {}
    }
}

mod keywords {
    syn::custom_keyword!(or);
    syn::custom_keyword!(or_panic);
    syn::custom_keyword!(or_return);
    syn::custom_keyword!(allow_none);
}

#[derive(Default, deluxe::ExtractAttributes)]
#[deluxe(attributes(clone))]
struct CloneAttrs {
    #[deluxe(append, rename = strong, alias = weak)]
    captures: Vec<Capture>,
    #[deluxe(flatten)]
    or: UpgradeFailAction,
}

#[derive(Default, deluxe::ExtractAttributes)]
#[deluxe(attributes(closure))]
struct ClosureAttrs {
    local: deluxe::Flag,
    #[deluxe(append, rename = strong, alias = weak, alias = watch)]
    captures: Vec<Capture>,
    #[deluxe(flatten)]
    or: UpgradeFailAction,
}

fn parse_strong(input: syn::parse::ParseStream<'_>) -> syn::Result<Option<syn::Expr>> {
    if input.is_empty() {
        return Ok(None);
    }
    let content;
    syn::parenthesized!(content in input);
    if content.is_empty() {
        return Ok(None);
    }
    let expr = content.parse()?;
    content.parse::<syn::parse::Nothing>()?;
    Ok(Some(expr))
}

#[inline]
fn has_expr(input: syn::parse::ParseBuffer) -> bool {
    // check if only one token
    if input.peek(keywords::or_panic) || input.peek(keywords::allow_none) {
        let _ = input.step(|c| Ok(((), c.token_tree().unwrap().1)));
        if input.is_empty() {
            return false;
        }
    }
    // check if only one token and one expr
    if input.peek(keywords::or) || input.peek(keywords::or_return) {
        let _ = input.step(|c| Ok(((), c.token_tree().unwrap().1)));
        if input.is_empty() {
            return false;
        }
        if input.parse::<syn::Expr>().is_err() {
            return false;
        }
        if input.is_empty() {
            return false;
        }
    }
    true
}

fn parse_weak(
    input: syn::parse::ParseStream<'_>,
) -> syn::Result<(Option<syn::Expr>, Option<UpgradeFailAction>)> {
    if input.is_empty() {
        return Ok((None, None));
    }
    let content;
    syn::parenthesized!(content in input);
    if content.is_empty() {
        return Ok((None, None));
    }
    let expr = if has_expr(content.fork()) {
        Some(content.parse()?)
    } else {
        None
    };
    let lookahead = content.lookahead1();
    let fail_action = if lookahead.peek(keywords::or) {
        content.parse::<keywords::or>()?;
        let ret = content.parse()?;
        Some(UpgradeFailAction::Default(ret))
    } else if lookahead.peek(keywords::or_panic) {
        content.parse::<keywords::or_panic>()?;
        Some(UpgradeFailAction::Panic)
    } else if lookahead.peek(keywords::or_return) {
        content.parse::<keywords::or_return>()?;
        let ret = if content.is_empty() {
            None
        } else {
            Some(content.parse()?)
        };
        Some(UpgradeFailAction::Return(ret))
    } else if lookahead.peek(keywords::allow_none) {
        content.parse::<keywords::allow_none>()?;
        Some(UpgradeFailAction::AllowNone)
    } else if content.is_empty() {
        None
    } else {
        return Err(lookahead.error());
    };
    content.parse::<syn::parse::Nothing>()?;
    Ok((expr, fail_action))
}

fn has_captures<'p>(mut inputs: impl Iterator<Item = &'p syn::Pat>) -> bool {
    inputs.any(|pat| {
        pat.attrs()
            .iter()
            .any(|a| a.path.is_ident("strong") || a.path.is_ident("weak"))
    })
}

fn extract_attr<T: HasAttributes>(attrs: &mut T, name: &str) -> Option<syn::Attribute> {
    let attrs = attrs.attrs_mut().ok()?;
    let attr_index = attrs.iter().position(|a| a.path.is_ident(name));
    attr_index.map(|attr_index| attrs.remove(attr_index))
}

struct Visitor<'v> {
    crate_path: &'v syn::Path,
    errors: &'v Errors,
}

impl<'v> Visitor<'v> {
    fn create_gclosure(&mut self, closure: &syn::ExprClosure) -> Option<syn::Expr> {
        let has_closure = closure.attrs.iter().any(|a| a.path.is_ident("closure"));
        let has_watch = closure
            .inputs
            .iter()
            .any(|pat| pat.attrs().iter().any(|a| a.path.is_ident("watch")));
        if !has_closure && !has_watch {
            return None;
        }

        let mut attrs = closure.attrs.clone();
        let ClosureAttrs {
            local,
            mut captures,
            or: mut action,
        } = deluxe::extract_attributes_optional(&mut attrs, self.errors);
        let local = !has_closure || local.is_set();

        let mode = match closure.body.as_ref() {
            syn::Expr::Async(_) => Mode::ClosureAsync,
            _ => Mode::Closure,
        };
        let mut inputs = closure.inputs.iter().cloned().collect::<Vec<_>>();
        if let Some(caps) = self.get_captures(&mut inputs, mode) {
            captures.extend(caps);
        }
        self.extract_default_fail_action(&mut attrs, &mut action);
        if !action.is_unspecified() {
            let action = Rc::new(action);
            for capture in &mut captures {
                capture.set_default_fail(&action);
            }
        }
        if !captures.is_empty() && closure.capture.is_none() {
            self.errors.push_spanned(
                closure,
                "Closure must be `move` to use #[watch] or #[strong] or #[weak]",
            );
        }
        self.validate_captures(&captures, &inputs);

        let mut rest_index = None;
        for (index, pat) in inputs.iter_mut().enumerate() {
            if let Ok(attrs) = pat.attrs_mut() {
                if let Some(attr) = extract_attr(attrs, "rest") {
                    if !attr.tokens.is_empty() {
                        self.errors.push_spanned(
                            &attr.tokens,
                            format!(
                                "Unknown tokens on #[{}] attribute",
                                attr.path.to_token_stream(),
                            ),
                        );
                    }
                    rest_index = Some(index);
                    break;
                }
            }
        }
        if let Some(rest_index) = rest_index {
            while inputs.len() > rest_index + 1 {
                let pat = inputs.remove(rest_index + 1);
                self.errors
                    .push_spanned(pat, "Arguments not allowed past #[rest] parameter");
            }
        }

        let glib = self.crate_path;
        let closure_ident = syn::Ident::new("____closure", Span::mixed_site());
        let values_ident = syn::Ident::new("____values", Span::mixed_site());
        let constructor = if local {
            format_ident!("new_local")
        } else {
            format_ident!("new")
        };
        let outer = captures
            .iter()
            .enumerate()
            .map(|(i, c)| c.outer_tokens(i, glib));
        let rename = captures.iter().enumerate().map(|(i, c)| c.rename_tokens(i));
        let inner = captures
            .iter()
            .enumerate()
            .map(|(i, c)| c.inner_tokens(i, mode, glib));
        let after = captures.iter().map(|c| c.after_tokens(glib));
        let required_arg_count = inputs
            .iter()
            .enumerate()
            .rev()
            .find_map(|(i, p)| {
                (Some(i) != rest_index && !matches!(p, syn::Pat::Wild(_))).then_some(i + 1)
            })
            .unwrap_or(0);
        let assert_arg_count = (required_arg_count > 0).then(|| {
            quote! {
                if #values_ident.len() < #required_arg_count {
                    ::std::panic!(
                        "Closure called with wrong number of arguments: Expected {}, got {}",
                        #required_arg_count,
                        #values_ident.len(),
                    );
                }
            }
        });
        let arg_unwraps = inputs.iter().enumerate().map(|(index, pat)| match pat {
            syn::Pat::Wild(_) => None,
            _ => {
                let attrs = pat.attrs();
                Some(if Some(index) == rest_index {
                    quote! {
                        #(#attrs)*
                        let #pat = &#values_ident[#index..#values_ident.len()];
                    }
                } else {
                    quote! {
                        #(#attrs)*
                        let #pat = #glib::Value::get(&#values_ident[#index])
                            .unwrap_or_else(|e| {
                                ::std::panic!("Wrong type for closure argument {}: {:?}", #index, e)
                            });
                    }
                })
            }
        });
        let expr = &closure.body;
        let inner_body = quote! { {
            #assert_arg_count
            #(#inner)*
            #(#arg_unwraps)*
            #expr
        } };
        let body = if mode == Mode::ClosureAsync {
            let async_inner = captures
                .iter()
                .enumerate()
                .map(|(i, c)| c.async_inner_tokens(i));
            quote! {
                let #values_ident = #values_ident.to_vec();
                #(#async_inner)*
                #glib::MainContext::default().spawn_local(
                    async move { let _: () = #inner_body.await; }
                );
                ::std::option::Option::None
            }
        } else {
            let inner_body = match &closure.output {
                syn::ReturnType::Type(_, ty) => {
                    let ret = syn::Ident::new("____ret", Span::mixed_site());
                    quote! {
                        {
                            let #ret: #ty = #inner_body;
                            #ret
                        }
                    }
                }
                _ => quote! { #inner_body },
            };
            quote! {
                #glib::closure::IntoClosureReturnValue::into_closure_return_value(
                    #inner_body
                )
            }
        };
        Some(parse_quote_spanned! { Span::mixed_site() =>
            {
                #(#outer)*
                #(#rename)*
                let #closure_ident = #glib::closure::RustClosure::#constructor(move |#values_ident| {
                    #body
                });
                #(#after)*
                #closure_ident
            }
        })
    }

    fn create_closure(&mut self, closure: &syn::ExprClosure) -> Option<syn::Expr> {
        let has_clone = closure.attrs.iter().any(|a| a.path.is_ident("clone"));
        if !has_clone && !has_captures(closure.inputs.iter()) {
            return None;
        }
        let mut attrs = closure.attrs.clone();
        let CloneAttrs {
            mut captures,
            or: mut action,
        } = deluxe::extract_attributes_optional(&mut attrs, self.errors);

        let mut inputs = closure.inputs.iter().cloned().collect::<Vec<_>>();
        if let Some(caps) = self.get_captures(&mut inputs, Mode::Clone) {
            captures.extend(caps);
        }
        self.validate_captures(&captures, &inputs);
        if closure.capture.is_none() {
            self.errors.push_spanned(
                closure,
                "Closure must be `move` to use #[strong] or #[weak]",
            );
        }
        self.extract_default_fail_action(&mut attrs, &mut action);
        if !action.is_unspecified() {
            let action = Rc::new(action);
            for capture in &mut captures {
                capture.set_default_fail(&action);
            }
        }
        let glib = self.crate_path;
        let outer = captures
            .iter()
            .enumerate()
            .map(|(i, c)| c.outer_tokens(i, glib));
        let rename = captures.iter().enumerate().map(|(i, c)| c.rename_tokens(i));
        let inner = captures
            .iter()
            .enumerate()
            .map(|(i, c)| c.inner_tokens(i, Mode::Clone, glib));
        let output;
        let body = if let syn::Expr::Async(syn::ExprAsync {
            attrs,
            capture,
            block,
            ..
        }) = &*closure.body
        {
            output = syn::ReturnType::Default;
            let block = match &closure.output {
                syn::ReturnType::Type(_, ty) => {
                    let ret = syn::Ident::new("____ret", Span::mixed_site());
                    quote! {
                        let #ret: #ty = #block;
                        #ret
                    }
                }
                _ => quote! { #block },
            };
            parse_quote! {
                #(#attrs)*
                async #capture {
                    #(#inner)*
                    #block
                }
            }
        } else {
            output = closure.output.clone();
            let old_body = &closure.body;
            parse_quote! {
                {
                    #(#inner)*
                    #old_body
                }
            }
        };
        let body = syn::ExprClosure {
            attrs,
            movability: closure.movability,
            asyncness: closure.asyncness,
            capture: closure.capture,
            or1_token: closure.or1_token,
            inputs: FromIterator::from_iter(inputs.into_iter()),
            or2_token: closure.or2_token,
            output,
            body: Box::new(body),
        };
        Some(parse_quote_spanned! { Span::mixed_site() =>
            {
                #(#outer)*
                #(#rename)*
                #body
            }
        })
    }

    fn create_async(&mut self, async_: &syn::ExprAsync) -> Option<syn::Expr> {
        let has_clone = async_.attrs.iter().any(|a| a.path.is_ident("clone"));
        if !has_clone {
            return None;
        }
        let mut attrs = async_.attrs.clone();
        let CloneAttrs {
            mut captures,
            or: mut action,
        } = deluxe::extract_attributes_optional(&mut attrs, self.errors);

        self.validate_captures(&captures, &[]);
        if async_.capture.is_none() {
            self.errors
                .push_spanned(async_, "Async block must be `move` to use #[clone]");
        }
        self.extract_default_fail_action(&mut attrs, &mut action);
        if !action.is_unspecified() {
            let action = Rc::new(action);
            for capture in &mut captures {
                capture.set_default_fail(&action);
            }
        }
        let glib = self.crate_path;
        let outer = captures
            .iter()
            .enumerate()
            .map(|(i, c)| c.outer_tokens(i, glib));
        let rename = captures.iter().enumerate().map(|(i, c)| c.rename_tokens(i));
        let inner = captures
            .iter()
            .enumerate()
            .map(|(i, c)| c.inner_tokens(i, Mode::Clone, glib));
        let block = &async_.block;
        let block = parse_quote! {
            {
                #(#inner)*
                #block
            }
        };
        let body = syn::ExprAsync {
            attrs,
            async_token: async_.async_token,
            capture: async_.capture,
            block,
        };
        Some(parse_quote_spanned! { Span::mixed_site() =>
            {
                #(#outer)*
                #(#rename)*
                #body
            }
        })
    }

    fn validate_pat_ident(&mut self, pat: syn::Pat) -> Option<syn::Ident> {
        match pat {
            syn::Pat::Ident(syn::PatIdent { ident, .. }) => Some(ident),
            _ => {
                self.errors
                    .push_spanned(pat, "Pattern for captured variable must be an identifier");
                None
            }
        }
    }

    fn validate_captures(&mut self, captures: &[Capture], inputs: &[syn::Pat]) {
        let mut has_watch = false;
        let mut names = HashSet::new();
        for pat in inputs {
            extract_idents(pat, &mut names);
        }
        for capture in captures {
            if let Capture::Watch { span, .. } = capture {
                if has_watch {
                    self.errors
                        .push(*span, "Only one #[watch] attribute is allowed on closure");
                } else {
                    has_watch = true;
                }
            }
            if let Some(ident) = capture.ident() {
                if names.contains(ident) {
                    self.errors.push_spanned(
                        ident,
                        format!(
                            "Identifier `{ident}` is used more than once in this parameter list",
                        ),
                    );
                } else {
                    names.insert(ident);
                }
            }
        }
    }

    fn get_captures(&mut self, inputs: &mut Vec<syn::Pat>, mode: Mode) -> Option<Vec<Capture>> {
        let mut captures = Vec::new();
        let mut index = 0;
        while index < inputs.len() {
            let mut strong = None;
            let mut weak = None;
            let mut watch = None;
            if let Ok(attrs) = inputs[index].attrs_mut() {
                if let Some(attr) = extract_attr(attrs, "strong") {
                    strong = Some(attr);
                } else if let Some(attr) = extract_attr(attrs, "weak") {
                    weak = Some(attr);
                } else if mode != Mode::Clone {
                    if let Some(attr) = extract_attr(attrs, "watch") {
                        watch = Some(attr);
                    }
                }
                if strong.is_some() || weak.is_some() || watch.is_some() {
                    for attr in attrs {
                        self.errors.push_spanned(
                            attr,
                            "Extra attributes not allowed on #[strong] or #[weak] or #[watch] capture",
                        );
                    }
                }
            }
            if let Some(strong) = strong {
                let span = strong.span();
                let from = parse_strong.parse2(strong.tokens).unwrap_or_else(|e| {
                    self.errors.push_syn(e);
                    None
                });
                let pat = inputs.remove(index);
                let ident = if matches!(pat, syn::Pat::Wild(_)) {
                    None
                } else {
                    self.validate_pat_ident(pat)
                };
                if ident.is_some() || from.is_some() {
                    captures.push(Capture::Strong { span, ident, from });
                } else {
                    self.errors.push(
                        span,
                        "capture must be named or provide a source expression using #[strong(...)]",
                    );
                }
            } else if let Some(weak) = weak {
                let span = weak.span();
                let (from, or) = parse_weak.parse2(weak.tokens).unwrap_or_else(|e| {
                    self.errors.push_syn(e);
                    (None, None)
                });
                let pat = inputs.remove(index);
                let ident = if matches!(pat, syn::Pat::Wild(_)) {
                    None
                } else {
                    self.validate_pat_ident(pat)
                };
                if ident.is_some() || from.is_some() {
                    captures.push(Capture::Weak {
                        span,
                        ident,
                        from,
                        or: or.map(Rc::new),
                    });
                } else {
                    self.errors.push(
                        span,
                        "capture must be named or provide a source expression using #[weak(...)]",
                    );
                }
            } else if let Some(watch) = watch {
                let span = watch.span();
                let from = parse_strong.parse2(watch.tokens).unwrap_or_else(|e| {
                    self.errors.push_syn(e);
                    None
                });
                let pat = inputs.remove(index);
                let ident = if matches!(pat, syn::Pat::Wild(_)) {
                    None
                } else {
                    self.validate_pat_ident(pat)
                };
                if ident.is_some() || from.is_some() {
                    captures.push(Capture::Watch { span, ident, from });
                } else {
                    self.errors.push(
                        span,
                        "capture must be named or provide a source expression using #[watch(...)]",
                    );
                }
            } else {
                index += 1;
            }
        }
        if captures.is_empty() {
            None
        } else {
            Some(captures)
        }
    }

    fn extract_default_fail_action(
        &mut self,
        attrs: &mut Vec<syn::Attribute>,
        action_out: &mut UpgradeFailAction,
    ) {
        loop {
            let action = if let Some(attr) = extract_attr(attrs, "default_panic") {
                let span = attr.span();
                if let Err(e) = syn::parse2::<syn::parse::Nothing>(attr.tokens) {
                    self.errors.push_syn(e);
                }
                Some((span, UpgradeFailAction::Panic))
            } else if let Some(attr) = extract_attr(attrs, "default_allow_none") {
                let span = attr.span();
                if let Err(e) = syn::parse2::<syn::parse::Nothing>(attr.tokens) {
                    self.errors.push_syn(e);
                }
                Some((span, UpgradeFailAction::AllowNone))
            } else if let Some(attr) = extract_attr(attrs, "default_return") {
                let span = attr.span();
                let ret = (|input: syn::parse::ParseStream<'_>| {
                    if input.is_empty() {
                        return Ok(None);
                    }
                    let content;
                    syn::parenthesized!(content in input);
                    let expr = content.parse::<syn::Expr>()?;
                    content.parse::<syn::parse::Nothing>()?;
                    input.parse::<syn::parse::Nothing>()?;
                    Ok(Some(expr))
                })
                .parse2(attr.tokens);
                match ret {
                    Ok(expr) => Some((span, UpgradeFailAction::Return(expr))),
                    Err(e) => {
                        self.errors.push_syn(e);
                        None
                    }
                }
            } else {
                None
            };
            if let Some((span, action)) = action {
                if !action_out.is_unspecified() {
                    self.errors.push(span, "Duplicate default action specified");
                }
                *action_out = action;
            } else {
                break;
            }
        }
    }
}

impl<'v> VisitMut for Visitor<'v> {
    fn visit_expr_mut(&mut self, expr: &mut syn::Expr) {
        let new_expr = if let syn::Expr::Closure(closure) = expr {
            syn::visit_mut::visit_expr_mut(self, closure.body.as_mut());
            self.create_gclosure(closure)
                .or_else(|| self.create_closure(closure))
        } else if let syn::Expr::Async(async_) = expr {
            self.create_async(async_)
        } else {
            syn::visit_mut::visit_expr_mut(self, expr);
            None
        };
        if let Some(new_expr) = new_expr {
            *expr = new_expr;
        }
    }
}

pub fn impl_clone_block(item: &mut syn::Item, crate_path: &syn::Path, errors: &Errors) {
    let mut visitor = Visitor { crate_path, errors };
    visitor.visit_item_mut(item);
}
