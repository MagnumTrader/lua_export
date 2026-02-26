
// This is what we want to implement for our structs
// So that we can do AvgRange::to_lua_struct()
pub trait ToLuaStruct {
    fn to_lua_struct() -> LuaStruct;
}

pub struct LuaStruct {
    pub name: &'static str
}
