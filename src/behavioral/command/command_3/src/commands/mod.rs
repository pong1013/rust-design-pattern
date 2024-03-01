mod fan;
mod garage_door;
mod light;
mod party;

pub use fan::*;
pub use garage_door::*;
pub use light::*;
pub use party::*;
pub trait Command {
    fn name(&self) -> String;
    fn execute(&mut self);
    fn undo(&mut self);
}

pub struct NoCommand;
impl Command for NoCommand {
    fn name(&self) -> String {
        String::from("No command")
    }
    fn execute(&mut self) {}
    fn undo(&mut self) {}
}
