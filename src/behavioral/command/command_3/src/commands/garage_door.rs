use super::Command;
use crate::devices::GarageDoor;

use std::cell::RefCell;
use std::rc::Rc;

pub struct GarageDoorOpenCommand {
    garage_door: Rc<RefCell<GarageDoor>>,
}

impl GarageDoorOpenCommand {
    pub fn new(garage_door: Rc<RefCell<GarageDoor>>) -> Self {
        GarageDoorOpenCommand { garage_door }
    }
}

impl Command for GarageDoorOpenCommand {
    fn execute(&mut self) {
        self.garage_door.borrow().up();
    }
    fn name(&self) -> String {
        String::from("Garage open")
    }
    fn undo(&mut self) {
        self.garage_door.borrow().down();
    }
}

pub struct GarageDoorCloseCommand {
    garage_door: Rc<RefCell<GarageDoor>>,
}
impl GarageDoorCloseCommand {
    pub fn new(garage_door: Rc<RefCell<GarageDoor>>) -> Self {
        GarageDoorCloseCommand { garage_door }
    }
}
impl Command for GarageDoorCloseCommand {
    fn execute(&mut self) {
        self.garage_door.borrow().down();
    }
    fn name(&self) -> String {
        String::from("Garage close")
    }
    fn undo(&mut self) {
        self.garage_door.borrow().up();
    }
}
