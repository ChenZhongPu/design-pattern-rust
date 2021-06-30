use crate::{TV, RemoteControl};
use std::rc::Rc;
use std::cell::RefCell;

pub struct SpecialRemote {
    tv: Rc<RefCell<dyn TV>>,
}

impl RemoteControl for SpecialRemote {
    fn tv(&self) -> Rc<RefCell<dyn TV>> {
        Rc::clone(&self.tv)
    }
}

impl SpecialRemote {
    pub fn new(tv: Rc<RefCell<dyn TV>>) -> Self {
        SpecialRemote { tv }
    }
    pub fn up(&self) {
        self.set_channel(self.channel() + 1);
    }

    pub fn down(&self) {
        self.set_channel(self.channel() - 1);
    }
}