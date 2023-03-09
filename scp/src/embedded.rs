use crate::receiver::{Receiver, ReceiverState, ReceiverError};
use embedded_hal::serial::Read;

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
