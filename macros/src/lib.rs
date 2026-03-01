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

fn remove_lua_attr(attrs: &mut Vec<Attribute>) {
    attrs.retain_mut(|attr| !attr.path().is_ident("lua"));
}

fn has_lua_attr(attrs: &Vec<Attribute>) -> bool {
    attrs.iter().any(|a| a.path().is_ident("lua"))
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

    let mut quote_fields = Vec::new();

    for field in fields {

        if !has_lua_attr(&field.attrs) {
            continue;
        }
        remove_lua_attr(&mut field.attrs);

        let Type::Path(TypePath { path, .. }) = &field.ty else {
            panic!("Lua export only work with types of type path")
        };

        let last_ty = path.segments.last().unwrap();
        let field_ident = &field.ident;
        quote_fields.push(quote! {
            LuaField {
                name: stringify!(#field_ident),
                ty: stringify!(#last_ty)
            }
        })
    }

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

    let mut quote_methods = Vec::new();
    for item in items {

        let fn_impl = match item {
            ImplItem::Fn(impl_item_fn) => impl_item_fn,
            // FIX: we just ignores all other types, so we dont 
            // catch when user annotates other items than Fn
            _ => continue, 
        };

        let ImplItemFn {
            attrs,
            sig,
            ..
        } = fn_impl;

        if !has_lua_attr(attrs) {
            continue;
        }
        remove_lua_attr(attrs);

        let ident = &sig.ident;
        quote_methods.push(quote! {
            LuaMethod {
                name: stringify!(#ident)
            }
        });
    }

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
