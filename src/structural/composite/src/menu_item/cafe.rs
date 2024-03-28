use super::MenuItem;
use crate::menu_component::MenuComponent;
use std::collections::hash_map::IntoIter;
use std::collections::HashMap;
#[derive(Clone)]
pub struct CafeMenu {
    menu_items: HashMap<String, MenuItem>,
}

impl CafeMenu {
    pub fn new() -> Self {
        let mut menu = CafeMenu {
            menu_items: HashMap::new(),
        };
        menu.add_item(
            "Veggie Burger",
            "Veggie burger on a whole weat bun, lettuce, tomato, and fries",
            true,
            10.9,
        );
        menu.add_item("BLT", "", false, 17.0);
        menu.add_item(
            "Soup of the day",
            "A cup of the soup of the day, with a side salad",
            true,
            10.0,
        );
        menu.add_item("Steamed Veggies and Brown Rice", "", true, 12.5);
        menu.add_item("Pasta", "", false, 17.5);
        menu
    }

    pub fn add_item(&mut self, name: &str, description: &str, vegetarian: bool, price: f32) {
        let menu_item = MenuItem::new(name, description, vegetarian, price);
        self.menu_items
            .insert(String::from(menu_item.name()), menu_item);
    }

    pub fn create_iter(self) -> IntoIter<String, MenuItem> {
        self.menu_items.into_iter()
    }
}
