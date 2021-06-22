pub struct Light {
    location: String,
    level: i32,
}
impl Light {
    pub fn new(location: &str) -> Self {
        Light {
            location: String::from(location),
            level: 0,
        }
    }

    pub fn on(&mut self) {
        self.level = 100;
        println!("{} light is on", self.location);
    }

    pub fn off(&mut self) {
        self.level = 0;
        println!("{} light is off", self.location);
    }

    pub fn dim(&mut self, level: i32) {
        self.level = level;
        if level == 0 {
            self.off();
        } else {
            println!("The {} light is dimmed to {}%", self.location, level);
        }
    }

    pub fn get_level(&self) -> i32 {
       return self.level;
    }
}