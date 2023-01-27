use microbit::hal::{Timer as RawTimer, timer::{Instance, OneShot}};
use embedded_hal::timer::CountDown;

const FREQUENCY_HZ: u32 = 60;

pub struct Timer<T> {
    raw_timer: RawTimer<T, OneShot>
}

impl<T> Timer<T> where T: Instance {
    pub fn new(hardware_timer: T) -> Timer<T> {
        Timer {
            raw_timer: RawTimer::one_shot(hardware_timer)
        }
    }

    pub fn start(&mut self, value: u8) {
        self.raw_timer.start(1_000_000 * value as u32 / FREQUENCY_HZ);
    }

    pub fn current_value(&self) -> u8 {
        (self.raw_timer.read() / 1_000_000 * FREQUENCY_HZ) as u8
    }
}
