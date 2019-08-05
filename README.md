# TODO MVC

Implementing TODO MVC in Rust using wasm-bindgen and web-sys
to learn how MVC works in this language.

Progress:
- [ ] `run`
  - [ ] `app`
    - [x] `scheduler::Scheduler::new`
    - [x] `store::Store::new`
    - [x] `controller::Controller::new`
    - [x] `view::View::new`
        - [x]`element::Element::qs`
    - [ ] `view::View::init`
        - [ ] `scheduler::Scheduler::add_message`
            - [ ] `scheduler::Scheduler::run`
                - [ ] `scheduler::Scheduler::next_message`
                    - [ ] `controller::Controller::call`
                        - [ ] `controller::Controller::[all message handlers]`
                    - [ ] `view::View::call`
        - [ ] `view::View::bind_[*]`
    - [ ] `scheduler::Scheduler::set_view`
    - [ ] `scheduler::Scheduler::set_controller`

Example comes from [The `wasm-bindgen` Guide](https://rustwasm.github.io/docs/wasm-bindgen/examples/todomvc.html)