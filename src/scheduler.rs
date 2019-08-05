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
        // Add new message to the call stack.
        {
            // Tries to mutably borrow the wrapped vector of messages.
            // Notice that `mut` keyword in the pattern. This is declaring the
            // moved borrow as `mut` so that we can mutate the moved object.
            //
            // Do not get confused between interior mutability of RefCell
            // and exterior mutability of RefMut. You may be wondering why
            // `mut` is needed here if events is a RefCell. Wasn't the whole
            // purpose of RefCell to allow interior mutability? Well it is, and
            // in fact, that is being used here, but its subtle. Notice that
            // self is immutable. This means that it should be impossible to
            // modify any of the fields of self, including events. This is
            // where RefCell comes in. Using unsafe code, it returns a RefMut,
            // which is a wrapper for the mutably borrowed Vec<Message>.
            // Now there is no more unsafe code! Everything after now will
            // now obey the normal mutability rules. If we declare `events`
            // without the `mut` keyword, then `events` in immutable.
            // Therefore, its field, even though it is a mutable reference,
            // is immutable. By using the `mut` keyword, the RefMut struct is
            // mutable, so the underlying value is also mutable.
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
    pub fn run(&self) {
        let mut events_len = 0;
        {
            // Tries to mutably borrow the wrapped vector of messages.
            if let Ok(events) = self.events.try_borrow() {
                // The borrow was successful, assigns actual event length.
                // Even though clone() is not needed, explicitly writing it
                // makes the intent clearer.
                events_len = events.len().clone();
            } else {
                // The vector is currently mutably borrowed.
                exit("this might be a deadlock");
            }
        }
        if events_len == 0 {
            // There are no more events in the call stack, so turn off running.
            if let Ok(mut running) = self.running.try_borrow_mut() {
                // The borrow was successful, set bool to false.
                *running = false;
            } else {
                // The bool is already borrowed, so log error.
                exit("This might be a deadlock");
            }
        } else {
            // There are still events in the call stack, so turn on running.
            // TODO(benlee12): Why create an extra scope?
            {
                // If `mut` is removed, the compiler will complain about
                // mutably borrowing a immutable value. This is because of
                // deref coercion on `running`. Since deref_mut needs to
                // mutably borrow `running`, `running` must be declared as
                // `mut`. This makes sense since we are changing its value
                // after all!
                if let Ok(mut running) = self.running.try_borrow_mut() {
                    // The borrow was successful, set bool to true.
                    *running = true;
                } else {
                    // The bool is already borrowed, so log error.
                    exit("This might be a deadlock");
                }
            }
            self.next_message();
        }
    }

    /// Pops the call stack and handles the call appropriately.
    pub fn next_message(&self) {
        // Pops the call stack for a Message.
        let event = {
            // Tries to borrow the list of callbacks.
            if let Ok(mut events) = self.events.try_borrow_mut() {
                // Deref coercion, pops the call stack, returns the potentially
                // popped event as an Option<Option<Message>>
                //
                // If the vector is not empty, the return type will be
                // Some(Some(Message)).
                //
                // If the vector is empty, the return type will be
                // Some(None);
                //
                // The outer Some indicates the success of the borrow.
                Some(events.pop())
            } else {
                // Borrow was not successful, events already borrowed.
                exit("This might be a deadlock");
                None
            }
        };
        // Based on Message, Scheduler determines which function to call in
        // either View or Controller, which will handle the Message itself.
        if let Some(Some(event)) = event {
            match event {
                // Pattern matching:
                // PATTERN = Message::Controller(controller::ControllerMessage)
                // EXPRESS = Message::Controller(e)
                // e = controller::ControllerMessage;
                Message::Controller(e) => {
                    // Tries to mutably borrow the controller.
                    if let Ok(mut controller) = self.controller.try_borrow_mut() {
                        // Borrow successful,
                        // controller = RefMut<Option<Controller>>
                        if let Some(ref mut ag) = *controller {
                            ag.call(e);
                        }
                    } else {
                        exit("This might be a deadlock");
                    }
                }
                Message::View(e) => {
                    // self.view = Rc<RefCell<Option<View>>>
                    // Deref coercion -> RefCell<Option<View>>
                    // try_borrow_mut(&self) -> Result<RefMut<Option<View>>>
                    // view = RefMut<Option<View>>
                    if let Ok(mut view) = self.view.try_borrow_mut() {
                        // Note: the ref mut keyword changes the usual move to
                        // a mutable borrow. Usually match statements moves the
                        // value.
                        //
                        // Pattern Matching
                        // PATTERN = Some(ref mut ag)
                        // EXPRESS = Option<View>
                        // ag = &mut View
                        if let Some(ref mut ag) = *view {
                            // Calls function on a view based on message `e`.
                            ag.call(e);
                        }
                    } else {
                        exit("This might be a deadlock");
                    }
                }
            }
            // Continues to the next iteration of run().
            self.run();
        } else if let Ok(mut running) = self.running.try_borrow_mut() {
            // Since we couldn't find a new Message, we cleared the event
            // stack, so we should set running to false.
            *running = false;
        } else {
            // Deadlock cause running is borrowed elsewhere.
            exit("This might be a deadlock");
        }
    }
}
