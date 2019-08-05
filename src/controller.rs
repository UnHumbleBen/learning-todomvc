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
/// - TODO(benlee12): Check if the string includes "#" or not.
/// - `active_route` is the fragment string of the URL
/// - `last_active_route` is the previous `active_route`. It is used to
///   determine whether the displayed list needs to be refreshed or not.
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
    pub fn call(&mut self, method_name: ControllerMessage) {
        // For ergonomics, remove reductant enum.
        use self::ControllerMessage::*;
        // Determining which ControllerMessage variant was passed.
        match method_name {
            AddItem(title) => self.add_item(title),
            SetPage(hash) => self.set_page(hash),
            EditItemSave(id, value) => self.edit_item_save(id, value),
            EditItemCancel(id) => self.edit_item_cancel(id),
            RemoveCompleted() => self.remove_completed_items(),
            RemoveItem(id) => self.remove_item(&id),
            ToggleAll(completed) => self.toggle_all(completed),
            ToggleItem(id, completed) => self.toggle_item(id, completed),
        }
    }

    pub fn add_item(&mut self, title: String) {}
    pub fn set_page(&mut self, hash: String) {}
    pub fn edit_item_save(&mut self, id: String, title: String) {}
    pub fn edit_item_cancel(&mut self, id: String) {}
    pub fn remove_completed_items(&mut self) {}
    pub fn remove_item(&mut self, id: String) {}
    pub fn toggle_all(&mut self, completed: bool) {}

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
