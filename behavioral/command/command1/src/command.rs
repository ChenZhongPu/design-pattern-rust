use crate::Light;
use crate::GarageDoor;

pub trait Command {
    fn execute(&self);
}

pub struct NoCommand;
impl Command for NoCommand {
    fn execute(&self) {}
}

pub struct LightOnCommand {
    light: Light,
}

impl LightOnCommand {
    pub fn new(light: Light) -> Self {
        LightOnCommand { light }
    }
}

impl Command for LightOnCommand {
    fn execute(&self) {
        self.light.on();
    }
}

pub struct LightOffCommand {
    light: Light,
}

impl LightOffCommand {
    pub fn new(light: Light) -> Self {
        LightOffCommand { light }
    }
}

impl Command for LightOffCommand {
    fn execute(&self) {
        self.light.off();
    }
}

pub struct GarageDoorOpenCommand {
    garage_door: GarageDoor,
}

impl GarageDoorOpenCommand {
    pub fn new(garage_door: GarageDoor) -> Self {
        GarageDoorOpenCommand { garage_door }
    }
}

impl Command for GarageDoorOpenCommand {
    fn execute(&self) {
        self.garage_door.up();
    }
}