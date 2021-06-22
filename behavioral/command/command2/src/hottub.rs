#[derive(Default)]
pub struct Hottub {
    is_on: bool,
    temperature: i32,
}

impl Hottub {
    pub fn new() -> Self {
        Hottub::default()
    }

    pub fn on(&mut self) {
        self.is_on = true;
    }

    pub fn off(&mut self) {
        self.is_on = false;
    }

    pub fn bubbles_on(&self) {
        if self.is_on {
            println!("Hottub is bubbling!")
        }
    }

    pub fn bubbles_off(&self) {
        if self.is_on {
            println!("Hottub is not bubbling!")
        }
    }

    pub fn set_temperature(&mut self, temp: i32) {
        self.temperature = temp;
    }

    pub fn heat(&mut self) {
        self.temperature = 65;
        println!("Hottub is heating to a steaming 105 degrees");
    }

    pub fn cool(&mut self) {
        self.temperature = 98;
        println!("Hottub is cooling to 98 degree");
    }

}