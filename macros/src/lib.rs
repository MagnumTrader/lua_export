mod codegen;
mod parse;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{Item, spanned::Spanned};

#[proc_macro_attribute]
pub fn lua_export(attrs: TokenStream, tokens: TokenStream) -> TokenStream {
    match inner(attrs.into(), tokens.into()) {
        Ok(ts) => ts.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

fn inner(attrs: TokenStream2, tokens: TokenStream2) -> syn::Result<TokenStream2> {
    let span = tokens.span();
    match syn::parse2::<Item>(tokens)? {
        Item::Struct(item_struct) => codegen::handle_struct(item_struct, attrs),
        Item::Impl(item_impl) => codegen::handle_impl(item_impl),
        item => Err(syn::Error::new(
            span,
            format!("lua_export not implemented for {}", item_to_str(item)),
        )),
    }
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
