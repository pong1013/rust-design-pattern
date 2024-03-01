mod fan;
mod garage_door;
mod light;

pub use fan::*;
pub use garage_door::*;
pub use light::*;

pub trait Command {
    fn name(&self) -> String;
    fn execute(&self);
}

pub struct NoCommand;
impl Command for NoCommand {
    fn name(&self) -> String {
        String::from("No command")
    }
    fn execute(&self) {}
}
