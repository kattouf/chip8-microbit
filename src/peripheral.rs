pub mod display;
pub mod keypad;
pub mod timer;

use microbit::{hal::{twim, Twim}, Board, pac::{twim0::frequency::FREQUENCY_A, TWIM0}};

use crate::peripheral::{timer::Timer, display::Display, keypad::Keypad};

pub struct Peripheral {
    pub delay_timer: Timer,
    pub sound_timer: Timer,
    pub display: Display<Twim<TWIM0>>,
    pub keypad: Keypad,
}

impl Peripheral {
    pub fn new(board: Board) -> Self {
        let i2c = twim::Twim::new(board.TWIM0, board.i2c_external.into(), FREQUENCY_A::K100);

        Peripheral {
            delay_timer: Timer {},
            sound_timer: Timer {},
            display: Display::new(i2c),
            keypad: Keypad {} }
    }
}
