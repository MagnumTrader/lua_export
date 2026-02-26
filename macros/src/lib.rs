#![allow(unused, unreachable_code)]
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    Data, DataStruct, Item, ItemStruct, Stmt,
    parse::{Parse, ParseStream},
};

#[proc_macro_attribute]
pub fn lua_export(attr: TokenStream, tokens: TokenStream) -> TokenStream {
    match inner(attr.into(), tokens.into()) {
        Ok(ts) => ts.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

fn inner(attr: TokenStream2, mut tokens: TokenStream2) -> syn::Result<TokenStream2> {
    let ItemStruct { ident, .. } = syn::parse2(tokens.clone())?;

    let impl_trait = quote! {
        impl ToLuaStruct for #ident {
            fn to_lua_struct() -> LuaStruct {
                LuaStruct {
                    name: stringify!(#ident)
                }
            }
        }
    };
    tokens.extend(impl_trait);
    Ok(tokens)
}

struct StmtList {
    stmts: Vec<Stmt>,
}

impl Parse for StmtList {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut stmts = Vec::new();
        while !input.is_empty() {
            // Expect stmts we can parse first a fieldname, then this then that
            stmts.push(input.parse()?);
        }
        Ok(StmtList { stmts })
    }
}
