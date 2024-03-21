mod diner_menu;
mod pancake;
pub use diner_menu::*;
pub use pancake::*;
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
    pub fn get_name(&self) -> &str {
        return &self.name;
    }
    pub fn get_description(&self) -> &str {
        return &self.description;
    }
    pub fn get_price(&self) -> f32 {
        return self.price;
    }
    pub fn is_vegetarian(&self) -> bool {
        return self.vegetarian;
    }
}
