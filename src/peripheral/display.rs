use embedded_graphics::{draw_target::DrawTarget, pixelcolor::BinaryColor, prelude::*};
use microbit::display;
use ssd1306::{mode::BufferedGraphicsMode, prelude::*, I2CDisplayInterface, Ssd1306};

const BUFFER_WIDTH: usize = 64;
const BUFFER_HEIGHT: usize = 32;

static mut PIXEL_BUFFER: [bool; BUFFER_WIDTH * BUFFER_HEIGHT] =
    [false; BUFFER_WIDTH * BUFFER_HEIGHT];

pub struct Display<I2C> {
    ssd1306driver:
        Ssd1306<I2CInterface<I2C>, DisplaySize128x64, BufferedGraphicsMode<DisplaySize128x64>>,
    // pixel_buffer: [bool; 128 * 64],
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
            // pixel_buffer: [false; 128 * 64],
        }
    }

    pub fn clear_screen(&mut self) {
        unsafe {
            for val in PIXEL_BUFFER.iter_mut() {
                *val = false;
            }
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
        unsafe {
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
                    let prev_value = PIXEL_BUFFER[bit_location];
                    let new_value = PIXEL_BUFFER[bit_location] ^ sprite_bit;
                    PIXEL_BUFFER[bit_location] = new_value;
                    pixel_unset_flag = pixel_unset_flag || prev_value == true && new_value == false;
                }
            }

            let pixels = PIXEL_BUFFER.iter().enumerate().map({
                |(offset, bit)| {
                    Pixel(
                        Point::new(
                            (offset % BUFFER_WIDTH) as i32,
                            (offset / BUFFER_WIDTH) as i32,
                        ),
                        BinaryColor::from(*bit),
                    )
                }
            });
            let upscale_pixels = pixels.flat_map({
                |pixel| {
                    [
                        Pixel(Point::new(pixel.0.x * 2, pixel.0.y * 2), pixel.1),
                        Pixel(Point::new(pixel.0.x * 2 + 1, pixel.0.y * 2), pixel.1),
                        Pixel(Point::new(pixel.0.x * 2, pixel.0.y * 2 + 1), pixel.1),
                        Pixel(Point::new(pixel.0.x * 2 + 1, pixel.0.y * 2 + 1), pixel.1),
                    ]
                }
            });
            self.ssd1306driver.draw_iter(upscale_pixels).unwrap();
            self.ssd1306driver.flush().unwrap();
        }
        pixel_unset_flag
    }
}
