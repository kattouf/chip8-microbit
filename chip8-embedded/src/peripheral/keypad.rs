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
                        return self.map_to_cosmac_vip_key(column_index as u8, row_index as u8);
                    }
                }
            }
        }
    }

    pub fn is_pressed(&self, key: u8) -> bool {
        let (column, row) = self.map_from_cosmac_vip_key(key);

        let keys = self.keypad.decompose();
        keys[row as usize][column as usize].is_low().unwrap()
    }

    fn map_to_cosmac_vip_key(&self, column: u8, row: u8) -> u8 {
        match (column, row) {
            (0, 3) => 0x1,
            (0, 2) => 0x2,
            (0, 1) => 0x3,
            (0, 0) => 0xC,
            (1, 3) => 0x4,
            (1, 2) => 0x5,
            (1, 1) => 0x6,
            (1, 0) => 0xD,
            (2, 3) => 0x7,
            (2, 2) => 0x8,
            (2, 1) => 0x9,
            (2, 0) => 0xE,
            (3, 3) => 0xA,
            (3, 2) => 0x0,
            (3, 1) => 0xB,
            (3, 0) => 0xF,
            _ => panic!("Unexpected key with column: {}, row: {}", column, row),
        }
    }

    fn map_from_cosmac_vip_key(&self, key: u8) -> (u8, u8) {
        match key {
            0x1 => (0, 3),
            0x2 => (0, 2),
            0x3 => (0, 1),
            0xC => (0, 0),
            0x4 => (1, 3),
            0x5 => (1, 2),
            0x6 => (1, 1),
            0xD => (1, 0),
            0x7 => (2, 3),
            0x8 => (2, 2),
            0x9 => (2, 1),
            0xE => (2, 0),
            0xA => (3, 3),
            0x0 => (3, 2),
            0xB => (3, 1),
            0xF => (3, 0),
            _ => panic!("Unexpected key {}", key),
        }
    }
}