use super::MenuItem;
use std::array::IntoIter;
#[derive(Clone)]
pub struct DinerMenu {
    number_of_items: usize,
    menu_items: [MenuItem; 6],
}

impl DinerMenu {
    pub fn new() -> Self {
        let mut menu = DinerMenu {
            number_of_items: 0,
            menu_items: Default::default(),
        };
        menu.add_item(MenuItem::new("Vegetarian BLT", "", true, 15.5));
        menu.add_item(MenuItem::new("BLT", "", false, 17.0));
        menu.add_item(MenuItem::new("Soup of the day", "", true, 10.0));
        menu.add_item(MenuItem::new("Hotdog", "", false, 15.0));
        menu.add_item(MenuItem::new(
            "Steamed Veggies and Brown Rice",
            "",
            true,
            12.5,
        ));
        menu.add_item(MenuItem::new("Pasta", "", false, 17.5));
        menu
    }

    pub fn add_item(&mut self, item: MenuItem) {
        if self.number_of_items >= 6 {
            eprintln!("Sorry, menu is full! Can't add item to menu");
        } else {
            self.menu_items[self.number_of_items] = item;
            self.number_of_items += 1;
        }
    }

    pub fn create_iter(self) -> IntoIter<MenuItem, 6> {
        self.menu_items.into_iter()
    }
    //This change ensures that the returned reference lives as long as the DinerMenu instance ('a), eliminating the mismatch with the 'static lifetime.
}
