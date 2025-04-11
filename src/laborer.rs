//! This module defines the Laborer struct, which represents a worker that can perform calculations.
//! It also defines the WorkerCommand and WorkerResponse enums, which are used to communicate with the worker.
use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};
use wasm_bindgen::{JsCast, prelude::Closure};
use web_sys::{MessageEvent, Worker, WorkerOptions, WorkerType};

use crate::types::{TApproximation, TPrecision};

/// The WorkerCommand enum defines the commands that can be sent to the worker.
/// Only CalculatePi is used in this example, the rest are just for demonstration.
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum WorkerCommand {
    Initialize,
    CalculatePi(TPrecision),
    Multiply(i32, i32),
    Add(i32, i32),
    Ping,
}

/// The WorkerResult holds the result, like the value of Pi, and the iteration number.
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct WorkerResult {
    pub result: TApproximation,
    pub iteration: TPrecision,
}

/// The WorkerResponse enum defines the responses that can be sent back from the worker to the main thread.
/// If has to signal it is ready to receive commands, or send back the result of a calculation.
/// It can also send a Pong message to indicate that it is alive and responsive.
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum WorkerResponse {
    Ready,
    Result(WorkerResult),
    Pong,
}

/// Implement the Display trait for convenience.
impl Display for WorkerCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Implement the Display trait for convenience.
impl Display for WorkerResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Result: {}, Iteration: {}", self.result, self.iteration)
    }
}

impl WorkerResult {
    pub fn new(result: TApproximation, iteration: TPrecision) -> Self {
        WorkerResult { result, iteration }
    }
}

/// The Laborer struct represents a worker that can perform calculations.
pub struct Laborer {
    worker: Worker,
    onmessage_callback: Closure<dyn FnMut(MessageEvent)>,
    state: WorkerState,
}

impl Laborer {
    /// This is a workaround to keep the worker alive
    /// and not drop it when the function returns.
    /// The worker will be dropped when the closure is dropped.
    pub fn forget(&self) {
        let _ = self.onmessage_callback;
    }

    pub fn set_callback(&mut self, callback: Closure<dyn FnMut(MessageEvent)>) {
        self.onmessage_callback = callback;
    }

    pub fn is_ready(&self) -> bool {
        matches!(self.state, WorkerState::Ready)
    }

    /// Create a new worker aka Laborer instance.
    pub fn new(worker_script_url: &str, onmessage_callback: Closure<dyn FnMut(MessageEvent)>) -> Self {
        let options = WorkerOptions::new();
        // using module workers, see also https://developer.mozilla.org/en-US/docs/Web/API/Worker/Worker
        options.set_type(WorkerType::Module);
        let worker = Worker::new_with_options(&worker_script_url, &options).expect("failed to spawn worker"); 
        worker.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));

        Self {
            worker,
            onmessage_callback,
            state: WorkerState::Unknown,
        }
    }
}

impl Laborer {
    /// Send a command from the main thread to the worker.
    pub fn send_command(&self, command: WorkerCommand) {
        let js_value = serde_wasm_bindgen::to_value(&command).unwrap();
        self.worker.post_message(&js_value).unwrap();
    }
}

/// This enum represents the state of the worker.
#[allow(dead_code)]
#[derive(Clone)]
enum WorkerState {
    Ready,
    Unknown,
    Busy,
}

