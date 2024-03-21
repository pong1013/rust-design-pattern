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

    pub fn print_menu(self) {
        let pancake_iter: std::vec::IntoIter<MenuItem> = self.pancake.create_iter();
        let diner_iter: std::array::IntoIter<MenuItem, 6> = self.diner.create_iter();
        println!("Menu\n----\nBreakfast");
        Waitress::print_menu_private(pancake_iter);
        println!("\nLunch");
        Waitress::print_menu_private(diner_iter);
    }

    fn print_menu_private(mut iterator: impl Iterator<Item = MenuItem>) {
        while let Some(item) = iterator.next() {
            println!(
                "{}  ${}  {} Vegetarian: {}",
                item.get_name(),
                item.get_price(),
                item.get_description(),
                item.is_vegetarian()
            );
        }
    }
}
