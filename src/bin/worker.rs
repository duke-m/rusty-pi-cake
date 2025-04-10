//! This is a Web Worker that handles messages from the main thread.
//! It processes commands and sends back responses.
use leptos::logging::log;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{DedicatedWorkerGlobalScope, MessageEvent};
use leptos_demo::laborer::*;

fn main() {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    #[cfg(debug_assertions)]
    log!("worker starting");

    let scope = DedicatedWorkerGlobalScope::from(JsValue::from(js_sys::global()));
    let scope_clone = scope.clone();

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
        let response = match command {
            WorkerCommand::Add(a, b) => {
                let result = a + b;
                WorkerResponse::Result(WorkerResult::new(result as f64, 0))
            },
            WorkerCommand::Multiply(a, b) => {
                let result = a * b;
                WorkerResponse::Result(WorkerResult::new(result as f64, 0))
            },
            WorkerCommand::CalculatePi(precision) => {
                // Pi calculation logic here
                let mut result: f64 = 0.0;
                
                for n in 0..precision.pow(10) {
                    let mut z = 1.0 / (2 * n + 1) as f64;
                    if n % 2 == 1 {
                        z = -z;
                    }
                    result += z;
                    
                    if n % 100000 == 0 {
                        #[cfg(debug_assertions)]
                        log!("{}: {}", n, result);
                        
                        // Send intermediate result
                        let progress_response = WorkerResponse::Result(
                            WorkerResult::new(4.0 * result, n as u64)
                        );
                        let js_value = serde_wasm_bindgen::to_value(&progress_response).unwrap();
                        scope_clone.post_message(&js_value).unwrap();
                    }
                }
                
                WorkerResponse::Result(WorkerResult::new(4.0 * result, precision as u64))
            },
            WorkerCommand::Ping => WorkerResponse::Pong,
            WorkerCommand::Initialize => {
                #[cfg(debug_assertions)]
                log!("Worker initialized");
                WorkerResponse::Ready
            }
        };

        // Send response using serde_wasm_bindgen
        let response_js = serde_wasm_bindgen::to_value(&response).unwrap();
        scope_clone.post_message(&response_js).unwrap();
    }) as Box<dyn Fn(MessageEvent)>);
    
    scope.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
    onmessage.forget();
    

    // Send ready message
    let ready_msg = WorkerResponse::Ready;
    let js_value = serde_wasm_bindgen::to_value(&ready_msg).unwrap();
    scope.post_message(&js_value).unwrap();
}
