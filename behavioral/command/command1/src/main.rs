use command1::*;

fn main() {
    let mut remote = SimpleRemoteControl::new();

    let light = Light;
    let garage_door = GarageDoor;
    let light_on = LightOnCommand::new(light);
    let garage_door_open = GarageDoorOpenCommand::new(garage_door);

    remote.set_command(Box::new(light_on));
    remote.button_pressed();

    remote.set_command(Box::new(garage_door_open));
    remote.button_pressed();
}