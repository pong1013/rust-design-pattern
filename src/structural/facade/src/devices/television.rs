pub struct TV {
    description: String,
}

impl TV {
    pub fn new(description: &str) -> Self {
        TV {
            description: String::from(description),
        }
    }
    pub fn on(&self) {
        println!("{} on", self.description);
    }
    pub fn off(&self) {
        println!("{} off", self.description);
    }
}
