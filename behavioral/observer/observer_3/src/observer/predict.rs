pub use crate::Observer;
pub struct PredictDisplay;
impl Observer for PredictDisplay {
    fn update(&self, tmp: f64) {
        // a fake predication
        println!("PredictDisplay predicts temperature = {}", tmp + 0.1);
    }
}
