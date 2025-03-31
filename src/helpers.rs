use std::time::Duration;
use leptos::prelude::set_timeout;

use crate::constants::DELAY_MS;

/// Default delay for letting the browser breathe.
/// Lets the browser paint the UI before the calculation is blocking the main thread.
pub fn delayed_execution(f: impl Fn() + 'static) {
    set_timeout(f, Duration::from_millis(DELAY_MS));
}