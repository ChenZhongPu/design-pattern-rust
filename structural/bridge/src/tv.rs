pub trait TV {
    fn on(&self);
    fn off(&self);
    fn turn_channel(&mut self, channel: i32);
    fn channel(&self) -> i32;
}