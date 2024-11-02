First I will create a directory where I will have the lib and the app projects:

```sh
mkdir dynamic_link
cd dynamic_link/
```

Then I will create the lib project:

```sh
cargo new --lib hello_lib
```

Add the next two lines to the lib's `Cargo.toml` file:

```diff
[package]
name = "hello_lib"
version = "0.1.0"
edition = "2021"

+[lib]
+crate-type = ["dylib"]

[dependencies]
```

My `lib.rs` file look like this:

```rust
pub fn say_hi(name: String) {
    println!("Hello, {}!", name);
}
```

Create the app's project and add the library as dependency:

```sh
cargo new hello_app
cd hello_app/
cargo add hello_lib --path ../hello_lib
```

```rust
fn main() {
    let name = "Potato".to_string();
    hello_lib::say_hi(name);
}
```

```sh
$ readelf -d /tmp/hello_app

Dynamic section at offset 0x2db8 contains 26 entries:
  Tag        Type                         Name/Value
 0x0000000000000001 (NEEDED)             Shared library: [libhello_lib.so]
 0x0000000000000001 (NEEDED)             Shared library: [libstd-52417a9a08ba8fb9.so]
 0x0000000000000001 (NEEDED)             Shared library: [libc.so.6]
 ...
```

## Ref Links

- https://stackoverflow.com/questions/75903098/dynamic-linking-between-crates
