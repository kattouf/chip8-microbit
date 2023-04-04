use microbit::hal::{Timer as HalTimer, timer::{Instance, OneShot}};
use embedded_hal::timer::CountDown;

const FREQUENCY_HZ: u32 = 60;
const HARDWARE_FREQUENCY_HZ: u32 = 1_000_000;

pub struct Timer<T> {
    hal_timer: HalTimer<T, OneShot>,
    current_initial_delay: u32,
}

impl<T> Timer<T> where T: Instance {
    pub fn new(hal_timer: T) -> Timer<T> {
        Timer {
            hal_timer: HalTimer::one_shot(hal_timer),
            current_initial_delay: 0,
        }
    }

    pub fn start(&mut self, value: u8) {
        self.current_initial_delay = HARDWARE_FREQUENCY_HZ * value as u32 / FREQUENCY_HZ;
        self.hal_timer.start(self.current_initial_delay);
    }

    pub fn current_value(&mut self) -> u8 {
        if self.hal_timer.read() == 0 {
            self.current_initial_delay = 0;
        }

        let count_down_value = self.current_initial_delay - self.hal_timer.read();
        (count_down_value / HARDWARE_FREQUENCY_HZ * FREQUENCY_HZ) as u8
    }
}
