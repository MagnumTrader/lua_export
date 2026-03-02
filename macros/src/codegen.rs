use crate::parse::{LuaAttrInput, MethodSignature, parse_lua_attr, remove_lua_attr};

use proc_macro2::TokenStream;
use quote::quote;
use syn::{FnArg, ItemStruct, PatIdent, PatType, Type, TypePath, spanned::Spanned};

pub fn handle_struct(mut item_struct: ItemStruct, attrs: TokenStream) -> syn::Result<TokenStream> {
    let ItemStruct {
        ref ident,
        ref mut fields,
        ..
    } = item_struct;

    let mut quote_fields = Vec::new();
    let mut mlua_fields = Vec::new();

    for field in fields {
        //TODO: have some better struct to describe the fields here
        // Check if it has lua attr, return Some(LuaField)
        // LuaField {
        //  name: Ident
        //  rename: Option<String>
        // }
        let Some(field_attrs) = parse_lua_attr(&field.attrs) else {
            continue;
        };
        remove_lua_attr(&mut field.attrs);

        let Type::Path(TypePath { path, .. }) = &field.ty else {
            panic!("Lua export only work with types of type path")
        };


        let field_name = field.ident.as_ref().unwrap();
        let field_ident = match field_attrs.rename {
            Some(name) => &syn::Ident::new(&name, field.span()),
            None => field_name,
        };

        mlua_fields.push(quote!{
                fields.add_field_method_get(stringify!(#field_ident), |_, this| {
                    Ok(this.#field_name.clone())
                });
        });

        let last_ty = path.segments.last().unwrap();
        quote_fields.push(quote! {
            LuaField {
                name: stringify!(#field_ident),
                ty: stringify!(#last_ty)
            }
        })
    }

    let methods: LuaAttrInput = syn::parse2(attrs)?;
    let quote_methods = methods.method_signatures.iter().map(|m| {
        let method_name = &m.name;
        quote! {
            LuaMethod {
                name: stringify!(#method_name)
            }
        }
    });
    let method_verifications = method_verifications(&ident, &methods.method_signatures);

    let mlua_impl = quote! {
        impl ::mlua::UserData for MyTestIndicator {
            fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
                #(#mlua_fields)*
            }
            fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {}
        }
    }; 

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

fn receiver_to_typed(recv: &syn::Receiver, ty_name: &syn::Ident) -> FnArg {
    let par_name = ty_name.to_string().to_lowercase();
    let pat = Box::new(syn::Pat::Ident(PatIdent {
        attrs: vec![],
        by_ref: None,
        mutability: None,
        ident: syn::Ident::new(&par_name, recv.self_token.span()),
        subpat: None,
    }));

    let ty: Box<Type> = if let Some((and, lifetime)) = &recv.reference {
        // &self or &mut self → &TypeName or &mut TypeName
        Box::new(Type::Reference(syn::TypeReference {
            and_token: *and,
            lifetime: lifetime.clone(),
            mutability: recv.mutability,
            elem: Box::new(syn::parse_quote!(#ty_name)),
        }))
    } else {
        // self or mut self → TypeName
        Box::new(syn::parse_quote!(#ty_name))
    };

    FnArg::Typed(PatType {
        attrs: recv.attrs.clone(),
        pat,
        colon_token: Default::default(),
        ty,
    })
}

pub fn method_verifications(type_name: &syn::Ident, signatures: &[MethodSignature]) -> TokenStream {
    let mut code = quote::quote! {};

    for sig in signatures {
        let name = &sig.name;
        let receiver = if let Some(recv) = &sig.receiver {
            let typed = receiver_to_typed(recv, type_name);
            quote! {
                #typed,
            }
        } else {
            quote! {}
        };

        let args = sig.args.iter().map(|PatType { pat, ty, .. }| {
            eprintln!("{:?}", pat);
            quote! {
                #pat: #ty
            }
        });

        let returns = if let Some(return_ty) = &sig.returning {
            quote! {-> #return_ty}
        } else {
            quote! {}
        };

        let method_span = name.span();

        code.extend(
            // Creating compile time verification of the methods
            // this will be optimized away in a release build, atleast or so they say...
            quote::quote_spanned! {method_span=>
                const _: fn() = || {
                    let _: fn(#receiver #(#args),*) #returns = #type_name::#name;
                };
            },
        );
    }
    code
}
