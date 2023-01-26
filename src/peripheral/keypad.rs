use embedded_hal::digital::v2::InputPin;
use keypad::{keypad_new, keypad_struct};
use void::Void;
use microbit::{
    hal::{
        gpio::{
            p0::*,
            p1::*,
            Input, Level, OpenDrain, OpenDrainConfig, Output, PullUp,
        },
    }, board::Pins,
};

const KEYPAD_WIDTH: u8 = 4;
keypad_struct! {
    pub struct HexKeypad<Error = Void> {
        rows: (
            P0_17<Input<PullUp>>,
            P0_04<Input<PullUp>>,
            P0_09<Input<PullUp>>,
            P0_03<Input<PullUp>>,
        ),
        columns: (
            P0_10<Output<OpenDrain>>,
            P0_01<Output<OpenDrain>>,
            P0_13<Output<OpenDrain>>,
            P1_02<Output<OpenDrain>>,
        ),
    }
}

pub struct Keypad {
    keypad: HexKeypad,
}

impl Keypad {

    pub fn new(pins: Pins) -> Keypad {
        let keypad = keypad_new!(HexKeypad {
            rows: (
                pins.p0_17.into_pullup_input(),
                pins.p0_04.into_pullup_input(),
                pins.p0_09.into_pullup_input(),
                pins.p0_03.into_pullup_input(),
            ),
            columns: (
                pins.p0_10
                    .into_open_drain_output(OpenDrainConfig::HighDrive0Disconnect1, Level::Low),
                pins.p0_01
                    .into_open_drain_output(OpenDrainConfig::HighDrive0Disconnect1, Level::Low),
                pins.p0_13
                    .into_open_drain_output(OpenDrainConfig::HighDrive0Disconnect1, Level::Low),
                pins.p1_02
                    .into_open_drain_output(OpenDrainConfig::HighDrive0Disconnect1, Level::Low),
            ),
        });

        Keypad {
            keypad
        }
    }

    pub fn wait_for_keypress(&self) -> u8 {
        let keys = self.keypad.decompose();
        loop {
            for (row_index, row) in keys.iter().enumerate() {
                for (column_index, key) in row.iter().enumerate() {
                    let is_pressed = key.is_low().unwrap();
                    if is_pressed == true {
                        return (row_index * 4 + column_index) as u8;
                    }
                }
            }
        }
    }

    pub fn is_pressed(&self, key: u8) -> bool {
        let keys = self.keypad.decompose();

        let column = (key % KEYPAD_WIDTH) as usize;
        let row = (key / KEYPAD_WIDTH) as usize;

        keys[row][column].is_low().unwrap()
    }
}
