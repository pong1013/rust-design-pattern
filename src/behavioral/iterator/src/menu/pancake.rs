use crate::MenuItem;
use std::vec::IntoIter;

#[derive(Clone)]
pub struct PancakeHouseMenu {
    menu_items: Vec<MenuItem>,
}

impl PancakeHouseMenu {
    pub fn new() -> Self {
        let mut menu = PancakeHouseMenu {
            menu_items: Vec::new(),
        };
        menu.add_item(MenuItem::new(
            "K&B's Pancake Breakfast",
            "Pancakes with scrambled eggs, and toast",
            false,
            2.99,
        ));
        menu.add_item(MenuItem::new(
            "Regular Pancake Breakfast",
            "Pancakes with fried eggs, sausage",
            false,
            2.99,
        ));
        menu.add_item(MenuItem::new(
            "Blueberry Pancakes",
            "Pancakes made with fresh blueberries",
            true,
            3.49,
        ));
        menu.add_item(MenuItem::new(
            "Waffles",
            "Waffles, with your choice of blueberries or strawberries",
            true,
            3.59,
        ));
        menu
    }

    pub fn add_item(&mut self, item: MenuItem) {
        self.menu_items.push(item);
    }

    pub fn create_iter(self) -> IntoIter<MenuItem> {
        self.menu_items.into_iter()
    }
}
