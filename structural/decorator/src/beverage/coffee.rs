pub use crate::Beverage;
pub use crate::CondimentDecorator;

pub struct Espresso; // drink
impl Beverage for Espresso {
    fn cost(&self) -> f64 {
        1.99
    }
    fn discription(&self) -> String {
        String::from("Espresso")
    }
}

pub struct HouseBlend; //drink
impl Beverage for HouseBlend {
    fn cost(&self) -> f64 {
        0.89
    }
    fn discription(&self) -> String {
        String::from("HouseBlend")
    }
}

pub struct Mocha {
    // decorator Mocha
    beverage: Box<dyn Beverage>,
}
impl CondimentDecorator for Mocha {
    fn new(beverage: Box<dyn Beverage>) -> Mocha {
        Mocha { beverage }
    }
}

impl Beverage for Mocha {
    fn cost(&self) -> f64 {
        self.beverage.cost() + 0.2
    }
    fn discription(&self) -> String {
        self.beverage.discription() + (" + Mocha")
    }
}

pub struct Whip {
    beverage: Box<dyn Beverage>,
}

impl CondimentDecorator for Whip {
    fn new(beverage: Box<dyn Beverage>) -> Self {
        Whip { beverage }
    }
}

impl Beverage for Whip {
    fn cost(&self) -> f64 {
        self.beverage.cost() + 0.1
    }
    fn discription(&self) -> String {
        self.beverage.discription() + (" + Whip")
    }
}
