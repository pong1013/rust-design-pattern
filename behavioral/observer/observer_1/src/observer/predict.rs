pub use crate::Observer;
pub struct PredictDisplay {
    id: usize,
}

impl PredictDisplay {
    pub fn new(id: usize) -> Self {
        PredictDisplay { id }
    }
}

impl Observer for PredictDisplay {
    fn update(&self, tmp: f64) {
        // a fake predication
        println!(
            "PredictDisplay ({}) predicts temperature = {}",
            self.id,
            tmp + 0.1
        );
    }
    fn get_id(&self) -> usize {
        self.id
    }
}
