pub use crate::element::Element;
pub use crate::scheduler::Scheduler;
pub use std::cell::RefCell;
pub use std::rc::Rc;

// reexports crate::closure::Closure.
pub use wasm_bindgen::prelude::*;

/// Presentation layer.
///
/// # Fields
///
/// - `sched` is a shareable mutable container using references to a reference
///   counted pointer to a Scheduler
/// - Wrappers of DOM elements
///   - `todo_list` is the ul for the todo list.
///   - `todo_item_counter` is the span that contains the counter.
///   - `clear_completed` is the button that clears completed.
///   - `main` is the section that contains the todo list.
///   - `toggle_all` is the label for the checkbox for marking all as complete.
///   - `new_todo` is the input textbox that adds new todos.
/// - `callbacks`
/// - TODO(benlee12): figure out what `callbacks` does.
pub struct View {
    pub sched: RefCell<Rc<Scheduler>>,
    pub todo_list: Element,
    pub todo_item_counter: Element,
    pub clear_completed: Element,
    pub main: Element,
    pub toggle_all: Element,
    pub new_todo: Element,
    pub callbacks: Vec<(web_sys::EventTarget, String, Closure<dyn FnMut()>)>,
}

impl View {
    /// Creates a `View` struct that takes in a referenced shared pointer
    /// to the Scheduler `sched`.
    pub fn new(sched: Rc<Scheduler>) -> Option<View> {
        // Selects the ul for the todo list.
        let todo_list = Element::qs(".todo-list")?;
        // Selects the span that writes the number of todo items.
        let todo_item_counter = Element::qs(".todo-count")?;
        // Selects the button that clears completed todos.
        let clear_completed = Element::qs(".clear-completed")?;
        // Selects the section that contains the list of todos.
        let main = Element::qs(".main")?;
        // Selects the label for the checkbox which marks all as complete.
        let toggle_all = Element::qs(".toggle-all")?;
        // Selects the input that adds new todos.
        let new_todo = Element::qs(".new-todo")?;
        Some(View {
            sched: RefCell::new(sched),
            todo_list,
            todo_item_counter,
            clear_completed,
            main,
            toggle_all,
            new_todo,
            callbacks: Vec::new(),
        })
    }
    pub fn init(&mut self) {}
}

pub enum ViewMessage {}
