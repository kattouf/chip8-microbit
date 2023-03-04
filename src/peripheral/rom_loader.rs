use embedded_hal::serial::Read;

/// COSMAC VIP RAM size - interpreter implementation memory - variables and display refresh memory
const ROM_SIZE: usize = 0xFFF - 0x1FF - 0x160;

pub struct ROMLoader<T> {
    serial_reader: T,
}

impl<T> ROMLoader<T>
where
    T: Read<u8>,
    T::Error: core::fmt::Debug,
{
    pub fn new(serial: T) -> Self {
        ROMLoader {
            serial_reader: serial,
        }
    }

    pub fn load(&mut self) -> [u8; ROM_SIZE] {
        let mut prev_byte: u8 = 0;

        let mut buf = [0u8; ROM_SIZE];
        let mut buf_p = 0_usize;
        loop {
            let byte = nb::block!(self.serial_reader.read()).unwrap();

            if byte == b'd' && prev_byte == b'e' {
                break;
            }
            prev_byte = byte;

            if buf_p == ROM_SIZE {
                panic!("rom space overflow");
            }
            buf[buf_p] = byte;
            buf_p += 1;
        }

        buf
    }
}
