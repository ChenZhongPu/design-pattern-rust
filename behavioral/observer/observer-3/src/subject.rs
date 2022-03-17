use std::rc::{Weak, Rc};

use crate::Observer;

pub struct WeatherData {
    observers: Vec<Weak<dyn Observer>>,
    tmp: f64,
}

impl WeatherData {
    pub fn new() -> Self {
        WeatherData { observers: vec![], tmp: f64::NAN }
    }

    pub fn add_observer(&mut self, observer: &Rc<dyn Observer>) {
        self.observers.push(Rc::downgrade(observer));
    }

    pub fn remove_observer(&mut self, observer: &Rc<dyn Observer>) {
        if let Some(p) = self.observers.iter().position(|x| match x.upgrade() {
            Some(x_ptr) => Rc::ptr_eq(&x_ptr, observer),
            None => false,
        }) {
            self.observers.swap_remove(p);
        }
    }

    pub fn notify_observers(&self) {
        for observer in &self.observers {
            if let Some(x_ptr) = observer.upgrade() {
                x_ptr.update(self.tmp);
            }
        }
    }

    pub fn set_measurement(&mut self, tmp: f64) {
        self.tmp = tmp;
        self.notify_observers();
    }
}
