use crate::commands::*;
use std::fmt;

use std::cell::RefCell;
use std::rc::Rc;
pub struct RemoteControl {
    on_commands: Vec<Rc<RefCell<dyn Command>>>,
    off_commands: Vec<Rc<RefCell<dyn Command>>>,
    undo_command: Rc<RefCell<dyn Command>>,
}

impl RemoteControl {
    pub fn new() -> Self {
        RemoteControl {
            on_commands: vec![Rc::new(RefCell::new(NoCommand)); 5],
            off_commands: vec![Rc::new(RefCell::new(NoCommand)); 5],
            undo_command: Rc::new(RefCell::new(NoCommand)),
        }
    }

    pub fn set_command(
        &mut self,
        slot: usize,
        on_command: Rc<RefCell<dyn Command>>,
        off_command: Rc<RefCell<dyn Command>>,
    ) {
        self.on_commands[slot] = on_command;
        self.off_commands[slot] = off_command;
    }

    pub fn on_button_pushed(&mut self, slot: usize) {
        self.on_commands[slot].borrow_mut().execute();
        self.undo_command = Rc::clone(&self.on_commands[slot]);
    }

    pub fn off_button_pushed(&mut self, slot: usize) {
        self.off_commands[slot].borrow_mut().execute();
        self.undo_command = Rc::clone(&self.off_commands[slot]);
    }

    pub fn undo_button_pushed(&mut self) {
        self.undo_command.borrow_mut().undo();
    }
}

impl fmt::Display for RemoteControl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut string_buff = String::new();
        string_buff.push_str("\n----- Remote Control -----\n");

        for (i, (on_cmd, off_cmd)) in self.on_commands.iter().zip(&self.off_commands).enumerate() {
            string_buff.push_str(&format!(
                "[slot {}] {} - {}\n",
                i,
                on_cmd.borrow().name(),
                off_cmd.borrow().name()
            ));
        }

        write!(f, "{}", string_buff)
    }
}
