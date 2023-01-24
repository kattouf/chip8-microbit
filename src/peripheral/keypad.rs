pub struct Keypad();

impl Keypad {
    pub fn wait_for_keypress(&self) -> u8 {
        unimplemented!()
    }

    pub fn is_pressed(&self, key: u8) -> bool {
        unimplemented!()
    }
}
// use embedded_hal::blocking::delay::DelayMs;
// use void::Void;
// use keypad::{keypad_new, keypad_struct};

// use embedded_hal::{
//     digital::v2::InputPin,
//     timer::*,
// };
// use microbit::{
//     hal::{
//         gpio::{
//             p0::*,
//             p1::*,
//             Input, Level, OpenDrain, OpenDrainConfig, Output, PullUp,
//         },
//         Timer,
//     },
//     pac::{interrupt, twim0::frequency::FREQUENCY_A},
// };
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
