use syn::{Attribute, LitStr, Token, parse::Parse};

#[derive(Debug, Default)]
pub struct LuaAttrs {
    pub rename: Option<String>,
}

mod kw {
    syn::custom_keyword!(rename);
}

impl Parse for LuaAttrs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut rename = None;

        while !input.is_empty() {
            let lookahead = input.lookahead1();
            if lookahead.peek(kw::rename) {
                input.parse::<kw::rename>().expect("just peeked");
                input.parse::<Token![=]>()?;
                let lit: LitStr = input.parse()?;
                rename = Some(lit.value())
            }
        }
        Ok(LuaAttrs { rename })
    }
}

// FIX: return a Result instead and return error for unknown attributes.
pub fn parse_lua_attr(attrs: &[Attribute]) -> Option<LuaAttrs> {
    for attr in attrs {
        if attr.path().is_ident("lua") {
            match attr.parse_args() {
                Ok(p) => return Some(p),
                Err(_) => return Some(LuaAttrs::default()),
            }
        }
    }
    None
}

pub fn remove_lua_attr(attrs: &mut Vec<Attribute>) {
    attrs.retain_mut(|attr| !attr.path().is_ident("lua"));
}
