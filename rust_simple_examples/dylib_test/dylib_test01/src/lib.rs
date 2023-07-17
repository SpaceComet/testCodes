// `no_mangle` ensures the function name remains the same
// after compiled into a DLL
#[no_mangle]
pub fn hello() {
    println!("Hello from DLL");
}

#[no_mangle]
pub fn returns_number() -> i32 {
    8080
}

#[no_mangle]
pub fn square(num: i32) -> i32 {
    num * num
}

#[no_mangle]
pub fn option_r(a: i32, b: i32) -> Result<i32, String> {
    if b != 0 {
        Ok(a / b)
    } else {
        Err("Error".to_string())
    }
}
