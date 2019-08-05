pub use crate::{Scheduler, Store};
pub use std::cell::RefCell;
pub use std::rc::Weak;

/// The controller of the application.
///
/// Turns page state into functionality.
///
/// # Fields
///
/// - `store` is the struct that stores item into `localStorage`.
/// - `sched` is a reference cell to the weak pointer to the scheduler.
///   - TODO(benlee12): The Option could possibly be for deallocation.
/// - TODO(benlee12): Fill in the last two fields.
/// - `active_route`
/// - `last_active_route`
pub struct Controller {
    pub store: Store,
    pub sched: RefCell<Option<Weak<Scheduler>>>,
    pub active_route: String,
    pub last_active_route: String,
}

impl Controller {
    /// Initializes a new `Controller` that takes local storage maintainer
    /// `store` and a weak pointer to the Scheduler `sched` as its fields.
    pub fn new(store: Store, sched: Weak<Scheduler>) -> Controller {
        Controller {
            store,
            sched: RefCell::new(Some(sched)),
            active_route: "".into(),
            last_active_route: "none".into(),
        }
    }

    /// Used by `Scheduler` to convert a `ControllerMessage` into a function
    /// call on a `Controller`.
    pub fn call(&mut self, method_name: ControllerMessage) {}
}

pub enum ControllerMessage {
    SetPage(String),
}
