pub mod clock_alignment_timer;
pub mod display;
pub mod keypad;
pub mod rng;
pub mod rom_loader;
pub mod sound_timer;
pub mod timer;

use microbit::{
    hal::{
        gpio, twim,
        uarte::{Baudrate, Parity, Uarte},
        Twim,
    },
    pac::{twim0::frequency::FREQUENCY_A, TIMER0, TIMER2, TWIM0, UARTE0},
    Board,
};

use crate::{
    common::uarte_port::UartePort,
    peripheral::{
        clock_alignment_timer::ClockAlignmentTimer, display::Display, keypad::Keypad, rng::Rng,
        rom_loader::ROMLoader, sound_timer::SoundTimer, timer::Timer,
    },
};

pub struct Peripheral {
    pub rom_loader: ROMLoader<UartePort<UARTE0>>,
    pub delay_timer: Timer<TIMER0>,
    pub sound_timer: SoundTimer,
    pub clock_alignment_timer: ClockAlignmentTimer<TIMER2>,
    pub display: Display<Twim<TWIM0>>,
    pub keypad: Keypad,
    pub rng: Rng,
}

impl Peripheral {
    pub fn new(board: Board) -> Self {
        let i2c = twim::Twim::new(board.TWIM0, board.i2c_external.into(), FREQUENCY_A::K400);
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
            rom_loader: ROMLoader::new(serial),
            delay_timer: Timer::new(board.TIMER0),
            sound_timer: SoundTimer::new(
                board.TIMER1,
                board.PWM0,
                board
                    .speaker_pin
                    .into_push_pull_output(gpio::Level::Low)
                    .degrade(),
            )
            .unwrap(),
            clock_alignment_timer: ClockAlignmentTimer::new(board.TIMER2),
            display: Display::new(i2c),
            keypad: Keypad::new(board.pins),
            rng: Rng::new(board.RNG),
        }
    }
}
