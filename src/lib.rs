// Crate for the things that we will be using for translating rust types to lua
pub use lua_export_core::*;
pub use macros::*;
// TODO:s
// Add LuaDocs generation for LuaStruct
#[allow(dead_code)]
#[lua_export]
struct MyIndicator {
    #[lua]
    pub number: usize,
    #[lua]
    pub inner: std::string::String,
    pub skipping: usize,
}

#[allow(unused, unreachable_code)]
#[cfg(test)]
mod tests {

    use std::collections::HashSet;

    use super::*;

    #[lua_export]
    struct MyTestIndicator {
        #[lua]
        pub number: usize,
        #[lua]
        pub inner: std::string::String,
        pub skipping: usize,
        // TODO: implement rename for fields and methods
        #[lua(rename = "renamed")]
        pub wierd_name: usize,
    }

    #[lua_export]
    impl MyTestIndicator {

        const IGNORED: &'static str = "I am ignored by Lua export";

        // Included
        #[lua]
        pub fn fun(m: usize) -> &'static str {
            "hello"
        }

        // Not included
        pub fn other(m: usize) -> &'static str {
            "hello"
        }

        // Not included
        #[lua(rename = "renamed_method")]
        pub fn wierd_name(m: usize) -> &'static str {
            "hello"
        }
    }

    #[lua_export]
    impl From<String> for MyTestIndicator {
        #[lua]
        fn from(value: String) -> Self {
            todo!()
        }
    }

    fn get_test_indicator() -> LuaStruct {
        let l_types = get_lua_types();
        l_types
            .into_iter()
            .find(|s| s.name == "MyTestIndicator")
            .unwrap()
    }

    #[test]
    fn test_fields() {
        let ty = get_test_indicator();

        let mut fields = ty.fields.unwrap().iter();

        let first = fields.next().unwrap();
        assert_eq!(first.name, "number");
        assert_eq!(first.ty, "usize");

        let second = fields.next().unwrap();
        assert_eq!(second.name, "inner");
        assert_eq!(second.ty, "String");

        let second = fields.next().unwrap();
        assert_eq!(second.name, "renamed");
        assert_eq!(second.ty, "usize");

        assert!(fields.next().is_none());
    }

    #[test]
    fn test_methods() {
        let ty = get_test_indicator();
        // Test Signatures and returns aswell
        assert_eq!(
            ty.methods.iter().map(|m| m.name).collect::<HashSet<&'static str>>(),
            HashSet::from(["fun", "from", "renamed_method"])
        );
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
