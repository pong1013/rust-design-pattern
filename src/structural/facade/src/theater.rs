use crate::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct HomeTheaterFacade {
    amp: Amplifier,
    dvdplayer: Rc<RefCell<DVDplayer>>,
    tv: TV,
}

impl HomeTheaterFacade {
    pub fn new(amp: Amplifier, dvdplayer: DVDplayer, tv: TV) -> Self {
        HomeTheaterFacade {
            amp,
            dvdplayer: Rc::new(RefCell::new(dvdplayer)),
            tv,
        }
    }

    pub fn open_theater(&mut self) {
        println!("Welcome to home theater...");
        self.tv.on();
        self.dvdplayer.borrow().on();
        self.dvdplayer.borrow_mut().insert_DVD();
        self.amp.on();
        self.amp.set_volume(5);
        self.amp.set_DVDplayer(Rc::clone(&self.dvdplayer));
    }

    pub fn stop_theater(&self) {
        println!("Shutting down...");
        self.dvdplayer.borrow_mut().eject_DVD();
        self.dvdplayer.borrow().off();
        self.amp.off();
        self.tv.off();
    }
}
