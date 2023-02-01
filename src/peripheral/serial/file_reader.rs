use embedded_hal::serial::Read;

pub struct SerialReader<T> {
    serial: T,
}

impl<T> SerialReader<T>
where
    T: Read<u8>, T::Error: core::fmt::Debug,
{
    pub fn new(serial: T) -> Self {
        SerialReader { serial }
    }

    pub fn fetch_rom_from_serial_port(&mut self) -> [u8; 0xc8f] {
        let mut prev_byte: u8 = 0;

        let mut buf = [0u8; 0xc8f];
        let mut buf_p = 0_usize;
        loop {
            let byte = nb::block!(self.serial.read()).unwrap();

            if byte == b'd' && prev_byte == b'e' {
                break;
            }
            prev_byte = byte;

            if buf_p == 0xc8f - 1 {
                panic!("program space overflow");
            }
            buf[buf_p] = byte;
            buf_p += 1;
        }

        buf
    }
}
