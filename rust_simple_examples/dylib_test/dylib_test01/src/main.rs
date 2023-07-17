use libloading::{Library, Symbol};

fn main() {
    println!("Hello from main");

    // I know two ways you can do this:
    // 1. You can put all the code inside of an "unsafe" block, like this:
    unsafe {
        // Load the dll
        let lib = Library::new("dylib_test01_lib.dll").expect("Failed loading the lib");

        // Load the symbol of the hello function
        // (Just a simple function)
        let hello: Symbol<fn()> = lib.get(b"hello").expect("Failed to load symbol");

        // Execute the function
        hello();

        // Load the symbol of the returns_number function
        // (Function that returns a value)
        let returns_number: Symbol<unsafe extern "C" fn() -> i32> =
            lib.get(b"returns_number").expect("Failed to load symbol");

        println!("Number returned from DLL is: {}", returns_number());

        // load the symbol square.
        // (Function that takes a parameter and return a value)
        let square: Symbol<unsafe extern "C" fn(i32) -> i32> =
            lib.get(b"square").expect("Failed to load symbol");

        let r_num: i32 = 10;
        println!("The square of {} is {}", r_num, square(r_num));
    }

    // 2. You could do "unsafe" only when needed.
    // Load the dll
    let lib = unsafe { Library::new("dylib_test01_lib.dll").expect("Failed loading the lib") };

    // load the symbol option_r.
    // (Function that takes two parameters and returns a rust Result)
    let div_op: Symbol<fn(i32, i32) -> Result<i32, String>> =
        unsafe { lib.get(b"option_r").expect("Failed to load symbol") };

    println!(" {}/{} is {:?}", 10, 2, div_op(10, 2).is_ok());
    println!(" {}/{} is {:?}", 10, 0, div_op(10, 0).is_ok());
}
