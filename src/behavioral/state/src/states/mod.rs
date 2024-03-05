mod has_quater;
mod no_quater;
mod sold;
mod sold_out;

pub use no_quater::*;
pub use sold::*;
pub use sold::*;
pub use sold_out::*;

// State
use crate::GumballMachine;

pub trait State {
    fn insert_quarter(self: Box<Self>) -> Box<dyn State>;
    fn eject_quarter(self: Box<Self>) -> Box<dyn State>;
    fn turn_crank(self: Box<Self>) -> Box<dyn State>;
    fn dispense<'a>(self: Box<Self>, machine: &'a mut GumballMachine) -> Box<dyn State>;
}
