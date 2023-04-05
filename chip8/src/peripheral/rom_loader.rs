use embedded_hal::serial::Read;
use scp::receiver::Receiver;
use crate::common::{SimpleResult, SimpleError};

pub struct ROMLoader<T> {
    receiver: Receiver<T>,
}

impl<T> ROMLoader<T>
where
    T: Read<u8>,
    T::Error: core::fmt::Debug,
{
    pub fn new(serial: T) -> Self {
        ROMLoader {
            receiver: Receiver::new(serial)
        }
    }

    pub fn load(&mut self) -> SimpleResult<&[u8]> {
        self.receiver.receive().map_err(|_err| SimpleError("ROM receiving error"))
    }
}
