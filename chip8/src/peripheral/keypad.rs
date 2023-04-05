use embedded_hal::digital::v2::InputPin;
use keypad::{keypad_new, keypad_struct};
use microbit::{
    board::Pins,
    hal::gpio::{p0::*, p1::*, Input, Level, OpenDrain, OpenDrainConfig, Output, PullUp},
};
use void::Void;

use crate::common::{SimpleError, SimpleResult};

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

        Keypad { keypad }
    }

    pub fn wait_for_keypress(&self) -> SimpleResult<u8> {
        let keys = self.keypad.decompose();
        let mut pressed_key: Option<(u8, u8)> = None;
        loop {
            for (row_index, row) in keys.iter().enumerate() {
                for (column_index, key) in row.iter().enumerate() {
                    let key_index = (column_index as u8, row_index as u8);
                    let is_pressed = key
                        .is_low()
                        .map_err(|_err| SimpleError("Key reading error"))?;
                    if is_pressed == false && pressed_key == Some(key_index) {
                        return Ok(self.map_to_cosmac_vip_key(key_index.0, key_index.1));
                    }
                    if is_pressed == true {
                        pressed_key = Some(key_index);
                    }
                }
            }
        }
    }

    pub fn is_pressed(&self, key: u8) -> SimpleResult<bool> {
        let (column, row) = self.map_from_cosmac_vip_key(key);

        let keys = self.keypad.decompose();
        keys[row as usize][column as usize]
            .is_low()
            .map_err(|_err| SimpleError("Key reading error"))
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
