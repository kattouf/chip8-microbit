extern crate alloc;

use crate::decoder::{Decoder, DecodingError, DecodingState};
use embedded_hal::serial::Read;

pub struct Receiver<T> {
    serial_reader: T,
    decoder: Decoder,
}

impl<T> Receiver<T>
where
    T: Read<u8>,
    T::Error: core::fmt::Debug,
{
    pub fn new(serial: T) -> Self {
        Receiver {
            serial_reader: serial,
            decoder: Decoder::default(),
        }
    }

    pub fn receive(&mut self) -> Result<&[u8], DecodingError> {
        loop {
            let byte = nb::block!(self.serial_reader.read()).unwrap_or_else(|e| {
                panic!("Error reading from serial: {:?}", e);
            });
            self.decoder.put_byte(byte);

            if let DecodingState::Complete(result) = self.decoder.get_state() {
                return match result {
                    Ok(_) => Ok(self.decoder.decoded_data().unwrap()),
                    Err(e) => Err(e.clone()),
                };
            }
        }
    }
}
