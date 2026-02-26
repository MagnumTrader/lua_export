pub use macros::*;
pub use lua_export_core::*;

#[lua_export]
pub struct MyIndicator;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn first_test() {
        let myind = MyIndicator::to_lua_struct();
        assert_eq!("MyIndicator", myind.name)
    }
}
