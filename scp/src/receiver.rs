extern crate alloc;

use crate::{
    checksum_calculator::{CRCChecksumCalculator, ChecksumCalculator},
    constants::START_BYTE,
};
use alloc::boxed::Box;

#[derive(Debug)]
pub enum ReceiverError {
    DataIsCorrupted,
}

#[derive(Debug)]
pub enum ReceiverState {
    Idle,
    InProgress,
    Complete(Result<([u8; 8192], u16), ReceiverError>),
}

impl ReceiverState {
    pub fn get_data(&self) -> Option<([u8; 8192], u16)> {
        match self {
            ReceiverState::Complete(Ok(data)) => Some(*data),
            _ => None,
        }
    }
}

pub struct Receiver {
    checksum_calculator: Box<dyn ChecksumCalculator>,
    state: Option<Box<dyn ReceivingState>>,
    data_length: u16,
    data: [u8; 8192],
    checksum: u16,
}

impl Receiver {
    pub fn default() -> Self {
        Self::new(Box::new(CRCChecksumCalculator::new()))
    }

    pub fn new(checksum_calculator: Box<dyn ChecksumCalculator>) -> Self {
        Receiver {
            checksum_calculator,
            state: Some(Box::new(WaitForStartByteState::new())),
            data_length: 0,
            data: [0; 8192],
            checksum: 0,
        }
    }

    pub fn put_byte(&mut self, byte: u8) -> ReceiverState {
        if let Some(state) = self.state.take() {
            self.state = Some(state.put_byte(byte, self));

            if self.state.as_ref().unwrap().is_complete() {
                let result = match self.get_data_if_valid() {
                    Some(data) => Ok(data),
                    None => Err(ReceiverError::DataIsCorrupted),
                };
                return ReceiverState::Complete(result);
            } else {
                return ReceiverState::InProgress;
            }
        } else {
            unreachable!();
        }
    }

    fn get_data_if_valid(&mut self) -> Option<([u8; 8192], u16)> {
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

trait ReceivingState {
    fn put_byte(self: Box<Self>, byte: u8, receiver: &mut Receiver) -> Box<dyn ReceivingState>;

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

impl ReceivingState for WaitForStartByteState {
    fn put_byte(self: Box<Self>, byte: u8, _receiver: &mut Receiver) -> Box<dyn ReceivingState> {
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

impl ReceivingState for ReadDataLengthState {
    fn put_byte(mut self: Box<Self>, byte: u8, receiver: &mut Receiver) -> Box<dyn ReceivingState> {
        if self.readed_bytes_count == 0 {
            receiver.data_length = byte as u16;
        } else if self.readed_bytes_count == 1 {
            receiver.data_length |= (byte as u16) << 8;
        } else {
            unreachable!();
        }

        self.readed_bytes_count += 1;
        if self.readed_bytes_count == 2 {
            Box::new(ReadDataState::new())
        } else {
            self
        }
    }
}

struct ReadDataState {
    readed_bytes: u16,
}

impl ReadDataState {
    fn new() -> Self {
        ReadDataState { readed_bytes: 0 }
    }
}

impl ReceivingState for ReadDataState {
    fn put_byte(mut self: Box<Self>, byte: u8, receiver: &mut Receiver) -> Box<dyn ReceivingState> {
        receiver.data[self.readed_bytes as usize] = byte;
        self.readed_bytes += 1;
        if self.readed_bytes == receiver.data_length {
            Box::new(ReadChecksumState::new())
        } else {
            self
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

impl ReceivingState for ReadChecksumState {
    fn put_byte(mut self: Box<Self>, byte: u8, receiver: &mut Receiver) -> Box<dyn ReceivingState> {
        if self.readed_bytes_count == 0 {
            receiver.checksum = byte as u16;
        } else if self.readed_bytes_count == 1 {
            receiver.checksum |= (byte as u16) << 8;
        } else {
            unreachable!();
        }

        self.readed_bytes_count += 1;
        if self.readed_bytes_count == 2 {
            Box::new(CompleteState::new())
        } else {
            self
        }
    }
}

struct CompleteState;

impl CompleteState {
    fn new() -> Self {
        CompleteState {}
    }
}

impl ReceivingState for CompleteState {
    fn put_byte(self: Box<Self>, _byte: u8, _receiver: &mut Receiver) -> Box<dyn ReceivingState> {
        self
    }

    fn is_complete(&self) -> bool {
        true
    }
}
