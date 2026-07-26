//! This module cotnains the parsing for the `ident_str` attribute macro

// IMPLEMENTATION NOTE:
//
// The implementation of this macro is just to expand into the `ident_str_def` macro.  This is
// required for one main reason: in order for this crate to be useful, we need access to the macro
// parameters, but in order to get those the macro needs to expand.
//
// The user code ends up being
//     ident_str (#var = $foo) { MacroDefinition($foo) { body } }
// which expands to
//     MacroDefintion($foo) { ident_str_def(#var = $foo) { body } }
// and then when a user uses the macro they define, it expands into
//     ident_str_def(#var = <$foo expanded>) { body }
// with the last step expanding #var into just
//     body

use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    Token, braced, parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::{Brace, Paren},
};

#[derive(Debug)]
struct Def {
    _let_tok: Token![let],
    hash: Token![#],
    name: syn::Ident,
    eq: Token![=],
    value: syn::Expr,
    _semi: Token![;],
}

impl Parse for Def {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            _let_tok: input.parse()?,
            hash: input.parse()?,
            name: input.parse()?,
            eq: input.parse()?,
            value: input.parse()?,
            _semi: input.parse()?,
        })
    }
}

#[derive(Debug)]
struct MacroBranch {
    pat_parens: Paren,
    pat: TokenStream,
    arrow: Token![=>],
    def: Vec<Def>,
    body_brace: Brace,
    body: TokenStream,
}

impl Parse for MacroBranch {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let pat;
        let def;
        Ok(Self {
            pat_parens: parenthesized!(pat in input),
            pat: pat.parse()?,
            arrow: input.parse()?,
            def: {
                let mut out = Vec::new();
                loop {
                    let la = input.lookahead1();
                    if la.peek(Token![let]) {
                        out.push(input.parse()?);
                    } else if la.peek(syn::token::Brace) {
                        break out;
                    } else {
                        return Err(la.error());
                    }
                }
            },
            body_brace: braced!(def in input),
            body: def.parse()?,
        })
    }
}

impl ToTokens for MacroBranch {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.pat_parens
            .surround(tokens, |ts| self.pat.to_tokens(ts));
        self.arrow.to_tokens(tokens);
        self.body_brace.surround(tokens, |ts| {
            let body = &self.body;
            let mut defs = TokenStream::new();
            for d in &self.def {
                d.hash.to_tokens(&mut defs);
                d.name.to_tokens(&mut defs);
                d.eq.to_tokens(&mut defs);
                d.value.to_tokens(&mut defs);
                <Token![,]>::default().to_tokens(&mut defs);
            }
            quote! {
                ::ident_str::ident_str_def! {
                    #defs
                    => {
                        #body
                    }
                }
            }
            .to_tokens(ts)
        });
    }
}

struct MacroDef {
    macro_rules: syn::Ident,
    bang: Token![!],
    name: syn::Ident,
    body_brace: Brace,
    body: Punctuated<MacroBranch, Token![;]>,
}

impl Parse for MacroDef {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let body;
        Ok(Self {
            macro_rules: {
                let ident: syn::Ident = input.parse()?;
                if ident != "macro_rules" {
                    return Err(syn::Error::new(ident.span(), "Expected macro_rules"));
                }
                ident
            },
            bang: input.parse()?,
            name: input.parse()?,
            body_brace: braced!(body in input),
            body: Punctuated::parse_terminated(&body)?,
        })
    }
}

impl ToTokens for MacroDef {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.macro_rules.to_tokens(tokens);
        self.bang.to_tokens(tokens);
        self.name.to_tokens(tokens);
        self.body_brace.surround(tokens, |ts| {
            self.body.to_tokens(ts);
        });
    }
}

pub(crate) fn ident_str(input: ParseStream) -> syn::Result<TokenStream> {
    let parsed = MacroDef::parse(input)?;

    Ok(parsed.into_token_stream())
}
