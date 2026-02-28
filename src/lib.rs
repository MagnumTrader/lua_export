// Crate for the things that we will be using for translating rust types to lua
pub use lua_export_core::*;
pub use macros::*;
// TODO:s
// Add #[lua(skip)] attribute
// Extract the struct logic, so we can match on where the attribute is done
//
// Add #[lua_export] for methods and impl blocks math on it
//
// Add #[lua(rename = "Myname")] attribute

#[allow(unused, unreachable_code)]
#[cfg(test)]
mod tests {

    use super::*;

    #[derive(LuaExport)]
    pub struct MyTestIndicator {
        pub number: usize,
        pub inner: std::string::String,
    }

    #[test]
    fn first_test() {
        let mut l_types = get_lua_types();

        let ty = l_types.next().unwrap();
        assert_eq!(ty.name, "MyTestIndicator");
        assert_eq!(ty.fields.unwrap()[0].name, "number");
        assert_eq!(ty.fields.unwrap()[0].ty, "usize");
        assert_eq!(ty.fields.unwrap()[1].name, "inner");
        assert_eq!(ty.fields.unwrap()[1].ty, "String");
        assert!(l_types.next().is_none(), "We have only defined one struct in the teest module. fragile")
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
