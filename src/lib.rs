//! # TODO MVC
//!
//! Implementing TODO MVC in Rust using wasm-bindgen and web-sys
//! to learn how MVC works in this language.
//!
//! Example comes from [The `wasm-bindgen` Guide](https://rustwasm.github.io/docs/wasm-bindgen/examples/todomvc.html)
pub use wasm_bindgen::prelude::*;

pub use std::rc::Rc;

/// Controller of the program.
pub mod controller;
/// Schedules messages to the `Controller` and `View`.
pub mod scheduler;
/// Stores item into local storage.
pub mod store;
/// Presentation layer.
pub mod view;

// Imports neccesary structs from the modules declared above.
pub use crate::controller::{Controller, ControllerMessage};
pub use crate::scheduler::Scheduler;
pub use crate::store::Store;
pub use crate::view::{View, ViewMessage};

/// Message wrapper enum used to pass through the scheduler to the Controller or View.
pub enum Message {
    /// Message wrapper to send to the controller.
    Controller(ControllerMessage),
    /// Message wrapper to send to the view.
    View(ViewMessage),
}

/// Runs the app.
///
/// 1. Creates a `Scheduler`, `Store`, `Controller`, and `View`.
/// 2. `Controller` takes the `Store`.
/// 3. `Scheduler` takes the `Controller` and `View`.
/// 4. `Scheduler` adds an message to the event stack, intended for `Controller`.
pub fn app(name: &str) {
    // Creates referenced counted pointer a new `Scheduler`.
    let sched = Rc::new(Scheduler::new());
    // Creates a new `Store`.
    let store = match Store::new(name) {
        // Assigns store to `Store` if it exists.
        Some(s) => s,
        // Otherwise, return from `app`.
        None => return,
    };
    // Initializes the Controller.
    //
    // Rc::downgrade(&sched) creates a `Weak` pointer to the scheduler
    // and the pointer has type Weak<Scheduler>
    // TODO(benlee12): Why do we need a weak pointer?
    //
    // Moves store and a weak pointer to sched.
    let controller = Controller::new(store, Rc::downgrade(&sched));
    // sched.clone() returns a copy of `sched`, which had type Rc<Scheduler>
    // TODO(benlee12): Check that this is equivalent to Rc::clone(&sched)
    // View::new() returns an Option<View>, so the type of `view` is View.
    if let Some(mut view) = View::new(Rc::clone(&sched)) {
        // sch is a immutable reference to `sched`.
        let sch: &Rc<Scheduler> = &sched;
        // Initializes the View.
        view.init();
        // Sets the View field for scheduler.
        sch.set_view(view);
        // Sets the Controller field for scheduler.
        sch.set_controller(controller);
        // Adds an SetPage message to the Scheduler to be sent to Controller.
        //
        // "".to_string() converts &str "" to a String.
        // It gets wrapped around the enum ControllerMessage::SetPage variant.
        // This is further wrapped by the Message::Controller variant.
        // This Message is passed to add_message() which adds the message
        // to the event stack.
        sched.add_message(Message::Controller(ControllerMessage::SetPage(
            "".to_string(),
        )));
    }
}

/// Entry point into the program from JavaScript.
///
/// The `start` attributes configures the start section of the wasm executable so that
/// the `run` function executes as soon as the wasm module is instantiated.
///
/// Uses `Result` as the return type to enable handling of JS exceptions with `?`,
/// naturally propagting it upwards to the wasm boundary.
#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    // Sets the `console.error` panic hook.
    console_error_panic_hook::set_once();
    app("todos-wasmbindgen");

    Ok(())
}
