// "Borrowed" from Quicksilver as I want to know the time.

#[cfg(not(target_arch = "wasm32"))]
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(target_arch = "wasm32")]
use stdweb::web::Date;

#[cfg(not(target_arch = "wasm32"))]
pub fn current_time() -> f64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_secs() as f64 * 1000.0 + since_the_epoch.subsec_nanos() as f64 / 1e6
}

#[cfg(target_arch = "wasm32")]
pub fn current_time() -> f64 {
    Date::now()
}
