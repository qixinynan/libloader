#[macro_export]
/// Build a function from a dynamic link library
/// lib_path: The path of dll
/// fn_name: The function of dll's function
/// call_name: The call function of fn_name
/// (v: t): The params of the function
macro_rules! create_libfn {
    ($lib_path: expr, $fn_name: expr, $call_name:ident, $($v: ident: $t:ty),*) => {
        pub fn $call_name($($v: $t),*) {
            unsafe {
                let lib = libloading::Library::new("libstd.dylib").unwrap();
                let func: libloading::Symbol<fn($($t,)*)> = lib.get($fn_name.as_bytes()).unwrap();
                func($($v,)*)
            }
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_create_libfn() {
        create_libfn!("libstd.dylib", "println", my_println, str: &str);
        println!("12");
        my_println("Hello");
    }
}
