// Take a look at the license at the top of the repository in the LICENSE file.

use proc_macro2::Span;
use syn::LitStr;
use syn::parse::Parse;

#[derive(Clone)]
pub(crate) struct EmitsChangedSignal {
    pub(crate) kind: EmitsChangedSignalKind,
    pub(crate) span: Span,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum EmitsChangedSignalKind {
    True,
    Invalidates,
    Const,
    False,
}

impl Parse for EmitsChangedSignal {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        use EmitsChangedSignalKind::*;
        let lit: LitStr = input.parse()?;
        let value = lit.value();
        let kind = match value.as_str() {
            "true" => True,
            "invalidates" => Invalidates,
            "const" => Const,
            "false" => False,
            _ => {
                return Err(syn::Error::new(
                    lit.span(),
                    format!(
                        "unknown value {value:?}. Possible values are \"true\", \"invalidates\", \"const\", \"false\""
                    ),
                ));
            }
        };
        Ok(EmitsChangedSignal {
            kind,
            span: lit.span(),
        })
    }
}
