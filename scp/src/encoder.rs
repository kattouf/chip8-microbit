extern crate alloc;

use crate::{
    checksum_calculator::{CRCChecksumCalculator, ChecksumCalculator},
    constants::*,
};
use alloc::{boxed::Box, vec::Vec};

pub enum EncodingError {
    PayloadTooLong,
}

pub struct Encoder {
    checksum_calculator: Box<dyn ChecksumCalculator>,
}

impl Encoder {
    pub fn default() -> Self {
        Self {
            checksum_calculator: Box::new(CRCChecksumCalculator::new()),
        }
    }

    pub fn new(checksum_calculator: Box<dyn ChecksumCalculator>) -> Encoder {
        Encoder {
            checksum_calculator,
        }
    }

    pub fn encode(&self, payload: &[u8]) -> Result<Vec<u8>, EncodingError> {
        if payload.len() > PAYLOAD_MAX_SIZE.into() {
            return Err(EncodingError::PayloadTooLong);
        }
        let header: &[u8] = &[START_BYTE];
        let length: &[u8] = &(payload.len() as u16).to_ne_bytes();
        let checksum: &[u8] = &self.checksum_calculator.calculate(payload).to_ne_bytes();

        let protocol_message = [header, length, payload, checksum].concat();
        return Ok(protocol_message);
    }
}
