pub use std::rc::Weak;
pub use crate::{Scheduler, Store};

pub struct Controller {

}

impl Controller {
    pub fn new(store: Store, sched: Weak<Scheduler>) -> Controller {
        Controller { }
    }
}

pub enum ControllerMessage {
    SetPage(String),
}