use crate::observer::Observer;

pub trait Subject {
    fn register_observer(&mut self, observer: Box<dyn Observer>);
    fn notify_observers(&mut self);
    // todo: remove_observer
}

pub struct WeatherData {
    observers: Vec<Box<dyn Observer>>,
    temperature: f64,
    humidity: f64,
    pressure: f64,
}

impl WeatherData {
    pub fn new() -> Self {
        WeatherData {
            observers: vec![],
            temperature: f64::NAN,
            humidity: f64::NAN,
            pressure: f64::NAN,
        }
    }
}

impl Subject for WeatherData {
    fn register_observer(&mut self, observer: Box<dyn Observer>) {
        self.observers.push(observer);
    }
    fn notify_observers(&mut self) {
        for observer in self.observers.iter_mut() {
            observer.update(self.temperature, self.humidity, self.pressure);
        }
    }

}

impl WeatherData {
    pub fn measurements_changed(&mut self) {
        self.notify_observers();
    }

    pub fn set_measurements(&mut self, temperature: f64,
                            humidity: f64, pressure: f64) {
        self.temperature = temperature;
        self.humidity = humidity;
        self.pressure = pressure;
        self.measurements_changed();
    }
}
