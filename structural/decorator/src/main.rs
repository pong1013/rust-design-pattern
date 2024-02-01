use crate::beverage::*;
use decorator::*;

fn main() {
    let beverage = Espresso;
    println!("{}: {}", beverage.discription(), beverage.cost());

    let mut mocha = Mocha::new(Box::new(beverage));
    mocha = Mocha::new(Box::new(mocha));
    println!("{}: {}", mocha.discription(), mocha.cost());

    let whip = Whip::new(Box::new(mocha));
    println!("{}: {}", whip.discription(), whip.cost());
}
