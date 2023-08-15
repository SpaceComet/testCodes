use libloading::{Library, Symbol};
use tracing::{debug, info, Level};
use tracing_subscriber;

mod logs;

fn main() {
    let log_level = Level::TRACE;

    // The subscriber will "consume" the logs
    // `with_max_level` will only "consume" logs <= than the one set by parameter
    // `init()` makes the subscriber to be global. This means that by default it will
    // consume all the logs, in all threads.
    tracing_subscriber::fmt().with_max_level(*&log_level).init();

    let log_settings = logs::LogObj::new(Some(Level::TRACE), Some("test".to_string()));

    unsafe {
        // Load the dll
        let lib = Library::new("tracing_dylib_test.dll").expect("Failed loading the lib");
        info!("dll loaded");

        // Load the symbol of the power function
        // (Just a simple function)
        let power: Symbol<fn(base: i64, exp: i64, sub: Option<logs::LogObj>) -> i64> =
            lib.get(b"power").expect("Failed to load symbol");
        info!("power function loaded");

        // Execute the function
        info!("executing the power function...");
        let power_result = power(5, 2, Some(log_settings));

        info!("power function executed");
        debug!("power function result = {}", power_result);
    }
}
