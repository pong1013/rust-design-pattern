use std::fmt;

pub trait MenuComponent: fmt::Display {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn price(&self) -> f32;
    fn is_vegetarian(&self) -> bool;
    fn add(&mut self, menu: Box<dyn MenuComponent>);
    fn remove(&mut self, menu: Box<dyn MenuComponent>);
    fn child(&self, index: usize) -> &Box<dyn MenuComponent>;
}
