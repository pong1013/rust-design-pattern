pub struct DVDplayer {
    description: String,
    has_dvd: bool,
}
impl DVDplayer {
    pub fn new(description: &str) -> Self {
        DVDplayer {
            description: String::from(description),
            has_dvd: false,
        }
    }
    pub fn on(&self) {
        println!("{} on", self.description);
    }
    pub fn off(&self) {
        println!("{} off", self.description);
    }
    pub fn insert_DVD(&mut self) {
        self.has_dvd = true;
        println!("{} insert DVD", self.description)
    }
    pub fn eject_DVD(&mut self) {
        self.has_dvd = false;
        println!("{} eject DVD", self.description)
    }
}
