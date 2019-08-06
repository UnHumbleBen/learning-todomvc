//! Accessors for `localStorage`
//!
//! Uses the following JavaScript format to store the todo items list.
//!
//! ```
//! [
//!      todo_item_1,
//!      todo_item_2,
//!      // --snip--
//! ]
//! ```
//!
//! Each todo_item is stored as follows,
//!
//! ```
//! [
//!      todo_item.title,
//!      todo_item.completed,
//!      todo_item.id,
//! ]
//! ```
//!
//! where `title` is a String containing the task, `completed` is a bool
//! indicating task completion, and `id` is a `String` identifier for the task.
/// The JSON object contains methods for parsing JavaScript Object Notation
/// (JSON) and converting values to JSON.
pub use js_sys::JSON;
/// Imports JsValue
pub use wasm_bindgen::prelude::*;
/// Stores items into `localStorage`.
pub struct Store {
    /// `localStorage` which contains data stored across browser sessions.
    pub local_storage: web_sys::Storage,
    /// Contains a list of all the todo items.
    pub data: ItemList,
    /// The value of key used to access `localStorage`.
    pub name: String,
}
impl Store {
    /// Creates a new store with `name` as the local storage value name.
    /// Caches the `localStorage` for todo items if it exists.
    ///
    /// # Implementation Details
    ///
    /// Uses `Option<Store>` as the return type to enable handling of JS exceptions with `?`,
    /// naturally propagting it upwards to the wasm boundary.
    pub fn new(name: &str) -> Option<Store> {
        // Gets the `Window` object.
        let window = web_sys::window()?;
        // local_storage() gets the `localStorage`, wrapped in a Result and Option.
        // Through pattern matching, local_storage gets a `Storage` object.
        if let Ok(Some(local_storage)) = window.local_storage() {
            // Initializes the `Store` struct with a new localStorage, empty data, and given name.
            let mut store = Store {
                local_storage,
                data: ItemList::new(),
                name: String::from(name),
            };
            // Initializes the `data` field with that found from `localStorage`, if it exists.
            store.fetch_local_storage();
            // Return the newly created `store`.
            Some(store)
        } else {
            // Something must have went wrong with accessing `localStorage`, so return None.
            None
        }
    }

    /// Reads the local `ItemList` from `localStorage`.
    ///
    /// # Implementation Details
    ///
    /// Returns an `Option<()>` to enable handling JS exceptions with `?`.
    /// Caches the store into `self.data` to reduce calls to JS.
    ///
    /// Uses `&mut self` to borrow mutably since the `data` field of `Store`
    /// may be modified to update to the new ItemList.
    ///
    /// ## Procedure
    ///
    /// 1. Query the `localStorage` for the list of todo items.
    /// 2. Iterate through the list of todo items, copying each one to a cache.
    /// 3. Assigns that cache to the `data` field of `Store`.
    pub fn fetch_local_storage(&mut self) -> Option<()> {
        // Initialize a mutable ItemList since it might be manipulated.
        let mut item_list = ItemList::new();

        // Passes `self.name` as a key for `localStorage` to retrieve the key's value.
        // This returns a Result<Option<String>>
        // If we have an get_item executes successfully then the Result is unwrapped.
        // If there exists a local storage to be fetched, then the Option is unwrapped.
        // value gets a String.
        if let Ok(Some(value)) = self.local_storage.get_item(&self.name) {
            // `JSON.parse()` parses a `value`, constructing the `JsValue`
            // described by the string as a Result<JsValue, JsValue>.
            // ok() converts it to Option<JsValue> to allow handling any errors
            // with `?`. Assigns the JsValue to data.
            let data = JSON::parse(&value).ok()?;
            // try_iter() creates an iterator over `data` using JS iteration
            // protocol and `Symbol.iterator` wrapped in Result and Option.
            //
            // # Links
            //
            // - JS iteration protocol
            //   https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Iteration_protocols
            // - Symbol.iterator
            //   https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Symbol/iterator
            //
            // ok() converts the Result layer to an Option, which can be
            // unwrapped with `??`, leaving iter with a `IntoIter`.
            //
            // `IntoIter` is an rust iterator over the JS Symbol.iterator
            // iteration protocol.
            let iter = js_sys::try_iter(&data).ok()??;
            // iteration protocol. The Rust iterator will yield items of type
            // Result<JsValue>. If it yields an Ok(...), then the JS iterator
            // protocol returned an element. If it yields an Err(...), then
            // the JS iterator protocol threw an exception.
            for item in iter {
                // Unwraps the todo item.
                //
                // Converts item from Result<JsValue> --> Option<JsValue>.
                // from call to ok() then --> JsValue
                // if JS iterator protocol returned an element.
                let item = item.ok()?;
                // Assigns a casted pointer to item_array.
                //
                // ```
                // fn dyn_ref<T>(&self) -> Option<&T>
                // where
                //     T: JsCast,
                // ```
                //
                // Concretely, T = js_sys::Array
                //
                // Performs a dynamic cast (checked at runtime) of `item` into an Array.
                //
                // If `item` cannot be casted to Array, then the method returns None.
                // Otherwise it returns Some(&Array), which is unwrapped with ?.
                // TODO(benlee12): why use dyn_ref instead of dyn_into?
                let item_array: &js_sys::Array = wasm_bindgen::JsCast::dyn_ref(&item)?;
                // Extracts title field from item_array and assign it to title.
                //
                // pub fn shift(&Array) -> JsValue
                // Removes the first todo item from the data and returns that todo item.
                // Why does this not require `&mut Array`? Interior Mutability.
                // See https://github.com/rustwasm/wasm-bindgen/issues/1061#issuecomment-442937471.
                //
                // pub fn as_string(&self) -> Option<String>
                // Copies the JS string value into wasm linear memory, encoded
                // as UTF-8 and returns it as a Rust String.
                let title = item_array.shift().as_string()?;
                // Extracts the completed field from item_array and assigns it
                // to `completed` as a bool.
                let completed = item_array.shift().as_bool()?;
                // Extracts the id field from item_array and assigns it to `id`
                // as a String.
                let id = item_array.shift().as_string()?;
                // Creates a `Item` struct using the extracted fields.
                let temp_item = Item {
                    title,
                    completed,
                    id,
                };
                // Adds the newly created `Item` struct to `ItemList`.
                item_list.push(temp_item);
            }
        }
        // Assigns the data field of `Store` to the fetched `item_list`.
        self.data = item_list;
        // Returns this dummy value so satisfy type requirement, which had
        // benefit of easy handling with `?`.
        Some(())
    }

