pub use crate::controller::Controller;
pub use crate::view::View;
pub use crate::Message;
pub use std::cell::RefCell;
pub use std::rc::Rc;

/// Creates an event loop that starts each time a message is added.
pub struct Scheduler {
    pub controller: Rc<RefCell<Option<Controller>>>,
    pub view: Rc<RefCell<Option<View>>>,
    pub events: RefCell<Vec<Message>>,
    pub running: RefCell<bool>,
}

impl Scheduler {
    /// Constructs a new `Scheduler`
    ///
    /// ```
    /// Scheduler {
    ///     controller: Rc::new(RefCell::new(None)),
    ///     view: Rc::new(RefCell::new(None)),
    ///     events: RefCell::new(Vec::new()),
    ///     running: RefCell::new(false),
    /// }
    /// ```
    ///
    /// Each field is wrapped by `Rc` and `RefCell`.
    /// The `controller` and `view` fields are left as `None` because they are
    /// set later by [`set_controller`][set_controller] and [`set_view`][set_view]
    /// respectively.
    ///
    /// [set_controller]: struct.Scheduler.html#method.set_controller
    /// [set_view]: struct.Scheduler.html#method.set_view
    pub fn new() -> Scheduler {
        Scheduler {
            controller: Rc::new(RefCell::new(None)),
            view: Rc::new(RefCell::new(None)),
            events: RefCell::new(Vec::new()),
            running: RefCell::new(false),
        }
    }
    pub fn set_view(&self, view: View) {}
    pub fn set_controller(&self, controller: Controller) {}
    /// Add a new message onto the event stack.
    pub fn add_message(&self, message: Message) {}
}
