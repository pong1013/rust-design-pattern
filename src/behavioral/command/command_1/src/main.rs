use crate::commands::*;
use crate::devices::*;
use command_1::*;

fn main() {
    let mut remote = SimpleRemoteControl::new();

    let light_on = LightOnCommand::new(Light);
    let light_off = LightOffCommand::new(Light);
    let garage_door_open = GarageDoorOpenCommand::new(GarageDoor);

    remote.set_command(Box::new(light_on));
    remote.button_pressed();
    remote.set_command(Box::new(light_off));
    remote.button_pressed();

    remote.set_command(Box::new(garage_door_open));
    remote.button_pressed();
}
