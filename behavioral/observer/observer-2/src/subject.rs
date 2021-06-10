use crate::Observer;

use std::cell::RefCell;
use std::rc::Rc;

pub struct WeatherData {
    observers: Vec<Rc<RefCell<dyn Observer>>>,
    tmp: f64,
}

impl WeatherData {
    pub fn new() -> Self {
        WeatherData {
            observers: vec![],
            tmp: f64::NAN,
        }
    }
}

impl WeatherData {
    pub fn add_observer(&mut self, observer: Rc<RefCell<dyn Observer>>) {
        self.observers.push(observer);
    }
    pub fn remove_observer(&mut self, observer: Rc<RefCell<dyn Observer>>) {
        self.observers.remove(
            self.observers
                .iter()
                .position(|x| x.borrow().get_id() == observer.borrow().get_id())
                .expect("observer not found"),
        );
    }
    pub fn notify_observers(&self) {
        for observer in self.observers.iter() {
            observer.borrow_mut().update(self.tmp);
        }
    }
    pub fn set_measurement(&mut self, tmp: f64) {
        self.tmp = tmp;
        self.notify_observers();
    }
}
