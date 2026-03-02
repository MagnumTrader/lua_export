// Crate for the things that we will be using for translating rust types to lua
pub use lua_export_core::*;
pub use macros::*;
// TO DO:s
//
// Add LuaDocs generation for LuaStruct
// Add mlua feature -> Impl UserData for fields and methods

#[allow(unused, unreachable_code)]
#[cfg(test)]
mod tests {

    use std::collections::HashSet;

    use mlua::{Lua, ObjectLike};

    use super::*;

    #[lua_export(
        methods = [
            fun(&mut self, field1: usize) -> String,
            other(field1: usize) -> String,
        ]
    )]
    struct MyTestIndicator {
        #[lua]
        pub number: usize,
        #[lua]
        pub inner: std::string::String,
        pub skipping: usize,

        #[lua(rename = "renamed")]
        pub wierd_name: usize,
    }

    impl MyTestIndicator {
        // Included
        pub fn fun(&mut self, m: usize) -> String {
            "hello".to_string()
        }

        pub fn other(m: usize) -> String {
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
            ty.methods
                .iter()
                .map(|m| m.name)
                .collect::<HashSet<&'static str>>(),
            HashSet::from(["fun", "other"])
        );
    }

    #[test]
    fn lua_fields() {
        let lua = Lua::new();

        let indicator = MyTestIndicator {
            number: 1337,
            inner: String::from("hello"),
            skipping: 0,
            wierd_name: 123,
        };

        let ud = lua.create_userdata(indicator).unwrap();

        let got = ud.get::<usize>("number").unwrap();
        assert_eq!(got, 1337);

        let got = ud.get::<String>("inner").unwrap();
        assert_eq!(&got, "hello");

        let got = ud.get::<usize>("renamed").unwrap();
        assert_eq!(got, 123);

        let got = ud.get::<usize>("skipped");
        assert!(got.is_err())
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
