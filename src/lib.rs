//! # Pi Cake
//! A simple example of calculating Pi using a Leptos web app, just for measuring the time it takes.
//! This example uses the web_time crate to measure the time it takes to calculate Pi to compare it with ECMAScript.
//! There is no progress bar to avoid overhead in the measurement (we just want the calculation).
//!
//! This is not meant to be used in production, it's just a demonstration of how to use Leptos (with client side rendering),
//! especially when you're new to Rust. It will also demonstrate how to use Tailwind CSS and i18n (resp. L10n) in such an environment.
//!
//! ## How it works
//! Most of the code will be executed in the browser using WebAssembly.
//! The calculation is done in a blocking way, which is not recommended for web apps! To measure the time it takes to calculate Pi,
//! we use the web_time crate, which is a wrapper around the Performance.now() function in the browser. While this sounds
//! simple for someone coming from ECMAScript, it's not that obvious in Rust with the WASM target.
//! The web_time crate only works with the wasm32-unknown-unknown.
//!
//! ## How to run
//! You can run this example with the trunk package manager. Just install it with `cargo install trunk`.
//! Install some dependencies with npm or yarn.
//! Install the wasm32-unknown-unknown target with `rustup target add wasm32-unknown-unknown`.
//!
//! Now you can run the app with `trunk serve` and open it in your browser:
//! `trunk serve --open` will build the app, bundle it and open it in your default browser.
//! `trunk build --release` will build the app in release mode, you can combine it with serve, too.
//!
//! ## Generate documentation
//! You can generate the documentation with `cargo doc --open`.
//! If you prefer you can include private items with `cargo doc --document-private-items --open` (might be preferred for learning purposes).

use components::{CalcModal, PrecisionInput, ReLang, ReTitle};
use leptos::prelude::*;
use rust_i18n::t;
use laborer::{Laborer, WorkerCommand};

use std::rc::Rc;

pub mod types;
use types::*;
mod constants;
use constants::*;
mod calculate;
mod helpers;
use helpers::delayed_execution;
pub mod laborer;

mod components;

// Load the locales from the locales directory.
rust_i18n::i18n!("locales");


/// This is the main App component, which will be mounted to the body of the HTML document.
/// It will calculate Pi/4 with a given precision and show the duration and the approximation.
#[component]
pub fn App() -> impl IntoView {
    // Set up the worker outside the view
    let worker = Rc::new(Laborer::new(WORKER_LOADER_URL));
    worker.send_command(WorkerCommand::Initialize);
    worker.forget(); // Keep closures alive

    // worker.forget();
    // create updatable signals for the duration, value/approximation, precision, and calculating state
    let (duration, set_duration) = signal(0.0);
    let (value, set_value) = signal(0.0 as TApproximation);
    let (precision, set_precision) = signal(DEFAULT_PRECISION);
    let (calculating, set_calculating) = signal(false);

        // Clone the worker for button click
        let worker_for_click = Rc::clone(&worker);

    view! {
        // set the language of the HTML document dynamically, you can use signals to reactively change the language and other meta tags
        <ReLang language=rust_i18n::locale().to_string() />
        // set the title for demonstration
        <ReTitle text=format!("{}s set", precision.get().to_string()) />
        // only show the modal when calculating
        <Show
        when=move || calculating.get()
        // needs no fallback=|| {}
        >
        <CalcModal />
        {

        }
        </Show>
        <h1>"Rust WASM Pi Cake"</h1>

        <PrecisionInput set_precision=set_precision />

        <button
            on:click=move |_| {
                let worker = Rc::clone(&worker_for_click);
                let precision = precision.get(); // Extract as plain type
                delayed_execution(move || {
                    worker.send_command(WorkerCommand::CalculatePi(precision));
                    *set_calculating.write() = true;
                });
            }
        >
            {t!("start")}
        </button>

        <p>
        <dl>
            <dt>{t!("approximation")}</dt>
            <dd>{move || value.get()}</dd>
            </dl>
        <dl>
            <dt>{t!("duration")}</dt>
            <dd>{move || format!("{:.4}", duration.get())}</dd>
        </dl>
        </p>
    }
}
