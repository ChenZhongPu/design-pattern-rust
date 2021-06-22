use crate::{Command, NoCommand};
use std::rc::Rc;
use std::cell::RefCell;

pub struct RemoteControl {
    on_commands: Vec<Rc<RefCell<dyn Command>>>,
    off_commands: Vec<Rc<RefCell<dyn Command>>>,
    undo_command: Rc<RefCell<dyn Command>>,
}

impl RemoteControl {
    pub fn new() -> Self {
        RemoteControl {
            on_commands: vec![Rc::new(RefCell::new(NoCommand)); 4],
            off_commands: vec![Rc::new(RefCell::new(NoCommand)); 4],
            undo_command: Rc::new(RefCell::new(NoCommand)),
        }
    }

    pub fn set_command(&mut self, slot: usize, on_command: Rc<RefCell<dyn Command>>, off_command: Rc<RefCell<dyn Command>>) {
        self.on_commands[slot] = on_command;
        self.off_commands[slot] = off_command;
    }

    pub fn on_button_pushed(&mut self, slot: usize) {
        self.on_commands[slot].borrow_mut().execute();
        self.undo_command = Rc::clone(&self.on_commands[slot]);
    }

    pub fn off_button_pushed(&mut self, slot: usize) {
        self.off_commands[slot].borrow_mut().execute();
        self.undo_command = Rc::clone(&self.off_commands[slot]);
    }

    pub fn undo_button_pushed(&mut self) {
        self.undo_command.borrow_mut().undo();
    }
}