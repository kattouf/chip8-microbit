pub mod display;
pub mod keypad;
pub mod timer;
pub mod sound_timer;

use microbit::{hal::{twim, Twim}, Board, pac::{twim0::frequency::FREQUENCY_A, TWIM0, TIMER0, TIMER1}};

use crate::peripheral::{sound_timer::SoundTimer, timer::Timer, display::Display, keypad::Keypad};

pub struct Peripheral {
    pub delay_timer: Timer<TIMER0>,
    pub sound_timer: SoundTimer<TIMER1>,
    pub display: Display<Twim<TWIM0>>,
    pub keypad: Keypad,
}

impl Peripheral {
    pub fn new(board: Board) -> Self {
        let i2c = twim::Twim::new(board.TWIM0, board.i2c_external.into(), FREQUENCY_A::K100);

        Peripheral {
            delay_timer: Timer::new(board.TIMER0),
            sound_timer: SoundTimer::new(board.TIMER1),
            display: Display::new(i2c),
            keypad: Keypad::new(board.pins),
        }
    }
}
