#![allow(unused, unreachable_code)]
use quote::ToTokens;
use syn::{Attribute, LitStr, Token, bracketed, parenthesized, parse::Parse, token::Bracket};

mod kw {
    syn::custom_keyword!(rename);
    syn::custom_keyword!(methods);
}

#[derive(Debug, Default)]
pub struct LuaAttrs {
    pub rename: Option<String>,
}

#[derive(Debug)]
pub struct LuaAttrInput {
    pub method_signatures: Vec<MethodSig>,
}

impl Parse for LuaAttrInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut method_signatures = Vec::new();
        while !input.is_empty() {
            let _ = input.parse::<kw::methods>()?;
            let _ = input.parse::<Token![=]>()?;

            let bracketed;
            let _ = bracketed!(bracketed in input);

            let sigs = bracketed.parse_terminated(MethodSig::parse, Token![,])?;
            method_signatures.extend(sigs);
        }
        Ok(LuaAttrInput { method_signatures })
    }
}

#[derive(Debug)]
pub struct MethodSig {
    pub name: syn::Ident,
    pub receiver: Option<syn::Receiver>,
    pub args: Vec<(syn::Ident, syn::Type)>,
    pub returning: Option<syn::Type>,
}

impl Parse for MethodSig {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = input.parse::<syn::Ident>()?;

        let in_paren;
        let _ = parenthesized!(in_paren in input);

        let mut receiver = None;
        let mut args = Vec::new();

        if in_paren.peek(Token![self]) || in_paren.peek(Token![&]) {
            receiver = Some(in_paren.parse::<syn::Receiver>()?);
            if in_paren.peek(Token![,]) {
                let _ = in_paren.parse::<Token![,]>()?;
            }
        }

        while !in_paren.is_empty() {
            let arg_name = in_paren.parse::<syn::Ident>()?;
            let _ = in_paren.parse::<Token![:]>()?;
            let ty = in_paren.parse::<syn::Type>()?;
            if in_paren.peek(Token![,]) {
                let _ = in_paren.parse::<Token![,]>()?;
            }
            args.push((arg_name, ty));
        }

        let returning = if input.peek(Token![->]) {
            let _ = input.parse::<Token![->]>()?;
            Some(input.parse::<syn::Type>()?)
        } else {
            None
        };

        Ok(MethodSig {
            name,
            receiver,
            args,
            returning: returning,
        })
    }
}

impl Parse for LuaAttrs {
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
        Ok(LuaAttrs { rename })
    }
}

// FIX: return a Result instead and return error for unknown attributes.
pub fn parse_lua_attr(attrs: &[Attribute]) -> Option<LuaAttrs> {
    for attr in attrs {
        if attr.path().is_ident("lua") {
            match attr.parse_args() {
                Ok(p) => return Some(p),
                Err(_) => return Some(LuaAttrs::default()),
            }
        }
    }
    None
}

pub fn remove_lua_attr(attrs: &mut Vec<Attribute>) {
    attrs.retain_mut(|attr| !attr.path().is_ident("lua"));
}
