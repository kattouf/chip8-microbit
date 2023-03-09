use microbit::hal::{Timer as HalTimer, timer::{Instance, OneShot}};

const FREQUENCY_HZ: u32 = 400;
const HARDWARE_FREQUENCY_HZ: u32 = 1_000_000;

pub struct ClockAlignmentTimer<T> {
    raw_timer: HalTimer<T, OneShot>,
}

impl<T> ClockAlignmentTimer<T> where T: Instance {
    pub fn new(hardware_timer: T) -> ClockAlignmentTimer<T> {
        ClockAlignmentTimer {
            raw_timer: HalTimer::one_shot(hardware_timer),
        }
    }

    pub fn wait(&mut self) {
        self.raw_timer.delay(HARDWARE_FREQUENCY_HZ / FREQUENCY_HZ);
    }
}
