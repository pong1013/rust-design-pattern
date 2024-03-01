pub use crate::Observer;
pub struct CurrentConditionDisplay {
    id: usize,
}
impl CurrentConditionDisplay {
    pub fn new(id: usize) -> Self {
        CurrentConditionDisplay { id }
    }
}
impl Observer for CurrentConditionDisplay {
    fn update(&mut self, tmp: f64) {
        println!(
            "CurrentConditionDisplay ({}) gets temperature = {}",
            self.id, tmp
        );
    }
    fn get_id(&self) -> usize {
        self.id
    }
}
