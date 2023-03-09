use microbit::hal::{Timer as RawTimer, timer::{Instance, OneShot}};
use embedded_hal::timer::CountDown;

const FREQUENCY_HZ: u32 = 60;

pub struct SoundTimer<T> {
    raw_timer: RawTimer<T, OneShot>
}

impl<T> SoundTimer<T> where T: Instance {
    pub fn new(hardware_timer: T) -> SoundTimer<T> {
        SoundTimer {
            raw_timer: RawTimer::one_shot(hardware_timer)
        }
    }

    pub fn start(&mut self, value: u8) {
        self.raw_timer.start(1_000_000 * value as u32 / FREQUENCY_HZ);
    }
}
// use embedded_hal::{
//     digital::v2::InputPin,
//     timer::*,
// };

// use microbit::{
//     hal::{
//         gpio::{
//             p0::*,
//             p1::*,
//             Input, Level, OpenDrain, OpenDrainConfig, Output, PullUp,
//         },
//         Timer,
//     },
//     pac::{interrupt, twim0::frequency::FREQUENCY_A},
// };


// use cortex_m::interrupt::Mutex;
// use core::{cell::RefCell, ops::DerefMut};
//
// static TIMER: Mutex<RefCell<Option<Timer<microbit::pac::TIMER0>>>> = Mutex::new(RefCell::new(None));
// fn timer_demo() {
//     let board = microbit::Board::take().unwrap();
//     let mut timer = Timer::one_shot(board.TIMER0);
//     timer.enable_interrupt();
//     unsafe {
//         microbit::pac::NVIC::unmask(microbit::pac::Interrupt::TIMER0);
//     }

//     // rprintln!("Start");
//     timer.start(1_000_000_u32);

//     cortex_m::interrupt::free(move |cs| {
//         *TIMER.borrow(cs).borrow_mut() = Some(timer);
//     });
// }

// #[interrupt]
// fn TIMER0() {
//     cortex_m::interrupt::free(|cs| {
//         // rprintln!("End");
//         if let Some(ref mut timer) = TIMER.borrow(cs).borrow_mut().deref_mut() {
//             timer.cancel();
//         }
//     });
// }
