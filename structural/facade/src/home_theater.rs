use crate::{Amplifier, Tuner};
use std::cell::RefCell;
use std::rc::Rc;

pub struct HomeTheaterFacade {
    amp: RefCell<Amplifier>,
    tuner: Rc<RefCell<Tuner>>,
}

impl HomeTheaterFacade {
    pub fn new(amp: Amplifier, tuner: Tuner)-> Self {
       HomeTheaterFacade { amp: RefCell::new(amp), tuner: Rc::new(RefCell::new(tuner)) }
    }

    pub fn listen_radio(&self, frequency: f64) {
        println!("Tuning in the airwaves...");
        self.tuner.borrow().on();
        self.tuner.borrow_mut().set_frequency(frequency);
        self.amp.borrow().on();
        self.amp.borrow_mut().set_volume(5);
        self.amp.borrow_mut().set_tuner(Rc::clone(&self.tuner));
    }

    pub fn stop_radio(&self) {
        println!("Shutting down the radio...");
        self.tuner.borrow().off();
        self.amp.borrow().off();
    }

}