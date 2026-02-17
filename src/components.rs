//! This module contains the leptos components used in the application.
use constants::*;
use leptos::prelude::*;
use leptos_meta::{Html, Title, provide_meta_context};
use rust_i18n::t;
use types::*;

use crate::{constants, types};

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

/// Precision input component.
#[component]
pub fn PrecisionInput(set_precision: WriteSignal<TPrecision>) -> impl IntoView {
    view! {
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
    }
}

#[component]
pub fn ProgressBar(iteration: ReadSignal<TPrecision>, iteration_max: TPrecision) -> impl IntoView {
    view! {
        <progress
            value=move || iteration.get()
            max=iteration_max />
    }
}

/// Displays a full screen information modal, shows how to use children nodes.
#[component]
pub fn CalcModal(children: Children) -> impl IntoView {
    view! {
        <div class="biggy">
        <div><h3>{t!("calculating")}</h3>{children()}</div>
        </div>
    }
}
