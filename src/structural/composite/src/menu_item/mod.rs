mod cafe;

pub use cafe::*;

use super::MenuComponent;

use std::fmt;
#[derive(Default, Clone)]
pub struct MenuItem {
    name: String,
    description: String,
    vegetarian: bool,
    price: f32,
}
impl MenuItem {
    pub fn new(name: &str, description: &str, vegetarian: bool, price: f32) -> Self {
        MenuItem {
            name: String::from(name),
            description: String::from(description),
            vegetarian,
            price,
        }
    }
}
impl fmt::Display for MenuItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\n{}{}, {} -- {}",
            self.name(),
            if self.is_vegetarian() { "(v)" } else { "" },
            self.price(),
            self.description()
        )
    }
}

impl MenuComponent for MenuItem {
    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn price(&self) -> f32 {
        self.price
    }

    fn is_vegetarian(&self) -> bool {
        self.vegetarian
    }
    fn remove(&mut self, menu: Box<dyn MenuComponent>) {
        panic!("Ahhhhhhh")
    }
    fn child(&self, index: usize) -> &Box<dyn MenuComponent> {
        panic!("Ahhhhh");
    }
    fn add(&mut self, menu: Box<dyn MenuComponent>) {
        panic!("Ahhhhhh");
    }
}
