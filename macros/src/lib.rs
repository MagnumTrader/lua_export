#![allow(unused, unreachable_code)]
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    Data, DataStruct, DeriveInput, Fields, Item, ItemStruct, Path, Type, TypePath, parse::Parser,
    punctuated::Punctuated, spanned::Spanned, token::Comma,
};

#[proc_macro_derive(LuaExport, attributes(skip))]
pub fn lua_export(tokens: TokenStream) -> TokenStream {
    match inner(tokens.into()) {
        Ok(ts) => ts.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

const STRUCT_ERROR: &str = "Can only use lua_export on Structs with named fields";

fn inner(tokens: TokenStream2) -> syn::Result<TokenStream2> {

    let input = syn::parse2::<DeriveInput>(tokens)?;
    let ident = &input.ident;
    let span = input.span();

    let Data::Struct(DataStruct { fields, .. }) = input.data else {
        return Err(syn::Error::new(span, STRUCT_ERROR));
    };

    let fields = fields.iter().map(|field| {
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
