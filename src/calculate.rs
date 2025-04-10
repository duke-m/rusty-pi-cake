//! This module contains the logic to calculate Pi/4 using the Leibniz formula.
use leptos::{
    logging::log,
    prelude::{Get, ReadSignal, Write, WriteSignal},
};

/// The web-time library specifically targets browsers that support Performance.now()
/// with the wasm32-unknown-unknown target.
/// WASI doesn’t require support as it has it’s own native API to deal with std::time.
/// Using the default std::time::Instant will not work in the browser / with that target.
use web_time::Instant;

use crate::types::{TApproximation, TPrecision};

/// Do the expensive calculation, approximating Pi/4, and return the result and the duration.
/// Optionally include a logger function to log the progress.
fn calculate(precision: TPrecision, logger: Option<&impl Fn(&str)>) -> (TApproximation, f64) {
    let start = Instant::now();
    if logger.is_some() {
        logger.unwrap()("Calculating Pi/4...");
    }
    let mut result: TApproximation = 0.0;

    for n in 0..precision.pow(10) {
        let mut z = 1.0 / (2 * n + 1) as TApproximation;
        if n % 2 == 1 {
            z = -z;
        }
        result += z;
    }
    let duration = (Instant::now() - start).as_secs_f64();
    if logger.is_some() {
        logger.unwrap()(&format!("Calculation took: {:?}", duration));
    }
    // result is an approximated Pi/4

    (result * 4.0, duration)
}

/// Calculate the approximation of Pi/4 and update the signals.
pub fn calculate_and_update_signals(
    set_duration: WriteSignal<f64>,
    set_value: WriteSignal<TApproximation>,
    set_calculating: WriteSignal<bool>,
    precision: ReadSignal<TPrecision>,
) {
    let (value, duration) = calculate(precision.get(), Some(&logger)); // expensive!
    *set_duration.write() = duration;
    *set_value.write() = value;
    *set_calculating.write() = false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_calculates_correctly() {
        let precision = 1;
        let (result, duration) = calculate(precision, None::<&fn(&str)>);
        assert_eq!(result, 4.0);
        assert!(duration < 1.0);
    }

    #[test]
    fn it_calculates_correctly_with_logger() {
        let precision = 1;
        let (result, duration) = calculate(precision, Some(&logger));
        assert_eq!(result, 4.0);
        assert!(duration < 1.0);
    }

    #[test]
    fn it_calculates_correctly_with_higher_precision() {
        let precision = 5;
        let (result, duration) = calculate(precision, None::<&fn(&str)>);
        assert!(result > 3.14159);
        assert!(result < 3.14160);
        assert!(duration > 0.0001);
    }
}

/// Provide a wrapper to log to the console via the leptos logging system.
fn logger(s: &str) {
    log!("{}", s);
}
