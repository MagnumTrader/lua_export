#![allow(unused, unreachable_code)]
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    Attribute, Data, DataStruct, DeriveInput, Fields, ImplItem, ImplItemFn, Item, ItemImpl,
    ItemStruct, Meta, MetaList, Path, Type, TypePath, parse::Parser, punctuated::Punctuated,
    spanned::Spanned, token::Comma,
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
fn inner(attrs: TokenStream2, mut tokens: TokenStream2) -> syn::Result<TokenStream2> {
    let span = tokens.span();
    match syn::parse2::<Item>(tokens)? {
        Item::Struct(item_struct) => handle_struct(item_struct),
        Item::Impl(item_impl) => handle_impl(item_impl),
        item => Err(syn::Error::new(
            span,
            format!("lua_export not implemented for {}", item_to_str(item)),
        )),
    }
}

fn handle_struct(mut item_struct: ItemStruct) -> syn::Result<TokenStream2> {
    let ItemStruct {
        ref ident,
        ref mut fields,
        ..
    } = item_struct;

    let quote_fields: Vec<_> = fields
        .iter_mut()
        .filter(|f| include_lua(f))
        .map(|field| {
            // Remove the #[lua] attribute since its unknown when main macro is an attribute macro
            field.attrs.retain_mut(|f| !f.path().is_ident("lua"));
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
        })
        .collect();

    let reconstructed = quote! {

        #item_struct

        ::lua_export_core::inventory::submit!{
            ::lua_export_core::LuaItem {
                belongs_to: stringify!(#ident),
                items: &[#(#quote_fields),*]
            }
        }
    };

    Ok(reconstructed)
}

fn handle_impl(mut item_impl: ItemImpl) -> syn::Result<TokenStream2> {
    let ItemImpl {
        ref attrs,
        ref defaultness,
        ref unsafety,
        ref impl_token,
        ref generics,
        ref trait_,
        ref self_ty,
        ref brace_token,
        ref mut items,
    } = item_impl;

    let Type::Path(TypePath {
        ref qself,
        ref path,
    }) = **self_ty
    else {
        panic!("Expected path as self_ty, mabe  handle self")
    };

    let ident = *path.get_ident().as_ref().unwrap();

    //TODO: Refactor this, maybe not use iterators?
    let quote_methods: Vec<_> = items
        .iter_mut()
        .filter(|f| {
            let ImplItem::Fn(ImplItemFn {
                attrs,
                vis,
                defaultness,
                sig,
                block,
            }) = f
            else {
                panic!("Only functions in impl blocks are implemented")
            };
            attrs.iter().any(|a| a.path().is_ident("lua"))
        })
        .map(|f| {
            let ImplItem::Fn(ImplItemFn {
                attrs,
                vis,
                defaultness,
                sig,
                block,
            }) = f
            else {
                panic!("Only functions in impl blocks are implemented")
            };
            attrs.retain(|a| !a.path().is_ident("lua"));
            let ident = &sig.ident;
            quote! {
                LuaMethod {
                    name: stringify!(#ident)
                }
            }
        })
        .collect();

    let reconstructed = quote! {
        #item_impl

        ::lua_export_core::inventory::submit!{
            ::lua_export_core::LuaItem {
                belongs_to: stringify!(#ident),
                items: &[#(#quote_methods),*]
            }
        }
    };

    Ok(reconstructed)
}

// TODO: These can have more userfriendly errors, dont expose syn types
fn item_to_str(i: Item) -> &'static str {
    match i {
        Item::Const(_) => "Item::Const",
        Item::Enum(_) => "Item::Enum",
        Item::ExternCrate(_) => "Item::ExternCrate",
        Item::Fn(_) => "Item::Fn",
        Item::ForeignMod(_) => "Item::ForeignMod",
        Item::Impl(_) => "Item::Impl",
        Item::Macro(_) => "Item::Macro",
        Item::Mod(_) => "Item::Mod",
        Item::Static(_) => "Item::Static",
        Item::Struct(_) => "Item::Struct",
        Item::Trait(_) => "Item::Trait",
        Item::TraitAlias(_) => "Item::TraitAlias",
        Item::Type(_) => "Item::Type",
        Item::Union(_) => "Item::Union",
        Item::Use(_) => "Item::Use",
        Item::Verbatim(_) => "Item::Verbatim",
        _ => todo!(),
    }
}
