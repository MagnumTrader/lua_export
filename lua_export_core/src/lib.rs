#![allow(unused, unreachable_code)]

pub use inventory;
use std::collections::HashMap;

pub fn get_lua_types() -> impl Iterator<Item = LuaStruct> {
    // Collect meta data
    let mut m = HashMap::new();

    for item in inventory::iter::<LuaItem<LuaField>> {
        let s = m.entry(item.belongs_to).or_insert(LuaStruct {
            name: item.belongs_to,
            fields: None,
        });
        s.fields = Some(item.items);
    }
    // Will add methods and other meta data here if needed
    m.into_iter().map(|(_, s)| s)
}

/// Main type returned from the iterator.
/// Collected by TODO: insert instructions.
///
/// This is where we will implement to
/// lua_docs_str
/// and parse it in other ways.
#[derive(Debug)]
pub struct LuaStruct {
    pub name: &'static str,
    pub fields: Option<&'static [LuaField]>,
}

/// Not public api only used for connecting T
/// metadata, methods and fields with a struct.
pub struct LuaItem<T: 'static> {
    pub belongs_to: &'static str,
    pub items: &'static [T],
}

// i want to work with visibility
#[derive(Debug)]
pub struct LuaField {
    pub name: &'static str,
    pub ty: &'static str,
}

pub enum LuaType {
    // Lua 5.x?
    Integer,
    Number,
    String,
    Table,
    Nil,
}

impl From<syn::TypePath> for LuaType {
    fn from(value: syn::TypePath) -> Self {
        let s = value.path.segments.last().expect("Path should have last");
        let s = s.ident.to_string();
        match s.as_str() {
            "String" => LuaType::String,
            "usize" | "isize" | "i32" | "i8"  => LuaType::Integer,
            _ => unimplemented!("Type not implemented: {}", s),
        }
    }
}

inventory::collect!(LuaItem<LuaField>);
