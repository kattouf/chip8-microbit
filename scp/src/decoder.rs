extern crate alloc;

use crate::{
    checksum_calculator::{CRCChecksumCalculator, ChecksumCalculator},
    constants::*,
};
use alloc::{boxed::Box, vec::Vec};

#[derive(Clone, Copy)]
pub enum DecodingError {
    DataIsCorrupted,
}

#[derive(Clone, Copy)]
pub enum DecodingState {
    Idle,
    InProgress,
    Complete(Result<(), DecodingError>),
}

pub struct Decoder {
    checksum_calculator: Box<dyn ChecksumCalculator>,
    parsing_state: Option<Box<dyn ParsingState>>,
    decoding_state: DecodingState,
    data_length: u16,
    data: Vec<u8>,
    checksum: u16,
}

impl Decoder {
    pub fn default() -> Self {
        Self::new(Box::new(CRCChecksumCalculator::new()))
    }

    pub fn new(checksum_calculator: Box<dyn ChecksumCalculator>) -> Self {
        Decoder {
            checksum_calculator,
            parsing_state: Some(Box::new(WaitForStartByteState::new())),
            decoding_state: DecodingState::Idle,
            data_length: 0,
            data: Vec::new(),
            checksum: 0,
        }
    }

    pub fn put_byte(&mut self, byte: u8) {
        if let Some(parsing_state) = self.parsing_state.take() {
            self.parsing_state = Some(parsing_state.put_byte(byte, self));

            if self.parsing_state.as_ref().unwrap().is_complete() {
                if self.is_data_valid() {
                    self.decoding_state = DecodingState::Complete(Ok(()));
                } else {
                    self.decoding_state =
                        DecodingState::Complete(Err(DecodingError::DataIsCorrupted));
                }
            } else {
                self.decoding_state = DecodingState::InProgress;
            }
        } else {
            unreachable!();
        }
    }

    pub fn get_state(&self) -> &DecodingState {
        &self.decoding_state
    }

    pub fn take_decoded_data(self) -> Option<Vec<u8>> {
        if let DecodingState::Complete(Ok(_)) = self.decoding_state {
            Some(self.data)
        } else {
            None
        }
    }

    pub fn decoded_data(&self) -> Option<&Vec<u8>> {
        if let DecodingState::Complete(Ok(_)) = self.decoding_state {
            Some(&self.data)
        } else {
            None
        }
    }

    fn is_data_valid(&mut self) -> bool {
        let checksum = self.checksum_calculator.calculate(self.data.as_slice());
        checksum == self.checksum
    }
}

trait ParsingState {
    fn put_byte(self: Box<Self>, byte: u8, decoder: &mut Decoder) -> Box<dyn ParsingState>;

    fn is_complete(&self) -> bool {
        false
    }
}

struct WaitForStartByteState;

impl WaitForStartByteState {
    fn new() -> Self {
        WaitForStartByteState {}
    }
}

impl ParsingState for WaitForStartByteState {
    fn put_byte(self: Box<Self>, byte: u8, _decoder: &mut Decoder) -> Box<dyn ParsingState> {
        if byte == START_BYTE {
            Box::new(ReadDataLengthState::new())
        } else {
            self
        }
    }
}

struct ReadDataLengthState {
    readed_bytes_count: u8,
}

impl ReadDataLengthState {
    fn new() -> Self {
        ReadDataLengthState {
            readed_bytes_count: 0,
        }
    }
}

impl ParsingState for ReadDataLengthState {
    fn put_byte(mut self: Box<Self>, byte: u8, decoder: &mut Decoder) -> Box<dyn ParsingState> {
        if self.readed_bytes_count == 0 {
            decoder.data_length = byte as u16;
        } else if self.readed_bytes_count == 1 {
            decoder.data_length |= (byte as u16) << 8;
        } else {
            unreachable!();
        }

        self.readed_bytes_count += 1;
        if self.readed_bytes_count < 2 {
            self
        } else if self.readed_bytes_count == 2 {
            Box::new(ReadDataState::new())
        } else {
            unreachable!();
        }
    }
}

struct ReadDataState;

impl ReadDataState {
    fn new() -> Self {
        ReadDataState {}
    }
}

impl ParsingState for ReadDataState {
    fn put_byte(self: Box<Self>, byte: u8, decoder: &mut Decoder) -> Box<dyn ParsingState> {
        if decoder.data_length == 0 {
            return Box::new(ReadChecksumState::new());
        }

        decoder.data.push(byte);

        if decoder.data.len() < decoder.data_length.into() {
            self
        } else if decoder.data.len() == decoder.data_length.into() {
            Box::new(ReadChecksumState::new())
        } else {
            unreachable!();
        }
    }
}

struct ReadChecksumState {
    readed_bytes_count: u8,
}

impl ReadChecksumState {
    fn new() -> Self {
        ReadChecksumState {
            readed_bytes_count: 0,
        }
    }
}

impl ParsingState for ReadChecksumState {
    fn put_byte(mut self: Box<Self>, byte: u8, decoder: &mut Decoder) -> Box<dyn ParsingState> {
        if self.readed_bytes_count == 0 {
            decoder.checksum = byte as u16;
        } else if self.readed_bytes_count == 1 {
            decoder.checksum |= (byte as u16) << 8;
        } else {
            unreachable!();
        }

        self.readed_bytes_count += 1;
        if self.readed_bytes_count < 2 {
            self
        } else if self.readed_bytes_count == 2 {
            Box::new(CompleteState::new())
        } else {
            unreachable!();
        }
    }
}

struct CompleteState;

impl CompleteState {
    fn new() -> Self {
        CompleteState {}
    }
}

impl ParsingState for CompleteState {
    fn put_byte(self: Box<Self>, _byte: u8, _decoder: &mut Decoder) -> Box<dyn ParsingState> {
        self
    }

    fn is_complete(&self) -> bool {
        true
    }
}
