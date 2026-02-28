// Crate for the things that we will be using for translating rust types to lua
pub use lua_export_core::*;
pub use macros::*;
// TODO:s
// Extract the struct logic, so we can match on where the attribute is done
// Add #[lua_export] for methods and impl blocks math on it
// Add #[lua(rename = "Myname")] attribute

#[allow(unused, unreachable_code)]
#[cfg(test)]
mod tests {

    use super::*;

    #[derive(LuaExport)]
    pub struct MyTestIndicator {
        pub number: usize,
        pub inner: std::string::String,
        #[skip]
        pub skipping: usize
    }

    #[test]
    fn first_test() {
        let mut l_types = get_lua_types();

        let ty = l_types.next().unwrap();
        assert_eq!(ty.name, "MyTestIndicator");

        let mut fields = ty.fields.unwrap().iter();

        let first = fields.next().unwrap();
        assert_eq!(first.name, "number");
        assert_eq!(first.ty, "usize");

        let second = fields.next().unwrap();
        assert_eq!(second.name, "inner");
        assert_eq!(second.ty, "String");

        assert!(fields.next().is_none());

        assert!(l_types.next().is_none(), "We have only defined one struct in the test module. fragile")
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
