use crate::Light;
use std::cell::RefCell;
use std::rc::Rc;

pub trait Command {
    fn execute(&mut self);
    fn undo(&mut self);
}

pub struct NoCommand;
impl Command for NoCommand {
    fn execute(&mut self) {
    }
    fn undo(&mut self) {
    }
}

pub struct LightOnCommand {
    light: Rc<RefCell<Light>>,
    level: i32,
}

impl LightOnCommand {
    pub fn new(light: Rc<RefCell<Light>>) -> Self {
        LightOnCommand { light, level: 0 }
    }
}
impl Command for LightOnCommand {
    fn execute(&mut self) {
        self.level = self.light.borrow().get_level();
        self.light.borrow_mut().on();
    }

    fn undo(&mut self) {
        self.light.borrow_mut().dim(self.level);
    }
}

pub struct LightOffCommand {
    light: Rc<RefCell<Light>>,
    level: i32,
}

impl LightOffCommand {
    pub fn new(light: Rc<RefCell<Light>>) -> Self {
        LightOffCommand { light, level: 0 }
    }
}

impl Command for LightOffCommand {
    fn execute(&mut self) {
        self.level = self.light.borrow().get_level();
        self.light.borrow_mut().off();
    }

    fn undo(&mut self) {
        self.light.borrow_mut().dim(self.level);
    }
}

pub struct MacroCommand {
    commands: Vec<Rc<RefCell<dyn Command>>>,
}

impl MacroCommand {
    pub fn new(commands: Vec<Rc<RefCell<dyn Command>>>) -> Self {
        MacroCommand { commands }
    }
}

impl Command for MacroCommand {
    fn execute(&mut self) {
        for command in self.commands.iter() {
            command.borrow_mut().execute();
        }
    }

    fn undo(&mut self) {
        for command in self.commands.iter().rev() {
            command.borrow_mut().undo();
        }
    }
}

