use crate::{Hottub, Light};
use std::cell::{RefCell};
use std::rc::Rc;

pub trait Command {
    fn execute(&self);
}

pub struct NoCommand;
impl Command for NoCommand {
    fn execute(&self) {}
}

pub struct LightOnCommand {
    light: Rc<RefCell<Light>>,
}
impl LightOnCommand {
    pub fn new(light: Rc<RefCell<Light>>) -> Self {
        LightOnCommand { light }
    }
}
impl Command for LightOnCommand {
    fn execute(&self) {
        self.light.borrow().on();
    }
}

pub struct LightOffCommand {
    light: Rc<RefCell<Light>>,
}
impl LightOffCommand {
    pub fn new(light: Rc<RefCell<Light>>) -> Self {
        LightOffCommand { light }
    }
}
impl Command for LightOffCommand {
    fn execute(&self) {
       self.light.borrow().off();
    }
}

pub struct HottubOnCommand {
    hottub: Rc<RefCell<Hottub>>,
}

impl HottubOnCommand {
    pub fn new(hottub: Rc<RefCell<Hottub>>) -> Self {
       HottubOnCommand { hottub }
    }
}

impl Command for HottubOnCommand {
    fn execute(&self) {
        self.hottub.borrow_mut().on();
        self.hottub.borrow_mut().heat();
        self.hottub.borrow().bubbles_on();
    }
}

pub struct HottubOffCommand {
    hottub: Rc<RefCell<Hottub>>,
}

impl HottubOffCommand {
    pub fn new(hottub: Rc<RefCell<Hottub>>) -> Self {
        HottubOffCommand { hottub }
    }
}

impl Command for HottubOffCommand {
    fn execute(&self) {
        self.hottub.borrow_mut().cool();
        self.hottub.borrow_mut().off();
    }
}