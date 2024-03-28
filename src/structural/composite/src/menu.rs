use super::MenuComponent;
use std::fmt;
pub struct Menu {
    name: String,
    description: String,
    menu_components: Vec<Box<dyn MenuComponent>>,
}
impl Menu {
    pub fn new(name: &str, description: &str) -> Self {
        Menu {
            name: name.to_string(),
            description: description.to_string(),
            menu_components: vec![],
        }
    }
}
impl fmt::Display for Menu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buff = String::new();
        buff.push_str(&format!("\n{} - {}", self.name(), self.description()));
        buff.push_str("\n--------\n");

        for menu in self.menu_components.iter() {
            buff.push_str(&format!("{}\n", menu.as_ref()));
        }

        write!(f, "{}", buff)
    }
}
impl MenuComponent for Menu {
    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn price(&self) -> f32 {
        unimplemented!();
    }

    fn is_vegetarian(&self) -> bool {
        unimplemented!()
    }

    fn add(&mut self, menu: Box<dyn MenuComponent>) {
        self.menu_components.push(menu);
    }

    fn remove(&mut self, menu: Box<dyn MenuComponent>) {
        self.menu_components
            .retain(|child| child as *const _ == &menu as *const _);
    }

    fn child(&self, index: usize) -> &Box<dyn MenuComponent> {
        self.menu_components.get(index).unwrap()
    }
}
// self.menu_components is the vector of Box<dyn MenuComponent> stored in the Menu struct.
// .retain() is a method provided by Vec which removes elements from the vector that do not satisfy the given predicate.
// |child| is a closure taking each element (child) of the vector.
// Rc::ptr_eq(child, menu) checks if the raw pointers of child and menu are equal. This is important because Box<dyn MenuComponent> doesn't implement PartialEq directly, but raw pointers can be compared for equality.
// !Rc::ptr_eq(child, menu) negates the result, meaning it will retain elements where the raw pointers are not equal.
// So, overall, the .retain() method removes any elements from self.menu_components where the raw pointer of the element is equal to the raw pointer of menu. This effectively removes the menu from the vector.
