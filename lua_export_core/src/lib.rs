#![allow(unused, unreachable_code)]

use std::collections::HashMap;
pub use inventory;

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

#[derive(Debug)]
pub struct LuaStruct {
    pub name: &'static str,
    pub fields: Option<&'static [LuaField]>, 
}

/// Not public api only used for
pub struct LuaItem<T: 'static> {
    pub belongs_to: &'static str,
    pub items: &'static [T],
}

#[derive(Debug)]
pub struct LuaField {
    pub name: &'static str,
}

inventory::collect!(LuaItem<LuaField>);
