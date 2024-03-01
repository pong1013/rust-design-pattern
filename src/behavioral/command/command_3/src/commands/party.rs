use super::Command;

use std::cell::RefCell;
use std::rc::Rc;

pub struct MacroCommand {
    commands: Vec<Rc<RefCell<dyn Command>>>,
}
impl MacroCommand {
    pub fn new(commands: Vec<Rc<RefCell<dyn Command>>>) -> Self {
        MacroCommand { commands }
    }
}
impl Command for MacroCommand {
    fn name(&self) -> String {
        String::from("Party Mode")
    }
    fn execute(&mut self) {
        for command in &self.commands {
            command.borrow_mut().execute();
        }
    }
    fn undo(&mut self) {
        for command in &self.commands {
            command.borrow_mut().undo();
        }
    }
}
