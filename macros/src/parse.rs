#![allow(unused, unreachable_code)]
use quote::ToTokens;
use syn::{
    Attribute, FnArg, LitStr, PatType, Token, bracketed, parenthesized, parse::Parse,
    token::Bracket,
};

mod kw {
    syn::custom_keyword!(rename);
    syn::custom_keyword!(methods);
}

#[derive(Debug, Default)]
pub struct LuaFieldAttrs {
    pub rename: Option<String>,
}

#[derive(Debug)]
pub struct LuaAttrInput {
    pub method_signatures: Vec<MethodSignature>,
}

impl Parse for LuaAttrInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut method_signatures = Vec::new();
        while !input.is_empty() {
            let _ = input.parse::<kw::methods>()?;
            let _ = input.parse::<Token![=]>()?;

            let bracketed;
            let _ = bracketed!(bracketed in input);

            let sigs = bracketed.parse_terminated(MethodSignature::parse, Token![,])?;
            method_signatures.extend(sigs);
        }
        Ok(LuaAttrInput { method_signatures })
    }
}

#[derive(Debug)]
pub struct MethodSignature {
    pub name: syn::Ident,
    pub receiver: Option<syn::Receiver>,
    pub args: Vec<PatType>,
    pub returning: Option<syn::Type>,
}

impl Parse for MethodSignature {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = input.parse::<syn::Ident>()?;

        let in_paren;
        let _ = parenthesized!(in_paren in input);

        let mut receiver = None;
        if in_paren.peek(Token![self]) || in_paren.peek(Token![&]) {
            receiver = Some(in_paren.parse::<syn::Receiver>()?);
            if in_paren.peek(Token![,]) {
                let _ = in_paren.parse::<Token![,]>()?;
            }
        }

        let args = in_paren
            .parse_terminated(PatType::parse, Token![,])?
            .into_iter()
            .collect();

        let returning = if input.peek(Token![->]) {
            let _ = input.parse::<Token![->]>()?;
            Some(input.parse::<syn::Type>()?)
        } else {
            None
        };

        Ok(MethodSignature {
            name,
            receiver,
            args,
            returning,
        })
    }
}

impl Parse for LuaFieldAttrs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut rename = None;

        while !input.is_empty() {
            let lookahead = input.lookahead1();
            if lookahead.peek(kw::rename) {
                input.parse::<kw::rename>().expect("just peeked");
                input.parse::<Token![=]>()?;
                let lit: LitStr = input.parse()?;
                rename = Some(lit.value())
            }
        }
        Ok(LuaFieldAttrs { rename })
    }
}

// FIX: return a Result instead and return error for unknown attributes.
pub fn parse_lua_attr(attrs: &[Attribute]) -> Option<LuaFieldAttrs> {
    for attr in attrs {
        if attr.path().is_ident("lua") {
            match attr.parse_args() {
                Ok(p) => return Some(p),
                Err(_) => return Some(LuaFieldAttrs::default()),
            }
        }
    }
    None
}

pub fn remove_lua_attr(attrs: &mut Vec<Attribute>) {
    attrs.retain_mut(|attr| !attr.path().is_ident("lua"));
}
