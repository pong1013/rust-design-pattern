use super::Command;
use crate::devices::GarageDoor;

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
