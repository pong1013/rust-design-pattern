use super::dvd::DVDplayer;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Amplifier {
    description: String,
    dvdplayer: Option<Rc<RefCell<DVDplayer>>>,
    level: i32,
}

impl Amplifier {
    pub fn new(description: &str) -> Self {
        Amplifier {
            description: String::from(description),
            dvdplayer: None,
            level: 0,
        }
    }

    pub fn on(&self) {
        println!("{} on", self.description);
    }

    pub fn off(&self) {
        println!("{} off", self.description);
    }

    pub fn set_volume(&mut self, level: i32) {
        self.level = level;
        println!("{} setting volume to {}", self.description, level);
    }

    pub fn set_DVDplayer(&mut self, dvdplayer: Rc<RefCell<DVDplayer>>) {
        self.dvdplayer = Some(dvdplayer);
    }
}
