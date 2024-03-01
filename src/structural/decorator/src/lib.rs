pub mod beverage;

pub trait Beverage {
    fn cost(&self) -> f64;
    fn discription(&self) -> String;
}

pub trait CondimentDecorator: Beverage {
    fn new(beverage: Box<dyn Beverage>) -> Self;
}
