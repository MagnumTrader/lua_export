// Crate for the things that we will be using for translating rust types to lua
pub use lua_export_core::*;
pub use macros::*;

#[lua_export(number, inner)]
pub struct MyIndicator {
    //TODO: Only use real fields
    //TODO: 2 Add type meta data to LuaField, map -> LuaType
    pub inner: usize,
}

#[allow(unused, unreachable_code)]
#[cfg(test)]
mod tests {
    #[lua_export(number, inner)]
    pub struct MyIndicator {
        pub inner: usize,
    }

    use super::*;
    #[test]
    fn first_test() {
        let mut it = get_lua_types();

        let t = it.next().unwrap();
        assert_eq!(t.fields.unwrap()[0].name, "number");
        assert_eq!(t.fields.unwrap()[1].name, "inner");
    }

    struct Hello;

    fn t<T>() -> &'static str {
        std::any::type_name::<T>().split("::").last().unwrap()
    }

    #[test]
    fn stringify() {
        let x = t::<Hello>();
        assert_eq!(x, "Hello")
    }
}
