use crate::{Duck, Turkey};

pub struct Turkeyadapter {
    turkey: Box<dyn Turkey>,
}
impl Turkeyadapter {
    pub fn new(turkey: Box<dyn Turkey>) -> Self {
        Turkeyadapter { turkey }
    }
}
impl Duck for Turkeyadapter {
    fn quack(&self) {
        self.turkey.gobble();
    }
    fn fly(&self) {
        for _ in 0..=5 {
            self.turkey.fly();
        }
    }
}
