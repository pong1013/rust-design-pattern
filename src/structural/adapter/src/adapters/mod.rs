mod turkey;

pub use turkey::*;

pub trait Turkey {
    fn gobble(&self);
    fn fly(&self);
}
pub struct WildTurkey;
impl Turkey for WildTurkey {
    fn gobble(&self) {
        println!("Gobble!");
    }
    fn fly(&self) {
        println!("Short fly");
    }
}
