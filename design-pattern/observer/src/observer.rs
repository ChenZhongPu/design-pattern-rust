use std::fmt;

pub trait Observer {
    fn update(&mut self, temp: f64, humidity: f64, pressure: f64);
}

pub struct CurrentConditionDisplay {
    id: u32,
    temperature: f64,
    humidity: f64,
}

impl CurrentConditionDisplay {
    pub fn new(id: u32) -> Self {
        CurrentConditionDisplay {
            id, temperature: f64::NAN, humidity: f64::NAN,
        }
    }
}

impl Observer for CurrentConditionDisplay {
    fn update(&mut self, temp: f64, humidity: f64, _: f64) {
        self.temperature = temp;
        self.humidity = humidity;
        println!("{}", self);
    }
}

impl fmt::Display for CurrentConditionDisplay {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CurrentConditionDisplay({}): temperature={}, humidity={}", self.id, self.temperature, self.humidity)
    }
}

pub struct PredictDisplay {
    id: u32,
    predict_temperature: f64,
}

impl PredictDisplay {
    pub fn new(id: u32) -> Self {
        PredictDisplay {id, predict_temperature: f64::NAN,}
    }
}

impl Observer for PredictDisplay {
    fn update(&mut self, temp: f64, _: f64, _: f64) {
        self.predict_temperature = temp + 2.0;
        println!("{}", self);
    }
}

impl fmt::Display for PredictDisplay {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PredictDisplay({}): predicted temperature={}", self.id, self.predict_temperature)
    }
}