use super::Command;
use crate::devices::Fan;

use std::cell::RefCell;
use std::rc::Rc;

pub struct FanOnCommand {
    fan: Rc<RefCell<Fan>>,
}
impl FanOnCommand {
    pub fn new(fan: Rc<RefCell<Fan>>) -> Self {
        FanOnCommand { fan }
    }
}
impl Command for FanOnCommand {
    fn execute(&self) {
        self.fan.borrow_mut().high();
    }
    fn name(&self) -> String {
        format!("{} fan on", self.fan.borrow().location())
    }
}

pub struct FanOffCommand {
    fan: Rc<RefCell<Fan>>,
}
impl FanOffCommand {
    pub fn new(fan: Rc<RefCell<Fan>>) -> Self {
        FanOffCommand { fan }
    }
}
impl Command for FanOffCommand {
    fn execute(&self) {
        self.fan.borrow_mut().off();
    }
    fn name(&self) -> String {
        format!("{} fan off", self.fan.borrow().location())
    }
}
