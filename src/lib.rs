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

use leptos::{logging::log, prelude::*};
use rust_i18n::t;
use wasm_bindgen::prelude::Closure;
use web_sys::MessageEvent;
// Single-threaded reference-counting pointers. 'Rc' stands for 'Reference Counted'.
// We need for the worker to be shared between the view and the button click handler:
use std::rc::Rc;
use web_time::Instant;

use components::{CalcModal, PrecisionInput, ProgressBar, ReLang, ReTitle};
use constants::*;
use laborer::{Laborer, WorkerCommand, WorkerResponse};
use types::*;
pub mod laborer;

mod calculate;
mod components;
mod constants;
mod helpers;
pub mod types;

// Load the locales from the locales directory.
rust_i18n::i18n!("locales");

/// This function sets up the worker and registers the onmessage callback.
fn setup_worker(
    result: WriteSignal<TApproximation>,
    iteration: WriteSignal<TPrecision>,
    calculating: WriteSignal<bool>,
    time: ReadSignal<Instant>,
    duration: WriteSignal<f64>,
) -> Rc<Laborer> {
    // Set up the worker outside the view.
    // Rc is a reference counted pointer, which allows us to share the worker between the view and the button click handler.
    let onmessage_callback = Closure::<dyn FnMut(MessageEvent)>::new(move |event: MessageEvent| {
        let js_value = event.data();
        let response: WorkerResponse = serde_wasm_bindgen::from_value(js_value).unwrap();
        match response {
            WorkerResponse::Result(val) => {
                #[cfg(debug_assertions)]
                log!("Got result: {}", val);
                result.set(val.result);
                iteration.set(val.iteration);
            }
            WorkerResponse::Pong => log!("Got pong"),
            WorkerResponse::Ready => {
                duration.set(time.get().elapsed().as_secs_f64());
                #[cfg(debug_assertions)]
                log!("Worker is ready!");
                calculating.set(false);
            }
        }
    });
    let worker = Rc::new(Laborer::new(WORKER_LOADER_URL, onmessage_callback));
    worker.send_command(WorkerCommand::Initialize);
    worker.forget(); // Keep closures alive
    worker
}

/// This is the main App component, which will be mounted to the body of the HTML document.
/// It will calculate Pi/4 with a given precision and show the duration and the approximation.
#[component]
pub fn App() -> impl IntoView {
    // create updatable signals for the duration, value/approximation, precision, and calculating state
    let (duration, set_duration) = signal(0.0);
    let (time, set_time) = signal(Instant::now());
    let (value, set_value) = signal(0.0 as TApproximation);
    let (precision, set_precision) = signal(DEFAULT_PRECISION);
    let (calculating, set_calculating) = signal(false);
    let (iteration, set_iteration) = signal(0 as TPrecision);

    let worker = setup_worker(
        set_value,
        set_iteration,
        set_calculating,
        time,
        set_duration,
    );

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
        <CalcModal>
        <ProgressBar
            iteration=iteration
            iteration_max=precision.get().pow(10)
            />
        </CalcModal>
        </Show>
        <h1>"Rust WASM Pi Cake"</h1>

        <PrecisionInput set_precision=set_precision />

        <button
            on:click=move |_| {
                set_time.set(Instant::now());
                let worker = Rc::clone(&worker_for_click);
                worker.send_command(WorkerCommand::CalculatePi(precision.get()));
                *set_calculating.write() = true;
            }>
            {t!("start")}
        </button>
        <Show when=move || (value.get() > 0.0)>
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
        </Show>
    }
}
