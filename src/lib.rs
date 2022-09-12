#[macro_export]
/// Build a function from a dynamic link library
/// lib_path: The path of dll
/// fn_name: The function of dll's function
/// call_name: The call function of fn_name
/// (v: t): The params of the function
macro_rules! get_libfn {
    ($lib_path: expr, $fn_name: expr, $call_name:ident, $ret: ty, $($v: ident: $t:ty),*) => {
        pub fn $call_name($($v: $t),*) -> $ret {
            unsafe {
                let lib = libloading::Library::new("libstd.dylib").unwrap();
                let func: libloading::Symbol<fn($($t,)*) -> $ret> = lib.get($fn_name.as_bytes()).unwrap();
                func($($v,)*)
            }
        }
    };
    ($lib_path: expr, $fn_name: expr, $call_name:ident, $ret: ty) => {
        pub fn $call_name() -> $ret {
            unsafe {
                let lib = libloading::Library::new("libstd.dylib").unwrap();
                let func: libloading::Symbol<fn() -> $ret> = lib.get($fn_name.as_bytes()).unwrap();
                func()
            }
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_libfn() {
        get_libfn!("libstd.dylib", "println", my_println, (), str: &str);
        my_println("Hello World");

        get_libfn!("libstd.dylib", "add", my_add, usize, a: usize, b: usize);
        println!("10 + 20 = {}", my_add(10, 20));

        get_libfn!("libstd.dylib", "print_hello", my_print_hello, ());
        my_print_hello();
    }
}
