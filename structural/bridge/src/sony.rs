use crate::TV;

pub struct Sony {
    station: i32,
}

impl Sony {
    pub fn new() -> Self {
        Sony { station: 0 }
    }
}
impl TV for Sony {

    fn on(&self) {
        println!("Turning on the Sony TV");
    }

    fn off(&self) {
        println!("Turning off the Sony TV");
    }

    fn turn_channel(&mut self, channel: i32) {
        self.station = channel;
        println!("Set the Sony TV station to {}", channel);
    }

    fn channel(&self) -> i32 {
        self.station
    }
}