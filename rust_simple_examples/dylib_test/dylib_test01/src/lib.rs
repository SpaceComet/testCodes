// `no_mangle` ensures the function name remains the same
// after compiled into a DLL
#[no_mangle]
pub extern "C" fn hello() {
    println!("Hello from DLL");
}

#[no_mangle]
pub extern "C" fn returns_number() -> i32 {
    8080
}

#[no_mangle]
pub extern "C" fn square(num: i32) -> i32 {
    num * num
}
