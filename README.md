# TODO MVC

Implementing TODO MVC in Rust using wasm-bindgen and web-sys
to learn how MVC works in this language.

# Progress:
- [x] `run`
  - [x] `app`
    - [x] `scheduler::Scheduler::new`
    - [x] `store::Store::new`
    - [x] `controller::Controller::new`
    - [x] `view::View::new`
        - [x] `element::Element::qs`
    - [x] `view::View::init`
        - [x] `scheduler::Scheduler::add_message`
            - [x] `scheduler::Scheduler::run`
                - [x] `scheduler::Scheduler::next_message`
                    - [x] `controller::Controller::call`
                        - [ ] `controller::Controller::[all message handlers]`
                            - [x] `controller::Controller::add_message`
                            - [x] `controller::Controller::_filter`
                                - [ ] `store::ItemList::find`
                                - [ ] `store::(ItemListSlice as Into<ItemList>)::into`
                            - [x] `store::Store::insert`
                                - [x] `store::(ItemList as ItemListTrait)::push`
                                - [x] `store::Store::sync_local_storage`
                    - [ ] `view::View::call`
        - [ ] `view::View::bind_[*]`
    - [ ] `scheduler::Scheduler::set_view`
        - [ ] dependent functions
    - [ ] `scheduler::Scheduler::set_controller`
        - [ ] dependent functions

Note that checked means that the function has been implemented, but it will not
work until all of the functions it uses are also implemented.

Example comes from [The `wasm-bindgen` Guide](https://rustwasm.github.io/docs/wasm-bindgen/examples/todomvc.html)