use ssd1306::{mode::BufferedGraphicsMode, prelude::*, I2CDisplayInterface, Ssd1306};

const BUFFER_WIDTH: usize = 64;
const BUFFER_HEIGHT: usize = 32;
const DISPLAY_WIDTH: usize = 128;
const DISPLAY_HEIGHT: usize = 64;

pub struct Display<I2C> {
    ssd1306driver:
        Ssd1306<I2CInterface<I2C>, DisplaySize128x64, BufferedGraphicsMode<DisplaySize128x64>>,
    pixel_buffer: [bool; BUFFER_WIDTH * BUFFER_HEIGHT],
}

impl<I2C> Display<I2C>
where
    I2C: embedded_hal::blocking::i2c::Write,
{
    pub fn new(i2c: I2C) -> Display<I2C> {
        let interface = I2CDisplayInterface::new(i2c);
        let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
            .into_buffered_graphics_mode();
        display.init().unwrap();

        Display {
            ssd1306driver: display,
            pixel_buffer: [false; BUFFER_WIDTH * BUFFER_HEIGHT],
        }
    }

    pub fn clear_screen(&mut self) {
        for val in self.pixel_buffer.iter_mut() {
            *val = false;
        }
        self.ssd1306driver.clear();
        self.ssd1306driver.flush().unwrap();
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
            for offset in 0..8 {
                let sprite_bit = if (sprite_row_byte >> (7 - offset)) & 0b0000_0001 == 1 {
                    true
                } else {
                    false
                };
                let bit_location = sprite_start as usize + offset;
                if bit_location > row_end {
                    break;
                }
                let prev_value = self.pixel_buffer[bit_location];
                let new_value = self.pixel_buffer[bit_location] ^ sprite_bit;
                self.pixel_buffer[bit_location] = new_value;
                pixel_unset_flag = pixel_unset_flag || prev_value == true && new_value == false;
            }
        }

        let mut scaled_buffer = [false; DISPLAY_WIDTH * DISPLAY_HEIGHT];
        let scale = DISPLAY_WIDTH / BUFFER_WIDTH;
        for (offset, bit) in scaled_buffer.iter_mut().enumerate() {
            let x = offset % DISPLAY_WIDTH;
            let y = offset / DISPLAY_WIDTH;

            let buf_x = x / scale;
            let buf_y = y / scale;
            let buf_offset = buf_y * BUFFER_WIDTH + buf_x;

            *bit = self.pixel_buffer[buf_offset];
        }

        let mut raw_pixel_data = [0u8; DISPLAY_WIDTH * DISPLAY_HEIGHT / 8];
        for (offset, byte) in raw_pixel_data.iter_mut().enumerate() {
            let x = offset % DISPLAY_WIDTH;
            let y_base = offset / DISPLAY_WIDTH * 8;
            for y in y_base..(y_base + 8) {
                let bit_loc = y * DISPLAY_WIDTH + x;
                let value = if scaled_buffer[bit_loc] == true { 1 } else { 0 };
                *byte = *byte & !(1 << (y - y_base)) | (value << (y - y_base))
            }
        }

        self.ssd1306driver.draw(&raw_pixel_data).unwrap();
        pixel_unset_flag
    }
}
