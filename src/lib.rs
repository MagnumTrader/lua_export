// Crate for the things that we will be using for translating rust types
// to lua
//
//
// TODO: Test how inventory work
// Theory is that we can entirely encapsulate inventory in this crate
// but use it as a way to combine Fields and methods into lua structs
// LuaPart<T> { name: of struct, inner: T}
// Iterate over it, and insert it into the hashmap
// that has the name as the key.
pub use lua_export_core::*;
pub use macros::*;

#[lua_export(number, inner)]
pub struct MyIndicator {
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
}
