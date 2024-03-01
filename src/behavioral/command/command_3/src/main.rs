use crate::commands::*;
use crate::devices::*;
use command_3::*;
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

    // Macro Party
    let party_on = MacroCommand::new(vec![
        Rc::new(RefCell::new(living_light_on)),
        Rc::new(RefCell::new(kitchen_light_on)),
        Rc::new(RefCell::new(living_fan_high)),
    ]);
    let party_off = MacroCommand::new(vec![
        Rc::new(RefCell::new(living_light_off)),
        Rc::new(RefCell::new(kitchen_light_off)),
        Rc::new(RefCell::new(living_fan_off)),
    ]);

    // Set command to the remote button!!
    remote.set_command(
        3,
        Rc::new(RefCell::new(bed_fan_high)),
        Rc::new(RefCell::new(bed_fan_off)),
    );
    remote.set_command(
        4,
        Rc::new(RefCell::new(party_on)),
        Rc::new(RefCell::new(party_off)),
    );

    // undo
    remote.on_button_pushed(3);
    remote.undo_button_pushed();

    // party
    remote.on_button_pushed(4);
    remote.off_button_pushed(4);
    println!("{}", remote);
}
