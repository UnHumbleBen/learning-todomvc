/// The JSON object contains methods for parsing JavaScript Object Notation
/// (JSON) and converting values to JSON.
pub use js_sys::JSON;
/// Stores items into `localStorage`.
///
/// # Fields
///
/// - `local_storage` is the `localStorage` which contains data stored across browser sessions.
/// - `data` contains a list of all the todo items.
/// - `name` is the value of key used to access `localStorage`.
pub struct Store {
    pub local_storage: web_sys::Storage,
    pub data: ItemList,
    pub name: String,
}

impl Store {
    /// Creates a new store with `name` as the local storage value name.
    /// Caches the `localStorage` for todo items if it exists.
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
    /// Returns an `Option<()>` to enable handling JS exceptions with `?`.
    /// Caches the store into `self.data` to reduce calls to JS.
    ///
    /// Uses `&mut self` to borrow mutably since the `data` field of `Store`
    /// may be modified to update to the new ItemList.
    ///
    /// # Procedure
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
}

pub struct ItemList {}

impl ItemList {
    pub fn new() -> ItemList {
        ItemList {}
    }

    pub fn push(&mut self, item: Item) {}
}

/// Represents a todo item.
///
/// # Fields
///
/// - `title` is the name of the todo.
/// - `completed` is `true` if the todo is completed, `false otherwise.
/// - `id` is an unique id to identify this todo.
pub struct Item {
    pub title: String,
    pub completed: bool,
    pub id: String,
}
