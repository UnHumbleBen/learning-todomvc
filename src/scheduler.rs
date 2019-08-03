pub use crate::view::View;
pub use crate::controller::Controller;
pub use crate::Message;

/// Creates an event loop that starts each time a message is added.
pub struct Scheduler {}

impl Scheduler {
    pub fn new() -> Scheduler {
        Scheduler {}
    }
    pub fn set_view(&self, view: View) {}
    pub fn set_controller(&self, controller: Controller) {}
    /// Add a new message onto the event stack.
    pub fn add_message(&self, message: Message) {}
}
