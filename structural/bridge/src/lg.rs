use crate::TV;

pub struct LG {
    channel: i32,
}
impl LG {
    pub fn new() -> Self {
        LG { channel: 1 }
    }
}
impl TV for LG {
    fn on(&self) {
        println!("Turning on the LG TV");
    }

    fn off(&self) {
        println!("Turning off the LG TV");
    }

    fn turn_channel(&mut self, channel: i32) {
        self.channel = channel;
        println!("Set the LG TV Channel to {}", channel);
    }

    fn channel(&self) -> i32 {
        self.channel
    }
}