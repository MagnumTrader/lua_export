use crate::parse::{LuaAttrInput, MethodSig, parse_lua_attr, remove_lua_attr};

use proc_macro2::TokenStream;
use quote::quote;
use syn::{ImplItem, ImplItemFn, ItemImpl, ItemStruct, Type, TypePath, spanned::Spanned};

pub fn handle_struct(mut item_struct: ItemStruct, attrs: TokenStream) -> syn::Result<TokenStream> {
    let ItemStruct {
        ref ident,
        ref mut fields,
        ..
    } = item_struct;

    // TODO: we should probably be branching here.
    // keeping the context for example with the name of the struct into the generation
    // of methods etc then we reconstruct, and produce the LuaStruct
    //
    // we produce the implemention of mlua if feature is activated
    // TODO: extract this to function. collect_fields_quote
    let mut quote_fields = Vec::new();

    for field in fields {
        let Some(field_attrs) = parse_lua_attr(&field.attrs) else {
            continue;
        };
        remove_lua_attr(&mut field.attrs);

        let Type::Path(TypePath { path, .. }) = &field.ty else {
            panic!("Lua export only work with types of type path")
        };

        let last_ty = path.segments.last().unwrap();
        let field_ident = match field_attrs.rename {
            Some(name) => &syn::Ident::new(&name, field.span()),
            None => field.ident.as_ref().expect("expect to have named fields"),
        };
        quote_fields.push(quote! {
            LuaField {
                name: stringify!(#field_ident),
                ty: stringify!(#last_ty)
            }
        })
    }

    let methods: LuaAttrInput = syn::parse2(attrs)?;
    let quote_methods = methods.signatures.iter().map(|m| {
        let method_name = &m.name;

        quote!{
            LuaMethod {
                name: stringify!(#method_name)
            }
        }
    });
    let method_verifications = method_verifications(&methods.signatures);


    let mlua_impl = quote! {}; // TODO: should fetch from a function where we pass, the lua
    // representation?

    let reconstructed = quote! {

        #method_verifications
        #item_struct

        #mlua_impl

        ::lua_export_core::inventory::submit!{
            ::lua_export_core::LuaItem::<LuaField> {
                belongs_to: stringify!(#ident),
                items: &[#(#quote_fields),*]
            }
        }
        ::lua_export_core::inventory::submit!{
            ::lua_export_core::LuaItem::<LuaMethod> {
                belongs_to: stringify!(#ident),
                items: &[#(#quote_methods),*]
            }
        }
    };

    // GOAL: This function should emit the struct.
    // have published the luastruct to inventory
    // implemented UserData

    Ok(reconstructed)
}

pub fn method_verifications(signatures: &[MethodSig]) -> TokenStream {
    let mut code = quote::quote! {};

    for sig in signatures {
        let name = &sig.name;
        let types = sig.args.iter().map(|(_, ty)| ty);

        let returns = if let Some(r) = sig.returning.as_ref() {
            quote::quote! { -> #r}
        } else {
            quote::quote! {}
        };

        let method_span = name.span();
        code.extend(
            // FIX: Creating compile time verification of the methods
            // this will be optimized away in a release build, or so they say.
            quote::quote_spanned! {method_span=>
                const _: fn() = || {
                    let _: fn(&MyTestIndicator, #(#types),*) #returns = MyTestIndicator::#name;
                };
            },
        );
    }
    code
}
