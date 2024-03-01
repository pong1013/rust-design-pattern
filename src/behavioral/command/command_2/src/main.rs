use crate::commands::*;
use crate::devices::*;
use command_2::*;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let mut remote = RemoteControl::new();

    // Add device
    let living_room_light = Light::new("Living Room");
    let kitchen_light = Light::new("Kitchen");
    let living_room_fan = Fan::new("Living Room");
    let bedroom_fan = Fan::new("Bed Room");

    // Set livingroom light Command
    let rc_living_light = Rc::new(RefCell::new(living_room_light));
    let living_light_on = LightOnCommand::new(Rc::clone(&rc_living_light));
    let living_light_off = LightOffCommand::new(Rc::clone(&rc_living_light));
    // Set kitchen light Command
    let rc_kitchen_light = Rc::new(RefCell::new(kitchen_light));
    let kitchen_light_on = LightOnCommand::new(Rc::clone(&rc_kitchen_light));
    let kitchen_light_off = LightOffCommand::new(Rc::clone(&rc_kitchen_light));
    // fan command living
    let rc_living_fan = Rc::new(RefCell::new(living_room_fan));
    let living_fan_high = FanOnCommand::new(Rc::clone(&rc_living_fan));
    let living_fan_off = FanOffCommand::new(Rc::clone(&rc_living_fan));
    // bed
    let rc_bed_fan = Rc::new(RefCell::new(bedroom_fan));
    let bed_fan_high = FanOnCommand::new(Rc::clone(&rc_bed_fan));
    let bed_fan_off = FanOffCommand::new(Rc::clone(&rc_bed_fan));

    // Set command to the remote button!!
    remote.set_command(0, Box::new(living_light_on), Box::new(living_light_off));
    remote.set_command(1, Box::new(kitchen_light_on), Box::new(kitchen_light_off));
    remote.set_command(2, Box::new(living_fan_high), Box::new(living_fan_off));
    remote.set_command(3, Box::new(bed_fan_high), Box::new(bed_fan_off));

    println!("{}", remote);

    remote.on_button_pushed(0);
    remote.off_button_pushed(0);
    remote.on_button_pushed(1);
    remote.off_button_pushed(1);
    remote.on_button_pushed(2);
    remote.off_button_pushed(2);
    remote.on_button_pushed(3);
    remote.off_button_pushed(3);
}
