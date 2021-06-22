use crate::Command;
use crate::NoCommand;

pub struct SimpleRemoteControl {
    slot: Box<dyn Command>,
}

impl SimpleRemoteControl {
    pub fn new() -> Self {
        SimpleRemoteControl { slot: Box::new(NoCommand) }
    }

    pub fn set_command(&mut self, slot: Box<dyn Command>) {
        self.slot = slot;
    }

    pub fn button_pressed(&self) {
        self.slot.execute();
    }
}