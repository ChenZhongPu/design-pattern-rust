use crate::{Command, NoCommand};

pub struct RemoteControl {
    on_commands: Vec<Box<dyn Command>>,
    off_commands: Vec<Box<dyn Command>>,
}

impl RemoteControl {
    pub fn new() -> Self {
        RemoteControl {
            on_commands: vec![Box::new(NoCommand), Box::new(NoCommand), Box::new(NoCommand), Box::new(NoCommand)],
            off_commands: vec![Box::new(NoCommand), Box::new(NoCommand), Box::new(NoCommand), Box::new(NoCommand)],
        }
    }

    pub fn set_command(&mut self, slot: usize, on_command: Box<dyn Command>, off_command: Box<dyn Command>) {
        self.on_commands[slot] = on_command;
        self.off_commands[slot] = off_command;
    }

    pub fn on_button_pushed(&self, slot: usize)  {
        self.on_commands[slot].execute();
    }

    pub fn off_button_pushed(&self, slot: usize) {
        self.off_commands[slot].execute();
    }
}

