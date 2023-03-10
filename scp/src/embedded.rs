use crate::{receiver::{Receiver, ReceiverState, ReceiverError}, transmitter::{Transmitter, TransmitterError}};
use embedded_hal::serial::{Read, Write};

pub struct SerialReceiver<T> {
    serial_reader: T,
    receiver: Receiver,
}

impl<T> SerialReceiver<T>
where
    T: Read<u8>,
    T::Error: core::fmt::Debug,
{
    pub fn new(serial: T) -> Self {
        SerialReceiver {
            serial_reader: serial,
            receiver: Receiver::default(),
        }
    }

    pub fn receive(&mut self) -> Result<([u8; 8192], u16), ReceiverError> {
        loop {
            let byte = nb::block!(self.serial_reader.read()).unwrap();

            if let ReceiverState::Complete(result) = self.receiver.put_byte(byte) {
                return result;
            }
        }
    }
}

pub struct SerialTransmitter<T> {
    serial_writer: T,
    transmitter: Transmitter,
}

impl<T> SerialTransmitter<T>
where
    T: Write<u8>,
    T::Error: core::fmt::Debug,
{
    pub fn new(serial: T) -> Self {
        SerialTransmitter {
            serial_writer: serial,
            transmitter: Transmitter::default(),
        }
    }

    pub fn transmit(&mut self, data: &[u8]) -> Result<(), TransmitterError> {
        let encoded_data =  self.transmitter.prepare_to_transmit(data)?;
        for byte in encoded_data {
            nb::block!(self.serial_writer.write(byte)).unwrap();
        }

        Ok(())
    }
}
