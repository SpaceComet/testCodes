use tracing::{debug, event, info, instrument, trace, Level};
use tracing_subscriber;

// when adding `#[instrument]` the logs will have more information.
// The log will have the name of the function and the parameters
// with    = DEBUG power{base=2 exp=5}: tracing_simple_test: enter power
// without = DEBUG tracing_simple_test: enter power
#[instrument]
fn power(base: i64, exp: i64) -> i64 {
    debug!("enter power");
    let mut result = base;

    for i in 1..exp {
        result *= base;
        trace!(" {}  result = {}", i, result);
    }

    result
}

#[instrument]
fn power_p2(base: i64, exp: i64) -> i64 {
    power(base + 2, exp + 2) + 2
}

fn main() {
    // The subscriber will "consume" the logs
    // `with_max_level` will only "consume" logs <= than the one set by parameter
    // `init()` makes the subscriber to be global. This means that by default it will
    // consume all the logs, in all threads.
    let _subscriber = tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .init();

    info!("FLAG-01");

    event!(Level::INFO, "FLAG-02");

    println!(" power {} ", power(2, 5));

    println!(" power + 2! {} ", power_p2(2, 5));
}
