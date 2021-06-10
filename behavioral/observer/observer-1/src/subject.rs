use crate::Observer;

pub struct WeatherData<'a> {
    observers: Vec<&'a dyn Observer>,
    tmp: f64,
}

impl<'a> WeatherData<'a> {
    pub fn new() -> Self {
        WeatherData {
            observers: vec![],
            tmp: f64::NAN,
        }
    }
}

impl<'a> WeatherData<'a> {
    pub fn add_observer(&mut self, observer: &'a dyn Observer) {
        self.observers.push(observer);
    }
    pub fn remove_observer(&mut self, observer: &'a dyn Observer) {
        self.observers.remove(
            self.observers
                .iter()
                .position(|x| x.get_id() == observer.get_id())
                .expect("observer not found"),
        );
    }
    pub fn notify_observers(&self) {
        for observer in self.observers.iter() {
            observer.update(self.tmp);
        }
    }
    pub fn set_measurement(&mut self, tmp: f64) {
        self.tmp = tmp;
        self.notify_observers();
    }
}
