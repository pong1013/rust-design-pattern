use crate::commands::*;
use std::fmt;
pub struct RemoteControl {
    on_commands: Vec<Box<dyn Command>>,
    off_commands: Vec<Box<dyn Command>>,
}

impl RemoteControl {
    pub fn new() -> Self {
        RemoteControl {
            on_commands: vec![
                Box::new(NoCommand),
                Box::new(NoCommand),
                Box::new(NoCommand),
                Box::new(NoCommand),
            ],
            off_commands: vec![
                Box::new(NoCommand),
                Box::new(NoCommand),
                Box::new(NoCommand),
                Box::new(NoCommand),
            ],
        }
    }

    pub fn set_command(
        &mut self,
        slot: usize,
        on_command: Box<dyn Command>,
        off_command: Box<dyn Command>,
    ) {
        self.on_commands[slot] = on_command;
        self.off_commands[slot] = off_command;
    }

    pub fn on_button_pushed(&self, slot: usize) {
        self.on_commands[slot].execute();
    }

    pub fn off_button_pushed(&self, slot: usize) {
        self.off_commands[slot].execute();
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
                on_cmd.name(),
                off_cmd.name()
            ));
        }

        write!(f, "{}", string_buff)
    }
}
