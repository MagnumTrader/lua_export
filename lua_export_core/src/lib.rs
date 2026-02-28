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
            methods: Vec::new()
        });
        s.fields = Some(item.items);
    }

    for item in inventory::iter::<LuaItem<LuaMethod>> {
        eprintln!("{item:?}");
        let s = m.entry(item.belongs_to).or_insert(LuaStruct {
            name: item.belongs_to,
            fields: None,
            methods: Vec::new(),
        });
        s.methods.extend(item.items);
    }
    // Will add methods and other meta data here if needed
    m.into_iter().map(|(_, s)| s)
}

#[derive(Debug)]
pub struct LuaStruct {
    pub name: &'static str,
    pub fields: Option<&'static [LuaField]>,
    pub methods: Vec<LuaMethod>,

}

/// Not public api only used for connecting T
/// metadata, methods and fields with a struct.
#[derive(Debug)]
pub struct LuaItem<T: 'static> {
    pub belongs_to: &'static str,
    pub items: &'static [T],
}

#[derive(Debug, Clone, Copy)]
pub struct LuaField {
    pub name: &'static str,
    pub ty: &'static str,
}

inventory::collect!(LuaItem<LuaField>);

#[derive(Debug, Clone, Copy)]
pub struct LuaMethod {
    pub name: &'static str
}
inventory::collect!(LuaItem<LuaMethod>);

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

