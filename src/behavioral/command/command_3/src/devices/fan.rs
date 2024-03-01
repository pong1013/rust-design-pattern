pub struct Fan {
    location: String,
    speed: String,
}

impl Fan {
    pub fn new(location: &str) -> Self {
        Fan {
            location: String::from(location),
            speed: String::from("Off"),
        }
    }
    pub fn high(&mut self) {
        self.speed = String::from("High");
        println!("The speed of {} fan is high", self.location);
    }

    // pub fn medium(&mut self) {
    //     self.speed = Speed::Medium;
    //     println!("The speed of {} fan is medium", self.location);
    // }

    // pub fn low(&mut self) {
    //     self.speed = Speed::Low;
    //     println!("The speed of {} fan is medium", self.location);
    // }

    pub fn off(&mut self) {
        self.speed = String::from("Off");
        println!("{} fan is off", self.location);
    }
    pub fn location(&self) -> &String {
        &self.location
    }
}
