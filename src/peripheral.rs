pub mod display;
pub mod keypad;
pub mod timer;

use crate::peripheral::{timer::Timer, display::Display, keypad::Keypad};

pub struct Peripheral {
    pub delay_timer: Timer,
    pub sound_timer: Timer,
    pub display: Display,
    pub keypad: Keypad,
}
impl Peripheral {
    pub fn new() -> Self {
        Peripheral { delay_timer: Timer {}, sound_timer: Timer {}, display: Display {}, keypad: Keypad {} }
    }
}
