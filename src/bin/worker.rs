//! This is a Web Worker that handles messages from the main thread.
//! It processes commands and sends back responses.

#[cfg(debug_assertions)]
use leptos::logging::log;
use leptos_demo::{
    laborer::*,
    types::{TApproximation, TPrecision},
};
use wasm_bindgen::{JsCast, prelude::*};
use web_sys::{DedicatedWorkerGlobalScope, MessageEvent};

/// The base calculation function for the Pi approximation.
/// It uses the formula: 1/(2n+1) for odd n and -1/(2n+1) for even n.
/// This is a simple implementation of the Leibniz formula for Pi.
/// `#[inline(always)]` is used to suggest to the compiler to inline this function for performance.
#[inline(always)]
fn base_calculation(n: TPrecision) -> TApproximation {
    let mut z = 1.0 / (2 * n + 1) as f64;
    if n % 2 == 1 {
        z = -z;
    }
    z
}

fn main() {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    #[cfg(debug_assertions)]
    log!("worker starting");

    // see also https://developer.mozilla.org/en-US/docs/Web/API/DedicatedWorkerGlobalScope
    let scope = DedicatedWorkerGlobalScope::from(JsValue::from(js_sys::global()));
    let scope_clone = scope.clone();

    // Set up the worker to listen for messages
    // and process commands. The worker will be passed a closure that handles messages from the main thread.
    // We first Box the closure to put it on the heap for the worker to use, Closure::wrap is used to
    // convert the closure into a JavaScript function/closure.
    // The `move` keyword is used to capture the variables from the surrounding scope and move their ownership into the closure.
    let onmessage = Closure::wrap(Box::new(move |msg: MessageEvent| {
        #[cfg(debug_assertions)]
        log!("got message");

        // Parse command using serde_wasm_bindgen
        let js_value = msg.data();
        let command: WorkerCommand = match serde_wasm_bindgen::from_value(js_value) {
            Ok(cmd) => cmd,
            Err(e) => {
                web_sys::console::error_1(&format!("Error parsing command: {:?}", e).into());
                return;
            }
        };

        #[cfg(debug_assertions)]
        log!("Worker received command: {}", command.to_string());

        // Process command
        let match_result: Option<WorkerResponse> = match command {
            WorkerCommand::Add(a, b) => 
                Some(WorkerResponse::Result(WorkerResult::new((a + b) as f64, 0))),
            WorkerCommand::Multiply(a, b) => 
                Some(WorkerResponse::Result(WorkerResult::new((a * b) as f64, 0))),
            WorkerCommand::CalculatePi(precision) => {
                // Pi calculation logic here
                let mut result: TApproximation = 0.0;
                let check_step: TPrecision = (precision-2).pow(10); // report back from time to time

                for n in 0..precision.pow(10) {
                    result += base_calculation(n);

                    if n % check_step == 0 {
                        #[cfg(debug_assertions)]
                        log!("{}: {}", n, result);
                        // Send intermediate result
                        let progress_response =
                            WorkerResponse::Result(WorkerResult::new(4.0 * result, n as u64));
                        let js_value = serde_wasm_bindgen::to_value(&progress_response).unwrap();
                        scope_clone.post_message(&js_value).unwrap();
                    }
                }
                Some(WorkerResponse::Ready)
            }
            WorkerCommand::Ping => Some(WorkerResponse::Pong),
            WorkerCommand::Initialize => {
                #[cfg(debug_assertions)]
                log!("Worker initialized");
                Some(WorkerResponse::Ready)
            }
        };

        // send response if there is one:
        if let Some(response) = match_result {
            #[cfg(debug_assertions)]
            log!("Sending match response: {:?}", response);
            let js_value = serde_wasm_bindgen::to_value(&response).unwrap();
            scope_clone.post_message(&js_value).unwrap();
        }
    }) as Box<dyn Fn(MessageEvent)>);

    // Set the onmessage handler for the worker, use as_ref() to convert the type into a shared reference
    // of the (usually inferred) input type. See also https://doc.rust-lang.org/std/convert/trait.AsRef.html
    // and https://rustwasm.github.io/wasm-bindgen/api/wasm_bindgen/trait.JsCast.html#method.unchecked_ref
    scope.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
    onmessage.forget();

    // Send serialized WorkerResponse::Ready message to the main thread
    let ready_msg = WorkerResponse::Ready;
    let js_value = serde_wasm_bindgen::to_value(&ready_msg).unwrap();
    scope.post_message(&js_value).unwrap();
}

/// some basic unit tests to show how tests are done,
/// note that the tests are not run in the worker context which would require to use something like
/// `wasm-bindgen-test` or `wasm-pack test`
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_calculation_is_correct() {
        assert_eq!(base_calculation(0), 1.0);
        assert_eq!(base_calculation(1), -0.3333333333333333);
        assert_eq!(base_calculation(2), 0.2);
        assert_eq!(base_calculation(3), -0.14285714285714285);
    }

    #[test]
    fn worker_command_serializes() {
        let command = WorkerCommand::Add(1, 2);
        assert_eq!(command.to_string(), "Add(1, 2)");
    }
}