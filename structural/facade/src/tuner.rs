pub struct Tuner {
    description: String,
    frequency: f64,
    is_am: bool,
}

impl Tuner {
    pub fn new(description: &str) -> Self {
        Tuner {
            description: String::from(description),
            frequency: 0.0,
            is_am: false,
        }
    }

    pub fn on(&self) {
        println!("{} on", self.description);
    }

    pub fn off(&self) {
        println!("{} off", self.description);
    }

    pub fn set_frequency(&mut self, frequency: f64) {
        self.frequency = frequency;
    }

    pub fn set_am(&mut self) {
       self.is_am = true;
       println!("{} setting AM", self.description);
    }

    pub fn set_fm(&mut self) {
        self.is_am = false;
        println!("{} setting PM", self.description);
    }
}