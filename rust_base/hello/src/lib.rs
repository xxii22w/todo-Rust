pub mod spanish; // 告诉编译器需要外部的一个模块

use thiserror::Error;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum DolphinError {
    #[error("The deolphin is Hungry")]
    Hungry,
    #[error("The dolphin is too young")]
    TooYoung,
    #[error("The dolpin's name is too long and annoying to say")]
    LongName,
}

use log::{debug, error, info, trace, warn};

#[derive(Debug)]
pub struct Frog {
    energy: u8,
    sleeping: bool,
}

impl Frog {
    pub fn new() -> Self {
        debug!("A new Frog has been created");
        Default::default()
    }
    pub fn hop(&mut self) {
        self.energy -= 1;
        info!("A frog hopped! It has {} of energy left", self.energy);
        if self.energy == 0 {
            warn!("The frog will go to sleep since he ran out of energy");
            self.sleep();
        }
    }
    pub fn sleep(&mut self) {
        if !self.sleeping {
            error!("The frog is already asleep");
            self.sleeping = true;
        }
    }
}

impl Default for Frog {
    fn default() -> Self {
        let frog = Frog {
            energy: 5,
            sleeping: false,
        };
        trace!("A default frog value was generated: {:?}", frog);
        frog
    }
}
