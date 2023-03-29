use st7567s::{
    display::ST7567S,
    interface::{I2CDisplayInterface, I2CInterface},
};

const BUFFER_WIDTH: usize = 64;
const BUFFER_HEIGHT: usize = 32;
const DISPLAY_WIDTH: usize = 128;
const DISPLAY_HEIGHT: usize = 64;

pub struct Display<I2C> {
    display_driver: ST7567S<I2CInterface<I2C>>,
    pixel_buffer: [bool; BUFFER_WIDTH * BUFFER_HEIGHT],
}

impl<I2C> Display<I2C>
where
    I2C: embedded_hal::blocking::i2c::Write,
{
    pub fn new(i2c: I2C) -> Display<I2C> {
        let interface = I2CDisplayInterface::new(i2c);
        let mut display = ST7567S::new(interface);
        display.init().unwrap();

        Display {
            display_driver: display,
            pixel_buffer: [false; BUFFER_WIDTH * BUFFER_HEIGHT],
        }
    }

    pub fn clear_screen(&mut self) {
        for val in self.pixel_buffer.iter_mut() {
            *val = false;
        }
        self.display_driver.clear();
        self.display_driver.flush().unwrap();
    }

    pub fn draw_sprite(
        &mut self,
        coordinate: (usize, usize),
        data: [u8; 15],
        bytes_len: usize,
    ) -> bool {
        let mut pixel_unset_flag = false;
        for byte_num in 0..bytes_len {
            if coordinate.1 + byte_num > BUFFER_HEIGHT - 1 {
                break;
            }

            let row_start = (coordinate.1 + byte_num) * BUFFER_WIDTH;
            let row_end = (coordinate.1 + byte_num + 1) * BUFFER_WIDTH - 1;

            let sprite_start: usize = row_start + coordinate.0 as usize;

            let sprite_row_byte = data[byte_num as usize];
            for bit_shift in 0..8 {
                let sprite_bit = if (sprite_row_byte >> (7 - bit_shift)) & 0b0000_0001 == 1 {
                    true
                } else {
                    false
                };
                let bit_location = sprite_start as usize + bit_shift;
                if bit_location > row_end {
                    break;
                }
                let prev_value = self.pixel_buffer[bit_location];
                let new_value = self.pixel_buffer[bit_location] ^ sprite_bit;
                self.pixel_buffer[bit_location] = new_value;
                pixel_unset_flag = pixel_unset_flag || prev_value == true && new_value == false;
            }
        }

        let scale = DISPLAY_WIDTH / BUFFER_WIDTH;
        let mut driver_friendly_data = [0u8; DISPLAY_WIDTH * DISPLAY_HEIGHT / 8];
        for (offset, byte) in driver_friendly_data.iter_mut().enumerate() {
            let disp_x = offset % DISPLAY_WIDTH;
            let disp_y_base = offset / DISPLAY_WIDTH * 8;
            for bit_shift in 0..8 {
                let disp_y = disp_y_base + bit_shift;

                let buf_x = disp_x / scale;
                let buf_y = disp_y / scale;
                let buf_offset = buf_y * BUFFER_WIDTH + buf_x;

                let bit_value = if self.pixel_buffer[buf_offset] == true {
                    1
                } else {
                    0
                };
                *byte = *byte & !(1 << bit_shift) | (bit_value << bit_shift)
            }
        }

        let scale = scale as u8;
        let coordinate = (coordinate.0 as u8, coordinate.1 as u8);

        let min_x = coordinate.0;
        let min_y = coordinate.1;
        let width = 8;
        let height = bytes_len as u8;

        let min_x = min_x * scale;
        let min_y = min_y * scale;
        let width = width * scale;
        let height = height * scale;

        let start = (min_x, min_y);
        let end = (
            (min_x + width - 1).min(DISPLAY_WIDTH as u8 - 1),
            (min_y + height - 1).min(DISPLAY_HEIGHT as u8 - 1),
        );

        self.display_driver
            .bounded_draw(&driver_friendly_data, start, end)
            .unwrap();

        pixel_unset_flag
    }
}
