#![allow(unused, unreachable_code)]
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse::Parser, punctuated::Punctuated, token::Comma, Fields, ItemStruct, Path, Type, TypePath};

#[proc_macro_attribute]
pub fn lua_export(attr: TokenStream, tokens: TokenStream) -> TokenStream {
    match inner(attr.into(), tokens.into()) {
        Ok(ts) => ts.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

const STRUCT_ERROR: &str = "Can only use lua_export on Structs with named fields";

fn inner(attr: TokenStream2, mut tokens: TokenStream2) -> syn::Result<TokenStream2> {
    let ItemStruct {
        ident,
        fields: Fields::Named(fields),
        ..
    } = syn::parse2(tokens.clone())?
    else {
        return Err(syn::Error::new_spanned(tokens, STRUCT_ERROR));
    };


    // TODO: add a filter here that checks if the attributes have skip in them
    // shortcircuit the iterator
    let fields = fields.named.iter().map(|field|{
        let ident = field.ident.as_ref().expect("in named fields");
        let Type::Path(TypePath{ path, .. }) = &field.ty else {
            panic!("only works with path")
        };
        let last_ty = path.segments.last().unwrap();

        quote!{
            LuaField {
                name: stringify!(#ident),
                ty: stringify!(#last_ty)
            }
        }
    });


    let export_fields = quote! {
        ::lua_export_core::inventory::submit!{
            ::lua_export_core::LuaItem {
                belongs_to: stringify!(#ident),
                items: &[#(#fields),*]
            }
        }
    };

    tokens.extend(export_fields);
    Ok(tokens)
}
