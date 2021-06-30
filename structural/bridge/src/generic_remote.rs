use crate::{TV, RemoteControl};
use std::rc::Rc;
use std::cell::RefCell;

pub struct GenericRemote {
    tv: Rc<RefCell<dyn TV>>,
}

impl RemoteControl for GenericRemote {
    fn tv(&self) -> Rc<RefCell<dyn TV>> {
        Rc::clone(&self.tv)
    }
}

impl GenericRemote {
    pub fn new(tv: Rc<RefCell<dyn TV>>) -> Self {
        GenericRemote { tv }
    }

    pub fn next_channel(&self) {
        self.set_channel(self.channel() + 1);
    }

    pub fn prev_channel(&self) {
        self.set_channel(self.channel() - 1);
    }
}