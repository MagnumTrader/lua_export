# Lua_export
types and macros to export rust types to lua.
Implements UserData and exposes fields and methods into the lua runtime from rust.

# Example
```rust
#[lua_export]
struct MyRustStruct {
    inner: usize,
    other: String,
    #[skip]
    skipping: usize
}
```

This will produce: 
TODO: ...

