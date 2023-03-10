extern crate alloc;

use crate::{
    checksum_calculator::{CRCChecksumCalculator, ChecksumCalculator},
    constants::*,
};
use alloc::boxed::Box;

#[derive(Debug)]
pub enum DecoderError {
    DataIsCorrupted,
}

#[derive(Debug)]
pub enum DecodingState {
    Idle,
    InProgress,
    Complete(Result<([u8; 8192], u16), DecoderError>),
}

impl DecodingState {
    pub fn get_data(&self) -> Option<([u8; 8192], u16)> {
        match self {
            DecodingState::Complete(Ok(data)) => Some(*data),
            _ => None,
        }
    }
}

pub struct Decoder {
    checksum_calculator: Box<dyn ChecksumCalculator>,
    parsing_state: Option<Box<dyn ParsingState>>,
    data_length: u16,
    data: [u8; 8192],
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
            data_length: 0,
            data: [0; 8192],
            checksum: 0,
        }
    }

    pub fn put_byte(&mut self, byte: u8) -> DecodingState {
        if let Some(parsing_state) = self.parsing_state.take() {
            self.parsing_state = Some(parsing_state.put_byte(byte, self));

            if self.parsing_state.as_ref().unwrap().is_complete() {
                let result = match self.validate_data() {
                    Some(data) => Ok(data),
                    None => Err(DecoderError::DataIsCorrupted),
                };
                return DecodingState::Complete(result);
            } else {
                return DecodingState::InProgress;
            }
        } else {
            unreachable!();
        }
    }

    fn validate_data(&mut self) -> Option<([u8; 8192], u16)> {
        let checksum = self
            .checksum_calculator
            .calculate(&self.data[..self.data_length as usize]);
        if checksum == self.checksum {
            return Some((self.data, self.data_length));
        } else {
            return None;
        }
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

struct ReadDataState {
    readed_bytes_count: u16,
}

impl ReadDataState {
    fn new() -> Self {
        ReadDataState {
            readed_bytes_count: 0,
        }
    }
}

impl ParsingState for ReadDataState {
    fn put_byte(mut self: Box<Self>, byte: u8, decoder: &mut Decoder) -> Box<dyn ParsingState> {
        decoder.data[self.readed_bytes_count as usize] = byte;
        self.readed_bytes_count += 1;
        if self.readed_bytes_count < decoder.data_length {
            self
        } else if self.readed_bytes_count == decoder.data_length {
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
