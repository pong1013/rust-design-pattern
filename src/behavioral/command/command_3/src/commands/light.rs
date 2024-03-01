use super::Command;
use crate::devices::Light;

use std::cell::RefCell;
use std::rc::Rc;
// light同時會被on, off兩種指令借用，所以需要 Rc<> 來達成多重借用
// 優些設備會有不同的狀態，如high, low，因此需要 RefCell<> 來達成可變借用
pub struct LightOnCommand {
    light: Rc<RefCell<Light>>,
}
impl LightOnCommand {
    pub fn new(light: Rc<RefCell<Light>>) -> Self {
        LightOnCommand { light }
    }
}
impl Command for LightOnCommand {
    fn execute(&mut self) {
        self.light.borrow().on();
    }
    fn name(&self) -> String {
        format!("{} light on", self.light.borrow().location())
    }
    fn undo(&mut self) {
        self.light.borrow().off();
    }
}

pub struct LightOffCommand {
    light: Rc<RefCell<Light>>,
}
impl LightOffCommand {
    pub fn new(light: Rc<RefCell<Light>>) -> Self {
        LightOffCommand { light }
    }
}
impl Command for LightOffCommand {
    fn execute(&mut self) {
        self.light.borrow().off();
    }
    fn name(&self) -> String {
        format!("{} light off", self.light.borrow().location())
    }
    fn undo(&mut self) {
        self.light.borrow().on();
    }
}
