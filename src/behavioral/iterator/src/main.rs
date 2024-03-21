use iterator::*;

fn main() {
    let diner_menu = DinerMenu::new();
    let pancake_menu = PancakeHouseMenu::new();
    let waitress = Waitress::new(pancake_menu.clone(), diner_menu.clone());
    waitress.print_menu();
}
