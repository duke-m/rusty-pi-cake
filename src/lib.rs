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

use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Html, Title};
use rust_i18n::t;

mod types; use types::*;
mod constants; use constants::*;
mod calculate; use calculate::calculate_and_update_signals;
mod helpers; use helpers::delayed_execution;

// Load the locales from the locales directory.
rust_i18n::i18n!("locales");

/// The ReLang component sets the language of the HTML document for demonstration purposes.
/// Reade more about the `provide_meta_context` and other functions in the leptos_meta crate.
#[component]
pub fn ReLang(language: String) -> impl IntoView {
    provide_meta_context();
    view! {
        <Html
        {..}
        lang=language
    />
}
}

/// The ReTitle component sets the title of the HTML document for demonstration purposes.
#[component]
pub fn ReTitle(text: String) -> impl IntoView {
    provide_meta_context();
    view! {
        <Title text />
}
}

/// The CalcModal component is a simple modal that shows a "Calculating" message.
#[component]
fn CalcModal() -> impl IntoView {
    view! {
    <div class="biggy">
    <div><h3>{t!("calculating")}</h3></div>
    </div>
    }
}

/// This is the main App component, which will be mounted to the body of the HTML document.
/// It will calculate Pi/4 with a given precision and show the duration and the approximation.
#[component]
pub fn App() -> impl IntoView {
    // create updatable signals for the duration, value/approximation, precision, and calculating state
    let (duration, set_duration) = signal(0.0);
    let (value, set_value) = signal(0.0 as TApproximation);
    let (precision, set_precision) = signal(DEFAULT_PRECISION);
    let (calculating, set_calculating) = signal(false);

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
            delayed_execution(move || {
                calculate_and_update_signals(set_duration, set_value, set_calculating, precision);
            });
        }
        </Show>
        <h1>"Rust WASM Pi Cake"</h1>

        // input field for the precision, use TW's group hover
        <div class="group/precision">
            <input
                id="precision_input"
                prop:value=DEFAULT_PRECISION
                type="number"
                on:input:target=move |ev| {
                    set_precision.set(ev.target().value().parse::<TPrecision>().unwrap_or(0)); // parse or "fail" with 0
                }
            />
            <div class="my-group-precision-hover capitalize">{t!("precision")}</div>
        </div>
        // label for screen reader only
        <label for="precision_input">{t!("precision")}</label>

        <button
            on:click= move |_| {
                delayed_execution(move || {
                    *set_calculating.write() = true;
                });
            }>
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
