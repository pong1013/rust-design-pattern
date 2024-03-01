use super::Command;
use crate::devices::Light;

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
