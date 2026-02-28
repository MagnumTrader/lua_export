#![allow(unused, unreachable_code)]
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    Attribute, Data, DataStruct, DeriveInput, Fields, Item, ItemStruct, Meta, MetaList, Path, Type,
    TypePath, parse::Parser, punctuated::Punctuated, spanned::Spanned, token::Comma,
};

#[proc_macro_attribute]
pub fn lua_export(attrs: TokenStream, tokens: TokenStream) -> TokenStream {
    match inner(attrs.into(), tokens.into()) {
        Ok(ts) => ts.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

const STRUCT_ERROR: &str = "Can only use lua_export on Structs with named fields";

fn include_lua(field: &syn::Field) -> bool {
    field.attrs.iter().any(|a| a.path().is_ident("lua"))
}

// handle the attributes instead parse the inner argument attr inner
fn inner(attrs: TokenStream2, tokens: TokenStream2) -> syn::Result<TokenStream2> {
    // refactor this so that we can parse impl blocks and stuff aswell, 
    // Then we handle the impl blocks for methods should it only be on the methods. can it look up
    // self?
    let i = syn::parse2::<Item>(tokens.clone())?;
    let input = syn::parse2::<DeriveInput>(tokens)?;
    let ident = &input.ident;
    let span = input.span();

    let Data::Struct(DataStruct { fields, .. }) = input.data else {
        return Err(syn::Error::new(span, STRUCT_ERROR));
    };

    let fields = fields.iter().filter(|f| include_lua(f)).map(|field| {
        // NOTE: we can return here, and collect into Result<Vec<_>> and do ? on that one
        // Maybe use combined for multile things
        let ident = field.ident.as_ref().expect("Only support named fields");
        let Type::Path(TypePath { path, .. }) = &field.ty else {
            panic!("only works with path")
        };
        let last_ty = path.segments.last().unwrap();

        quote! {
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

    Ok(export_fields)
}
