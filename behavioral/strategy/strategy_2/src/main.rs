use crate::behaviors::{FlyRocketPowered, FlyWithWings, Squeak};
use crate::duck::*;

mod behaviors;
mod duck;

fn main() {
    let fly_wing = FlyWithWings;
    let fly_rocket = FlyRocketPowered;
    let duck = MallardDuck;
    duck.swim();
    duck.display();
    duck.perform_fly(&fly_wing);
    duck.perform_fly(&fly_rocket);
}
