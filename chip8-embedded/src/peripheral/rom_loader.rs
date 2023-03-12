use embedded_hal::serial::Read;
use scp::receiver::Receiver;

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

    pub fn load(&mut self) -> &[u8] {
        self.receiver.receive().unwrap()
    }
}
