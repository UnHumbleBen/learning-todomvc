pub use crate::scheduler::Scheduler;
pub use std::rc::Rc;

pub struct View {}

impl View {
    pub fn new(sched: Rc<Scheduler>) -> Option<View> {
        None
    }
    pub fn init(&mut self) {}
}

pub enum ViewMessage {

}
