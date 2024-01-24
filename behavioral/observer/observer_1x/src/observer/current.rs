pub use crate::Observer;
pub struct CurrentConditionDisplay;

impl Observer for CurrentConditionDisplay {
    fn update(&self, tmp: f64) {
        println!("CurrentConditionDisplay gets temperature = {}", tmp);
    }
}
