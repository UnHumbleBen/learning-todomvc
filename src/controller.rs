// Controller needs access to Item and Store structs.
pub use crate::store::*;
// Controller needs to send messages to View.
pub use crate::view::ViewMessage;
// Needs to add messages to the Scheduler.
pub use crate::{Message, Scheduler};
// Used for generating ids.
pub use js_sys::Date;
pub use std::cell::RefCell;
pub use std::rc::Weak;

/// The controller of the application.
///
/// Turns page state into functionality.
///
/// # Fields
///
pub struct Controller {
    /// the struct that stores item into `localStorage`.
    pub store: Store,
    /// A reference cell to the weak pointer to the scheduler.
    ///
    /// TODO(benlee12): The Option could possibly be for deallocation.
    pub sched: RefCell<Option<Weak<Scheduler>>>,
    /// TODO(benlee12): Check if the string includes "#" or not.
    ///
    /// The fragment string of the URL
    pub active_route: String,
    /// The previous `active_route`. It is used to
    ///   determine whether the displayed list needs to be refreshed or not.
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
    pub fn call(&mut self, method_name: ControllerMessage) {
        // For ergonomics, remove reductant enum.
        use self::ControllerMessage::*;
        // Determining which ControllerMessage variant was passed.
        match method_name {
            AddItem(title) => self.add_item(title),
            SetPage(hash) => self.set_page(hash),
            // TODO(benlee12): Why do we need to move id and value?
            EditItemSave(id, value) => self.edit_item_save(id, value),
            // TODO(benlee12): Why do we need to move id?
            EditItemCancel(id) => self.edit_item_cancel(id),
            RemoveCompleted() => self.remove_completed_items(),
            // Note that we only need to take a string slice here, rather than
            // moving the entire String over. To remove an item, we just need
            // use the string as a key. But in the other methods, we actually
            // need to store the String.
            RemoveItem(id) => self.remove_item(&id),
            ToggleAll(completed) => self.toggle_all(completed),
            // TODO(benlee12): Why do we need to move id?
            ToggleItem(id, completed) => self.toggle_item(id, completed),
        }
    }

    /// Adds an `Item` to the `Store` with the title `title`.
    ///
    /// Signals the `View` to display it in the list.
    pub fn add_item(&mut self, title: String) {
        // Inserts item new Item to Store.
        self.store.insert(Item {
            // Uses the number of milliseconds elapsed since January 1, 1970
            // 00:00:00 UTC as an id.
            id: Date::now().to_string(),
            title,
            // Item starts off as active.
            completed: false,
        });
        // Tells View to clear the new todo input.
        self.add_message(ViewMessage::ClearNewTodo());
        // Refreshs the list.
        self._filter(true);
    }
    pub fn set_page(&mut self, hash: String) {}
    pub fn edit_item_save(&mut self, id: String, title: String) {}
    pub fn edit_item_cancel(&mut self, id: String) {}
    pub fn remove_completed_items(&mut self) {}
    pub fn remove_item(&mut self, id: &String) {}
    pub fn toggle_all(&mut self, completed: bool) {}
    pub fn toggle_item(&mut self, id: String, completed: bool) {}

    /// Forwards `view_message` to the Scheduler.
    pub fn add_message(&self, view_message: ViewMessage) {
        // self.sched = RefCell<Option<Weak<Scheduler>>>
        // Unwraps RefCell
        if let Ok(sched) = self.sched.try_borrow() {
            // sched = RefMut<Option<Weak<Scheduler>>
            // Unwraps Option
            if let Some(ref sched) = *sched {
                // sched = Weak<Scheduler>
                // Converts Weak to Rc
                if let Some(sched) = sched.upgrade() {
                    // sched = Rc<Scheduler>
                    //
                    // Yes! This increases the strong count, see this:
                    // https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=22f9c0ff181c70f2739f43b9350df0b2
                    //
                    // deref coercion -> Scheduler
                    sched.add_message(Message::View(view_message));
                }
            }
        }
    }
    /// Refresh the list based on the current route.
    pub fn _filter(&mut self, force: bool) {}
}

/// Messages that represent the methods to be called on the Controller
///
/// Note that each of these variants signal what needs to change to the
/// internal storage, not the actual representation that the users see.
///
/// As such, any description explained below that describes a visible change to
/// the user is not handled by the `Controller`, but instead, eventually by
/// the `View`. Whenever `View` needs to update, `Controller` sends a message
/// to the `Scheduler`, which will eventually forward that message to `View`.
pub enum ControllerMessage {
    /// Add a new todo item with the provided `String` as the title.
    AddItem(String),
    /// Set the `Controller`'s `active_route` to `String`.
    SetPage(String),
    EditItemSave(String, String),
    EditItemCancel(String),
    /// Remove all completed todo items from the todo list (in the storage).
    RemoveCompleted(),
    /// Remove an item with id `String` (in the storage).
    RemoveItem(String),
    /// If `bool` is true, mark all as completed. If `bool` is false, mark all
    /// as uncompleted. Signals the `View` to toggle the checkbox for marking
    /// all items as completed.
    ToggleAll(bool),
    /// Updates item with id `String` in storage based on `bool`.
    ///
    /// TODO(benlee12): Why is `bool` necessary when Store has it's own field?
    ToggleItem(String, bool),
}
