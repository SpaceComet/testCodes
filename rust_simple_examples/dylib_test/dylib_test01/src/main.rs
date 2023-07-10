use libloading::{Library, Symbol};

fn main() {
    println!("Hello from main");

    unsafe {
        let lib = Library::new("dylib_test01.dll").expect("Failed loading the lib");

        // Load the symbol of the hello function
        let hello: Symbol<unsafe extern "C" fn()> =
            lib.get(b"hello").expect("Failed to load symbol");

        hello();

        // Load the symbol of the returns_number function
        let returns_number: Symbol<unsafe extern "C" fn() -> i32> =
            lib.get(b"returns_number").expect("Failed to load symbol");

        println!("Number returned from DLL is: {}", returns_number());

        // load the symbol square.
        let square: Symbol<unsafe extern "C" fn(i32) -> i32> =
            lib.get(b"square").expect("Failed to load symbol");

        let r_num: i32 = 10;
        println!("The square of {} is {}", r_num, square(r_num));
    }
}
