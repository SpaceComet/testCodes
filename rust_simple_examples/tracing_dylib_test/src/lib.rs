mod logs;
use tracing::{debug, instrument, span, trace};

// `no_mangle` ensures the function name remains the same
// after compiled into a DLL

// when adding `#[instrument]` the logs will have more information.
// The log will have the name of the function and the parameters
// with    = DEBUG power{base=2 exp=5}: tracing_simple_test: enter power
// without = DEBUG tracing_simple_test: enter power
#[no_mangle]
#[instrument]
fn power(base: i64, exp: i64, logs: Option<logs::LogObj>) -> i64 {
    if let Some(logs_settings) = logs {
        tracing_subscriber::fmt()
            .with_max_level(logs_settings.log_max_level)
            .init();
    }

    let span = span!(tracing::Level::TRACE, "power", base, exp);
    let _enter = span.enter();
    trace!("test!!");

    debug!("entered power");
    let mut result = base;

    for i in 1..exp {
        result *= base;
        trace!(" {}  result = {}", i, result);
    }

    if base == 5 {
        let _test = power_p2(base, exp);
    }

    result
}

#[no_mangle]
#[instrument]
fn power_p2(base: i64, exp: i64) -> i64 {
    power(base + 2, exp + 2, None) + 2
}
