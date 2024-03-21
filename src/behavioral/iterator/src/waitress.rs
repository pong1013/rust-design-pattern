use crate::menu::*;
use crate::MenuItem;
pub struct Waitress {
    pancake: PancakeHouseMenu,
    diner: DinerMenu,
}

impl Waitress {
    pub fn new(pancake: PancakeHouseMenu, diner: DinerMenu) -> Self {
        Waitress { pancake, diner }
    }

    pub fn print_menu(&self, pancake: PancakeHouseMenu, diner: DinerMenu) {
        let pancake_iter = Box::new(pancake.create_iter().clone().into_iter())
            as Box<dyn Iterator<Item = MenuItem>>;
        let diner_iter =
            Box::new(diner.create_iter().clone().into_iter()) as Box<dyn Iterator<Item = MenuItem>>;
        println!("Menu\n----\nBreakfast");
        self.print_menu_private(pancake_iter);
        println!("\nLunch");
        self.print_menu_private(diner_iter);
    }

    fn print_menu_private(&self, mut iterator: Box<dyn Iterator<Item = MenuItem>>) {
        while let Some(item) = iterator.next() {
            println!(
                "Name: {}, Price: {}, Description: {}, Vegetarian: {}",
                item.get_name(),
                item.get_price(),
                item.get_description(),
                item.is_vegetarian()
            );
        }
    }
}
