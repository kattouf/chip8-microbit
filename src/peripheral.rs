pub mod display;
pub mod keypad;
pub mod timer;
pub mod sound_timer;
pub mod serial;
pub mod rng;

use microbit::{hal::{twim, Twim, uarte::{Uarte, Parity, Baudrate}}, Board, pac::{twim0::frequency::FREQUENCY_A, TWIM0, TIMER0, TIMER1, UARTE0}};

use crate::peripheral::{sound_timer::SoundTimer, timer::Timer, display::Display, keypad::Keypad, rng::Rng};
use self::serial::{file_reader::SerialReader, uarte_port::UartePort};

pub struct Peripheral {
    pub delay_timer: Timer<TIMER0>,
    pub sound_timer: SoundTimer<TIMER1>,
    pub display: Display<Twim<TWIM0>>,
    pub keypad: Keypad,
    pub serial_reader: SerialReader<UartePort<UARTE0>>,
    pub rng: Rng,
}

impl Peripheral {
    pub fn new(board: Board) -> Self {
        let i2c = twim::Twim::new(board.TWIM0, board.i2c_external.into(), FREQUENCY_A::K100);
        let serial = {
            let serial = Uarte::new(
                board.UARTE0,
                board.uart.into(),
                Parity::EXCLUDED,
                Baudrate::BAUD115200,
            );
            UartePort::new(serial)
        };

        Peripheral {
            delay_timer: Timer::new(board.TIMER0),
            sound_timer: SoundTimer::new(board.TIMER1),
            display: Display::new(i2c),
            keypad: Keypad::new(board.pins),
            serial_reader: SerialReader::new(serial),
            rng: Rng::new(board.RNG),
        }
    }
}
