pub use crate::controller::Controller;
pub use crate::exit;
pub use crate::view::View;
pub use crate::Message;
pub use std::cell::RefCell;
pub use std::rc::Rc;

/// Creates an event loop that starts each time a message is added.
///
/// # Fields
///
/// - `controller` is a wrapped `Controller`.
/// - `view` is a wrapped `View`.
/// - `events` is a wrapped call stack.
/// - `running` is a wrapped boolean, whose value is `true` when the `Scheduler`
///   is running and `false` when it is not.
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
    ///
    /// Triggers running the event loop if it's not already running.
    pub fn add_message(&self, message: Message) {
        // Assigns `running` to be the unwrapped running field.
        let running = {
            // Tries to immutably borrow the wrapped bool.
            if let Ok(running) = self.running.try_borrow() {
                // The borrow was successful, so clone the bool.
                // Note that deref coercion is used here since running is of
                // type Ref<bool> which implements the Deref trait.
                // TODO(benlee12): Does *running work?
                // Potential answer: Yes, but only because bool implement the
                // Copy trait.
                running.clone()
            } else {
                // The bool is currently mutably borrowed, so log error.
                // TODO(benlee12): Why is it deadlock?
                // I suspect it is because another function is currently
                // modifying the bool, so it is a deadlock but not in the sense
                // used in concurrency.
                exit("This might be a deadlock");
                false
            }
        };
        // TODO(benlee12): Why the extra scope? For code symmetry?
        {
            // Tries to mutably borrow the wrapped vector of messages.
            if let Ok(mut events) = self.events.try_borrow_mut() {
                // The borrow was successful, add message to the call stack.
                events.push(message);
            } else {
                // The vector is currently mutably borrowed.
                exit("This might be a deadlock");
            }
        }
        // Triggers running the event loop if it's not already running.
        if !running {
            self.run();
        }
    }

    /// Start the event loop, taking messages from the stack to run.
    pub fn run(&self) {}
}
