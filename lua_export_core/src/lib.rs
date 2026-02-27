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
}

inventory::collect!(LuaItem<LuaField>);
