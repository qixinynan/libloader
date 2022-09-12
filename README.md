# libloader

**libloader is a easy-to-use DLL loader for rust that based on libloading**

It is very easy to dynamically call a function from dynamic link library (DLL files for Windows, so files for Unix/Linux dylib for macOS )

Use this function to get the function from DLL

```rust
get_libfn!(
    "path.dll",
    "func_name",
    name_to_call,
    return_type, // ifreturn type is none, use "()" instead
    param1_name: param1_type
    param2_name: param2_type
    param3_name: param3_type
    ...
);
```

For example, We have these functions from libstd.dylib

```rust
// lib.rs （compiled into libstd.dylib)
#[no_mangle]
pub fn println(str: &str) {
    println!("{}", str);
}

#[no_mangle]
pub fn add(a: usize, b: usize) -> usize {
    a + b
}

#[no_mangle]
pub fn print_hello() {
    println!("Hello");
}

```

We can call it with:

```rust
// main.rs
use libloader::libloading
fn main() {
    get_libfn!("libstd.dylib", "println", my_println, (), str: &str);
    my_println("Hello World");

    get_libfn!("libstd.dylib", "add", my_add, usize, a: usize, b: usize);
    println!("10 + 20 = {}", my_add(10, 20));

    get_libfn!("libstd.dylib", "print_hello", my_print_hello, ());
    my_print_hello();
}
```

The output is:
```
Hello World
10 + 20 = 30
hello
```

