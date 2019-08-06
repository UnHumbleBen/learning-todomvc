pub use crate::controller::ControllerMessage;
pub use crate::element::Element;
pub use crate::{Message, Scheduler};
pub use std::cell::RefCell;
pub use std::rc::Rc;
// Brings JsCast trait in to scope so that unchecked_ref can be used.
pub use wasm_bindgen::JsCast;

// reexports crate::closure::Closure.
pub use wasm_bindgen::prelude::*;

/// Presentation layer.
///
/// TODO(benlee12): Why does example use #[wasm_bindgen]
pub struct View {
    /// shareable mutable container using references to a reference
    /// counted pointer to the Scheduler
    pub sched: RefCell<Rc<Scheduler>>,
    /// `<ul>` for the todo list.
    pub todo_list: Element,
    /// `<span>` that contains the counter.
    pub todo_item_counter: Element,
    /// `<button>` that clears completed.
    pub clear_completed: Element,
    /// `<section>` that contains the todo list.
    pub main: Element,
    /// `<label>` for the checkbox for marking all as complete.
    pub toggle_all: Element,
    /// `<input>` textbox that adds new todos.
    pub new_todo: Element,
    /// TODO(benlee12): figure out what `callbacks` does.
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
    pub fn init(&mut self) {
        // Assigns Window object to window if it exists, otherwise returns.
        let window = match web_sys::window() {
            Some(w) => w,
            None => return,
        };
        // Assigns document contained in window to document if it exists,
        // otherwise returns.
        let document = match window.document() {
            Some(d) => d,
            None => return,
        };
        //
        //
        // https://doc.rust-lang.org/src/core/cell.rs.html#1005-1013
        // impl<T: Clone> Clone for RefCell<T> {
        // # Panics
        //
        // Panics if the value is currently mutably borrowed.
        // #[inline]
        //     fn clone(&self) -> RefCell<T> {
        //         RefCell::new(self.borrow().clone())
        //     }
        // }
        //
        // Concretely, T = Rc<Scheduler>, self = &RefCell<Rc<Scheduler>
        //
        //
        // Evaluate: self.borrow()
        //
        // Note: '_ explictly marks elided lifetime so that there is is clear
        //       that there is a borrow in the return type.
        // pub fn borrow(&self) -> Ref<'_, T> {
        //     self.try_borrow().expect("already mutably borrowed")
        // }
        //
        // Concretely, T = Rc<Scheduler>
        //
        // Immutably borrows Rc<Scheduler>. The borrow lasts until the returned
        // `Ref<'_, Rc<Scheduler>>` exits scope. Multiple immutable borrows can
        // be taken out at the same time.
        //
        //
        // Evaulate: [self.borrow()].clone()
        //
        // pub fn clone(orig: &Ref<'b, T>) -> Ref<'b, T> {
        //     Ref {
        //         value: orig.value,
        //         borrow: orig.borrow.clone(),
        //     }
        // }
        //
        // Concretely, T = Rc<Scheduler>, `b = '_ previously = lifetime of
        // the T, the Rc<Scheduler>.
        //
        // Copies the Ref<'_, Rc<Scheduler>.
        //
        //
        // Without bogging into the details, the net result seems to be
        // sched to a clone of the RefCell, essentually acting as another
        // shared pointer to the Scheduler.
        let sched = self.sched.clone();
        // Creates a new instance of Closure from the provided boxed Rust
        // function. The closure provided meets these requirements:
        //
        // * Must implement Fn or FnMut
        // * Must be `static, so no stack references. Use move.
        //   TODO(benlee12): how does move solve this 'static problem.
        // * Must have at most 7 arguments.
        // * Argument and return value are types that can be shared with JS,
        //   indicated by #[wasm_bindgen] annotaitons or are simple numbers.
        //
        // pub fn wrap(data: Box<T>) -> Closure<T>
        // where
        //     F: Unsize<T> + 'static
        //
        // Concretely, T = closure_below: FnMut()
        // * Implements FnMut
        // TODO(benlee12): why FnMut()?
        // * 'static
        // TODO(benlee12): why is this satisfied?
        // * No arguments
        // * Argument = (), Return value = ()
        let set_page = Closure::wrap(Box::new(move || {
            // Get the 'Location' object, which contains information about the
            // URL of the document and provides methods for changing the URL
            // and loading another URL.
            if let Some(location) = document.location() {
                // Returns a String containing a '#' followed by the fragment
                // identifier of the URL. The String is not percent-decoded. If
                // the URL does not have a fragment identifier, returns an
                // empty string.
                if let Ok(hash) = location.hash() {
                    // Mutably borrows the wrapped Rc<Scheduler>.
                    if let Ok(sched) = &(sched.try_borrow_mut()) {
                        // Sends a message to the Scheduler to be sent to the
                        // Controller.
                        // TODO(benlee12): Find out what this message does.
                        sched.add_message(Message::Controller(ControllerMessage::SetPage(hash)));
                    }
                }
            }
            // Since closures are not Sized, they must be allocated on the
            // heap, as explained in this comment.
            // https://users.rust-lang.org/t/why-box-the-closure-in-closure/23436/2
        }) as Box<dyn FnMut()>);

        // Trait core::convert::Into
        //
        // https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#into.v
        //
        // A value-to-value conversion that consumes the input value.
        // Reciprocal of From.
        //   From<T> for U implies Into<U> for T
        // Both From and To are reflexive.
        //   From<T> for T and From<U> for U are implemented
        //
        // Implementing From automatically implements Into. However, sometimes,
        // it is not possible to implement From? Why is that? Let's say I want
        // to implement From for a type that I defined myself. I want to
        // add the functionality that this new type, let's call it Wrapper
        // can be converted to a Vec.
        //
        // Ideally, the code would look something like
        //
        // ```
        // let vector = Vec::from(wrapper)
        // ```
        //
        // So let's implement the From trait for the Vector. But wait! This
        // conflicts with the orphan rule, which prevents us from implementing
        // a trait if:
        //
        // * We do not own the trait.
        //
        // * We do not own the implementor.
        //
        // Since From and Vec are both defined in the standard library, we
        // cannot implement a From trait for Vec to convert to Wrapper.
        //
        // Maybe we can try Into instead:
        //
        // ```
        // let vector: Vec = wrapper.into()
        // ```
        //
        // This bypasses the orphan rule because even though we do not own the
        // Into trait, we do own the Wrapper class we defined!
        //
        // Let's example the usage below. EventTarget implements the From trait
        // to convert to a Window. In other words,
        //
        // ```
        // impl From<Window> for EventTarget
        // ```
        //
        // Via the blanket implementation for Into,
        //
        // ```
        // impl Into<EventTarget> for Window
        // ```
        //
        // EventTarget automatically implements the Into trait
        let window_et: web_sys::EventTarget = window.into();
        // Sets up `set_page` so that it will be called whenever a hashchange
        // event is delivered to the window.
        //
        // The hashchange event is fired when the fragment identifier of the
        // URL has changed (the part of the URL beginning with and following
        // the # symbol).
        //
        // TODO(benlee12): Trait object signature. What happened to Box?
        // Recall that set_page = Closure<dyn FnMut()>
        //
        // AsRef<T: ?Sized>
        // fn as_ref(&self) -> &T
        //
        // Concretely, Self = Closure
        // AsRef<T = JSValue>
        // fn as_ref(&Closure) -> &JsValue
        //
        // fn unchecked_ref<T>(&self) -> &T
        // where
        //     T: JsCast,
        //
        // Concretely, Self = JsValue, T = Function
        // fn unchecked_ref(&JsValue) -> &Function
        //
        // Use simple unwrap() for error handling.
        window_et
            .add_event_listener_with_callback("hashchange", set_page.as_ref().unchecked_ref())
            .unwrap();
        // Leaks `set_page` to ensure that it remains valid for the duration of
        // the entire program.
        //
        // If we did not do this then when `set_page` is dropped at the end of
        // `run`, the closures will raise an exception when called.
        // Reference: https://rustwasm.github.io/docs/wasm-bindgen/examples/closures.html?highlight=leak#srclibrs
        // TODO(benlee12): Why does `set_page` live till end of run()?
        //
        // Cycle collects this
        // self.callbacks.push((window_et, "hashchange".to_string(), set_page));
        // TODO(benlee12): Figure out what comment means.
        set_page.forget();

        // TODO(benlee12): Figure out what these do.
        self.bind_add_item();
        self.bind_edit_item_save();
        self.bind_edit_item_cancel();
        self.bind_remove_item();
        self.bind_toggle_item();
        self.bind_edit_item();
        self.bind_remove_completed();
        self.bind_toggle_all();
    }

    pub fn bind_add_item(&mut self) {}
    pub fn bind_edit_item_save(&mut self) {}
    pub fn bind_edit_item_cancel(&mut self) {}
    pub fn bind_remove_item(&mut self) {}
    pub fn bind_toggle_item(&mut self) {}
    pub fn bind_edit_item(&mut self) {}
    pub fn bind_remove_completed(&mut self) {}
    pub fn bind_toggle_all(&mut self) {}

    /// Used by `Scheduler` to convert a `ViewMessage` into a function call on
    /// a `View`.
    pub fn call(&mut self, method_name: ViewMessage) {}
}

pub enum ViewMessage {
    // TODO(benlee12): Why not just use a unit struct?
    ClearNewTodo(),
}
