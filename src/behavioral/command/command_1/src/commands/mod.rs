mod garage_door;
mod light;

pub use garage_door::*;
pub use light::*;

// interface
pub trait Command {
    fn execute(&self);
}

pub struct NoCommand;
impl Command for NoCommand {
    fn execute(&self) {}
}
