# TODO MVC

Implementing TODO MVC in Rust using wasm-bindgen and web-sys
to learn how MVC works in this language.

Progress:
- [ ] `view::View::init`
    - [ ] `scheduler::Scheduler::add_message`
        - [ ] `scheduler::Scheduler::run`
            - [ ] `scheduler::Scheduler::next_message`
                - [ ] `controller::Controller::call`
                    - [ ] `controller::Controller::[all message handlers]`
                - [ ] `view::View::call`
    - [ ] `view::View::bind_[*]`

Example comes from [The `wasm-bindgen` Guide](https://rustwasm.github.io/docs/wasm-bindgen/examples/todomvc.html)