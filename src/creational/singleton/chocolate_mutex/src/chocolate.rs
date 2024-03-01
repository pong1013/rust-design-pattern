use std::mem::MaybeUninit;
use std::sync::Mutex;
use std::sync::Once;

pub static mut CHOCOLATE_BOILER: MaybeUninit<ChocolateBoiler> = MaybeUninit::uninit();
pub static ONCE: Once = Once::new();
pub static mut INIT: Option<fn() -> state> = None;

pub struct state {
    pub empty: bool,
    pub boiled: bool,
}

pub struct ChocolateBoiler {
    inner: Mutex<state>,
}
impl ChocolateBoiler {
    pub fn instance() -> &'static mut ChocolateBoiler {
        ONCE.call_once(|| {
            unsafe {
                if let None = INIT {
                    panic!(
                        "Singeleton of chocolate boiler must be initialized before its being used."
                    )
                }
            }
            let init = unsafe { INIT.unwrap() };
            let value = init();

            let chocolate_boiler = ChocolateBoiler {
                inner: Mutex::new(value),
            };

            unsafe {
                CHOCOLATE_BOILER.write(chocolate_boiler);
            }
        });
        unsafe { CHOCOLATE_BOILER.assume_init_mut() }
    }
    pub fn drain(&mut self) {
        if !self.is_empty() {
            println!("Draining the boiler...");
            *self.inner.get_mut().unwrap() = state {
                empty: true,
                boiled: false,
            }
        } else {
            println!("The boiler is empty.");
        }
    }
    pub fn fill(&mut self) {
        if self.is_empty() {
            println!("Filling the boiler with a milk/chocolate mixture...");
            *self.inner.get_mut().unwrap() = state {
                empty: false,
                boiled: false,
            };
        } else {
            println!("The boiler has already filled...");
        }
    }
    pub fn boil(&mut self) {
        if !self.is_empty() && !self.is_Boiled() {
            println!("Start boiling the chocolate...");
            *self.inner.get_mut().unwrap() = state {
                empty: false,
                boiled: true,
            };
        } else {
            println!("The boiler has already boiled...")
        }
    }
    pub fn is_empty(&self) -> bool {
        (*self.inner.lock().unwrap()).empty
    }
    pub fn is_Boiled(&self) -> bool {
        (*self.inner.lock().unwrap()).boiled
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn chocolate_boiler() {
        unsafe {
            super::INIT = Some(|| state {
                empty: false,
                boiled: false,
            });
        }
        let instance1 = super::instance();
        let instance2 = super::instance();

        assert_eq!((*instance1.inner.lock().unwrap()).is_Boiled, false);
        assert_eq!((*instance2.inner.lock().unwrap()).is_Boiled, false);

        *instance1.inner.get_mut().unwrap() = state {
            empty: false,
            boiled: true,
        };
        assert_eq!((*instance1.inner.lock().unwrap()).boiled, true);

        *instance2.inner.get_mut().unwrap() = state {
            empty: false,
            boiled: false,
        };
        assert_eq!((*instance2.inner.lock().unwrap()).boiled, false);
    }
}
