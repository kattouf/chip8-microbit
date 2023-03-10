use crate::{decoder::{Decoder, DecodingState, DecoderError}, encoder::{Encoder, EncodingError}};
use embedded_hal::serial::{Read, Write};

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

    pub fn receive(&mut self) -> Result<([u8; 8192], u16), DecoderError> {
        loop {
            let byte = nb::block!(self.serial_reader.read()).unwrap();

            if let DecodingState::Complete(result) = self.decoder.put_byte(byte) {
                return result;
            }
        }
    }
}

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
        let encoded_data =  self.encoder.encode(data)?;
        for byte in encoded_data {
            nb::block!(self.serial_writer.write(byte)).unwrap();
        }

        Ok(())
    }
}