    /// Insert an item into the Store.
    ///
    /// `Item` item is the Item to insert
    pub fn insert(&mut self, item: Item) {
        self.data.push(item);
        self.sync_local_storage();
    }
    pub fn find(&mut self, query: ItemQuery) -> Option<ItemListSlice<'_>> {
        Some(ItemListSlice { list: vec![] })
    }

    /// Writes the local `ItemList` to `localStorage`.
    pub fn sync_local_storage(&mut self) {
        // Creates an empty JS Array to serialize the list of todo items.
        let array = js_sys::Array::new();
        // Serialize each item into child, then push each child into array.
        for item in self.data.iter() {
            // Create an empty JS Array for serializing a single todo item.
            let child = js_sys::Array::new();
            // TODO(benlee12): Why do we need to clone?
            // TODO(benlee12): Why does push borrow?
            child.push(&JsValue::from(&item.title));

            // The way based on the example
            // let s = item.title.clone();
            // child.push(&JsValue::from(&s));

            child.push(&JsValue::from(item.completed));
            child.push(&JsValue::from(&item.id));

            array.push(&JsValue::from(child));
        }
        // Converts `array` into a JSON formatted JsString.
        if let Ok(storage_string) = JSON::stringify(&JsValue::from(array)) {
            // TODO(benlee12): Remove unnecessary .to_string() and
            // add this .set_item(&self.name, &storage_string) below
            let storage_string: String = storage_string.into();
            self.local_storage
                // Passes `name` as key and storage string as
                .set_item(&self.name, &storage_string)
                // Simple error handling.
                .unwrap();
        }
    }
}
/// A trait for a list of items of type `T`.
///
/// TODO(benlee12): why use a trait?
///
/// TODO(benlee12): why do public traits must have public functions?
pub trait ItemListTrait<T> {
    /// Initializes an empty list of items.
    fn new() -> Self;
    /// Appends `item` to the back of the list of items.
    fn push(&mut self, item: T);
    /// Returns an iterator over the slice.
    fn iter(&self) -> std::slice::Iter<'_, T>;
}

pub struct ItemList {
    list: Vec<Item>,
}

impl ItemListTrait<Item> for ItemList {
    fn new() -> ItemList {
        // Initializes the struct with an empty vector in the `list` field.
        ItemList { list: Vec::new() }
    }

    fn push(&mut self, item: Item) {
        // Appends `item` to the back of the `list`.
        self.list.push(item);
    }

    fn iter(&self) -> std::slice::Iter<'_, Item> {
        // Returns an iterator over the vector slice.
        self.list.iter()
    }
}

/// Represents a todo item.
pub struct Item {
    /// The name of the todo.
    pub title: String,
    /// `true` if the todo is completed, `false` otherwise.
    pub completed: bool,
    /// A unique id to identify this todo.
    pub id: String,
}

/// Represents a search into the store.
pub enum ItemQuery {
    Completed { completed: bool },
    EmptyItemQuery,
}

pub struct ItemListSlice<'a> {
    list: Vec<&'a Item>,
}
