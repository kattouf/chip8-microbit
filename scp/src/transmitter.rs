extern crate alloc;

use crate::encoder::{Encoder, EncodingError};
use embedded_hal::serial::Write;

pub struct Transmitter<T> {
    serial_writer: T,
    encoder: Encoder,
}

impl<T> Transmitter<T>
where
    T: Write<u8>,
    T::Error: core::fmt::Debug,
{
    pub fn new(serial: T) -> Self {
        Transmitter {
            serial_writer: serial,
            encoder: Encoder::default(),
        }
    }

    pub fn transmit(&mut self, data: &[u8]) -> Result<(), EncodingError> {
        let encoded_data = self.encoder.encode(data)?;
        for byte in encoded_data {
            nb::block!(self.serial_writer.write(byte)).unwrap();
        }

        Ok(())
    }
}
