use crate::tuner::Tuner;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Amplifier{
    description: String,
    tuner: Option<Rc<RefCell<Tuner>>>,
    level: i32,
}

impl Amplifier {
    pub fn new(description: &str) -> Self {
       Amplifier {
           description: String::from(description),
           tuner: None,
           level: 0,
       }
    }

    pub fn on(&self) {
        println!("{} on", self.description);
    }

    pub fn off(&self) {
        println!("{} off", self.description);
    }

    pub fn set_volume(&mut self, level: i32) {
        self.level = level;
        println!("{} setting volume to {}", self.description, level);
    }

    pub fn set_tuner(&mut self, tuner: Rc<RefCell<Tuner>>) {
        self.tuner = Some(tuner);
    }
}