#![no_main]
#![no_std]
#![allow(unused_imports)]

use core::{cell::RefCell, ops::DerefMut};

use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use cpu::CPU;
use embedded_hal::{
    digital::v2::InputPin,
    timer::*,
};
// use panic_rtt_target as _;
use peripheral::{Peripheral, display::Display};
// use rtt_target::{rprint, rprintln, rtt_init_print};
use panic_halt as _;

use microbit::{
    hal::{
        gpio::{
            p0::*,
            p1::*,
            Input, Level, OpenDrain, OpenDrainConfig, Output, PullUp,
        },
        Timer,
    },
    pac::{interrupt, twim0::frequency::FREQUENCY_A},
};

use keypad::{keypad_new, keypad_struct};
use void::Void;

use embedded_hal::blocking::delay::DelayMs;

mod cpu;
mod peripheral;

// keypad_struct! {
//     pub struct HexKeypad<Error = Void> {
//         rows: (
//             P0_17<Input<PullUp>>,
//             P0_04<Input<PullUp>>,
//             P0_09<Input<PullUp>>,
//             P0_03<Input<PullUp>>,
//         ),
//         columns: (
//             P0_10<Output<OpenDrain>>,
//             P0_01<Output<OpenDrain>>,
//             P0_13<Output<OpenDrain>>,
//             P1_02<Output<OpenDrain>>,
//         ),
//     }
// }
// static TIMER: Mutex<RefCell<Option<Timer<microbit::pac::TIMER0>>>> = Mutex::new(RefCell::new(None));

const PROGRAM: [u8; 132] = [
    0x00,
    0xe0,
    0xa2,
    0x2a,
    0x60,
    0x0c,
    0x61,
    0x08,
    0xd0,
    0x1f,
    0x70,
    0x09,
    0xa2,
    0x39,
    0xd0,
    0x1f,
    0xa2,
    0x48,
    0x70,
    0x08,
    0xd0,
    0x1f,
    0x70,
    0x04,
    0xa2,
    0x57,
    0xd0,
    0x1f,
    0x70,
    0x08,
    0xa2,
    0x66,
    0xd0,
    0x1f,
    0x70,
    0x08,
    0xa2,
    0x75,
    0xd0,
    0x1f,
    0x12,
    0x28,
    0xff,
    0x00,
    0xff,
    0x00,
    0x3c,
    0x00,
    0x3c,
    0x00,
    0x3c,
    0x00,
    0x3c,
    0x00,
    0xff,
    0x00,
    0xff,
    0xff,
    0x00,
    0xff,
    0x00,
    0x38,
    0x00,
    0x3f,
    0x00,
    0x3f,
    0x00,
    0x38,
    0x00,
    0xff,
    0x00,
    0xff,
    0x80,
    0x00,
    0xe0,
    0x00,
    0xe0,
    0x00,
    0x80,
    0x00,
    0x80,
    0x00,
    0xe0,
    0x00,
    0xe0,
    0x00,
    0x80,
    0xf8,
    0x00,
    0xfc,
    0x00,
    0x3e,
    0x00,
    0x3f,
    0x00,
    0x3b,
    0x00,
    0x39,
    0x00,
    0xf8,
    0x00,
    0xf8,
    0x03,
    0x00,
    0x07,
    0x00,
    0x0f,
    0x00,
    0xbf,
    0x00,
    0xfb,
    0x00,
    0xf3,
    0x00,
    0xe3,
    0x00,
    0x43,
    0xe0,
    0x00,
    0xe0,
    0x00,
    0x80,
    0x00,
    0x80,
    0x00,
    0x80,
    0x00,
    0x80,
    0x00,
    0xe0,
    0x00,
    0xe0,
];

#[entry]
fn main() -> ! {
    // rtt_init_print!();

    test_program();
    loop {}
}

fn test_program() {
    let board = microbit::Board::take().unwrap();
    let peripheral = Peripheral::new(board);
    let mut cpu = CPU::new(false, peripheral);
    cpu.load_data(&PROGRAM);
    cpu.run();
}

// fn keypad_demo() {
//     let board = microbit::Board::take().unwrap();

//     let pins = board.pins;
//     let mut timer = Timer::new(board.TIMER0);

//     let keypad = keypad_new!(HexKeypad {
//         rows: (
//             pins.p0_17.into_pullup_input(),
//             pins.p0_04.into_pullup_input(),
//             pins.p0_09.into_pullup_input(),
//             pins.p0_03.into_pullup_input(),
//         ),
//         columns: (
//             pins.p0_10
//                 .into_open_drain_output(OpenDrainConfig::HighDrive0Disconnect1, Level::Low),
//             pins.p0_01
//                 .into_open_drain_output(OpenDrainConfig::HighDrive0Disconnect1, Level::Low),
//             pins.p0_13
//                 .into_open_drain_output(OpenDrainConfig::HighDrive0Disconnect1, Level::Low),
//             pins.p1_02
//                 .into_open_drain_output(OpenDrainConfig::HighDrive0Disconnect1, Level::Low),
//         ),
//     });

//     let keys = keypad.decompose();

//     loop {
//         for (row_index, row) in keys.iter().enumerate() {
//             // rprint!("row {}: ", row_index);
//             for key in row.iter() {
//                 let is_pressed = if key.is_low().unwrap() { 1 } else { 0 };
//                 // rprint!(" {} ", is_pressed);
//             }
//             // rprintln!();
//         }

//         timer.delay_ms(1000_u16);
//     }

//     // Give up ownership of the row and column pins.
//     // let ((_r0, _r1, _r2, _r3), (_c0, _c1, _c2, _c3)) = keypad.release();
// }

// fn timer_demo() {
//     let board = microbit::Board::take().unwrap();
//     let mut timer = Timer::one_shot(board.TIMER0);
//     timer.enable_interrupt();
//     unsafe {
//         microbit::pac::NVIC::unmask(microbit::pac::Interrupt::TIMER0);
//     }

//     // rprintln!("Start");
//     timer.start(1_000_000_u32);

//     cortex_m::interrupt::free(move |cs| {
//         *TIMER.borrow(cs).borrow_mut() = Some(timer);
//     });
// }

// #[interrupt]
// fn TIMER0() {
//     cortex_m::interrupt::free(|cs| {
//         // rprintln!("End");
//         if let Some(ref mut timer) = TIMER.borrow(cs).borrow_mut().deref_mut() {
//             timer.cancel();
//         }
//     });
// }
