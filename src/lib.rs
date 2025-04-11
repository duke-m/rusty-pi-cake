//! # Pi Cake
//! A simple demo of a WebAssembly app together with a WebAssembly worker using Leptos, rust_i18n,
//! Tailwind 4 and web-sys, i.e. a full Rust WebAssembly stack.
//! It stupidly calculates Pi using the Leibniz formula and shows the result in a modal.
//! It does so by sending messages to the worker and receiving intermediate results and the final result.
//!
//! ## How it works
//! The app uses a WebAssembly worker to do the heavy lifting of calculating Pi.
//! After the worker is created it is passed a closure that handles messages from the worker.
//! The worker sends messages back to the main thread, the main thread receives these messages and updates the UI accordingly.
//! That's why the UI keeps responsive while the worker is doing the heavy lifting.
//! Without the worker the UI would be blocked.
//! The WASM worker itself is loaded using a separate JS file.
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

mod components;
mod constants;
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
