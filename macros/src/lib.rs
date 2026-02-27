use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{ItemStruct, Path, parse::Parser, punctuated::Punctuated, token::Comma};

#[proc_macro_attribute]
pub fn lua_export(attr: TokenStream, tokens: TokenStream) -> TokenStream {
    match inner(attr.into(), tokens.into()) {
        Ok(ts) => ts.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

fn inner(attr: TokenStream2, mut tokens: TokenStream2) -> syn::Result<TokenStream2> {
    let ItemStruct { ident, .. } = syn::parse2(tokens.clone())?;

    let list = Punctuated::<Path, Comma>::parse_terminated.parse2(attr)?;

    //TODO: Loop over real fields,
    //parse type
    let fields = list.iter().map(|path| {
        let name = path.get_ident().expect("Expecting ident in field");

        quote! {
            LuaField {
                name: stringify!(#name),
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
