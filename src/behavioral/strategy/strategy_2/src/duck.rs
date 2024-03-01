use crate::behaviors::{FlyBehavior, QuackBehavior};

pub trait Duck {
    fn display(&self);
    fn perform_fly(&self, fly_behavior: &impl FlyBehavior) {
        fly_behavior.fly();
    }
    fn perform_quack(&self, quack_behavior: &impl QuackBehavior) {
        quack_behavior.quack();
    }
    fn swim(&self) {
        println!("All ducks float, even decoys!");
    }
}

pub struct MallardDuck;
impl Duck for MallardDuck {
    fn display(&self) {
        println!("I'm a real Mallard duck");
    }
}
