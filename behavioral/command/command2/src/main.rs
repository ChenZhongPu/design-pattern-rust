use command2::*;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
   let mut remote = RemoteControl::new();

   let living_room_light = Light::new("Living Room");
   let kitchen_light = Light::new("Kitchen");
   let hotthub = Hottub::new();

   let rc_living_light = Rc::new(RefCell::new(living_room_light));
   let living_light_on = LightOnCommand::new(Rc::clone(&rc_living_light));
   let living_light_off = LightOffCommand::new(Rc::clone(&rc_living_light));

   let rc_kitchen_light = Rc::new(RefCell::new(kitchen_light));
   let kitchen_light_on = LightOnCommand::new(Rc::clone(&rc_kitchen_light));
   let kitchen_light_off = LightOffCommand::new(Rc::clone(&rc_kitchen_light));

   let rc_hotthub = Rc::new(RefCell::new(hotthub));
   let hottub_on = HottubOnCommand::new(Rc::clone(&rc_hotthub));
   let hottub_off = HottubOffCommand::new(Rc::clone(&rc_hotthub));

   remote.set_command(0, Box::new(living_light_on), Box::new(living_light_off));
   remote.set_command(1, Box::new(kitchen_light_on), Box::new(kitchen_light_off));
   remote.set_command(2, Box::new(hottub_on), Box::new(hottub_off));

   remote.on_button_pushed(0);
   remote.off_button_pushed(0);
   remote.on_button_pushed(1);
   remote.off_button_pushed(1);
   remote.on_button_pushed(2);
   remote.off_button_pushed(2);
}
