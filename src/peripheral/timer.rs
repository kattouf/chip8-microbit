pub struct Timer();

impl Timer {
    pub fn start(&self, value: u8) {
    }

    pub fn current_value(&self) -> u8 {
        0
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
