mod parse;
mod codegen;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{
    spanned::Spanned, Attribute, Item
};

use crate::parse::LuaMacroInput;


#[proc_macro_attribute]
pub fn lua_export(attrs: TokenStream, tokens: TokenStream) -> TokenStream {
    match inner(attrs.into(), tokens.into()) {
        Ok(ts) => ts.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

fn inner(attrs: TokenStream2, tokens: TokenStream2) -> syn::Result<TokenStream2> {
    let span = tokens.span();
    let mut code = match syn::parse2::<Item>(tokens)? {
        Item::Struct(item_struct) => codegen::handle_struct(item_struct)?,
        Item::Impl(item_impl) => codegen::handle_impl(item_impl)?,
        item => return Err(syn::Error::new(
            span,
            format!("lua_export not implemented for {}", item_to_str(item)),
        )),
    };

    let methods: LuaMacroInput = syn::parse2(attrs)?;
    
    eprintln!("{:?}", methods);
    if !methods.signatures.is_empty() {
        let sig = &methods.signatures[0];
        let types = sig.args.iter().map(|(_, ty)| {
            ty
        });
        let returns = if let Some(r) = sig.returning.as_ref() {
            quote::quote!{ -> #r}
        } else {
            quote::quote!{}
        };
        code.extend(
            quote::quote!{
                const _: fn() = || {
                    let _: fn(&MyTestIndicator, #(#types),*) #returns = MyTestIndicator::other;
                };
            }
        );
    }

    Ok(code)
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
