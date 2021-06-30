use crate::TV;
use std::rc::Rc;
use std::cell::RefCell;

pub trait RemoteControl {
    fn tv(&self) -> Rc<RefCell<dyn TV>>;
    fn on(&self) {
        self.tv().borrow().on();
    }
    fn off(&self) {
        self.tv().borrow().off();
    }
    fn set_channel(&self, channel: i32) {
        self.tv().borrow_mut().turn_channel(channel);
    }
    fn channel(&self) -> i32 {
        self.tv().borrow().channel()
    }
}