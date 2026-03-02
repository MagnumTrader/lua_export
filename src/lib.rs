// Crate for the things that we will be using for translating rust types to lua
pub use lua_export_core::*;
pub use macros::*;
// TODO:s
//  
// Clean up method parsing.
// Add new methods to the
// Add LuaDocs generation for LuaStruct
// Add mlua feature -> Impl UserData for fields and methods
// - [x] Refactor to original design for methods. But assert by useing const _: fn() || ...
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

    #[lua_export(
        methods = [
            fun(field1: usize) -> &'static str,
        ]
    )]
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

    impl MyTestIndicator {

        const IGNORED: &'static str = "I am ignored by Lua export";

        // Included
        pub fn fun(&self, m: usize) -> &'static str {
            "hello"
        }

        // Not included
        pub fn other(&self, m: usize) -> String {
            "hello".to_string()
        }

        pub fn wierd_name(&self, m: usize) -> String {
            "hello".to_string()
        }

        pub fn const_verification(&self, m: usize) -> &'static str {
            "hello"
        }
    }

    impl From<String> for MyTestIndicator {
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
            HashSet::from(["fun"])
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
