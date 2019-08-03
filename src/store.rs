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
    pub fn fetch_local_storage(&mut self) -> Option<()> {
        // Initialize a mutable ItemList since it might be manipulated.
        let mut item_list = ItemList::new();

        // Passes `self.name` as a key for `localStorage` to retrieve the key's value.
        // This returns a Result<Option<String>>
        // If we have an get_item executes successfully then the Result is unwrapped.
        // If there exists a local storage to be fetched, then the Option is unwrapped.
        // value gets a String.
        if let Ok(Some(value)) = self.local_storage.get_item(&self.name) {}
        None
    }
}

pub struct ItemList {}

impl ItemList {
    pub fn new() -> ItemList {
        ItemList {}
    }
}
