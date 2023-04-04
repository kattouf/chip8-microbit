use core::cell::RefCell;
use core::ops::DerefMut;
use cortex_m::interrupt::Mutex;
use embedded_hal::timer::{Cancel, CountDown};
use microbit::{
    hal::{
        gpio::{Output, Pin, PushPull},
        pwm::{self, Seq, Pwm},
        time::Hertz,
        Timer as HalTimer,
    },
    pac::{interrupt, Interrupt, NVIC, PWM0, TIMER1},
};

const FREQUENCY_HZ: u32 = 60;
const HARDWARE_FREQUENCY_HZ: u32 = 1_000_000;

static TIMER: Mutex<RefCell<Option<HalTimer<TIMER1>>>> = Mutex::new(RefCell::new(None));
static SPEAKER: Mutex<RefCell<Option<Pwm<PWM0>>>> = Mutex::new(RefCell::new(None));

pub struct SoundTimer {}

impl SoundTimer {
    pub fn new(
        hal_timer: TIMER1,
        pwm: PWM0,
        speaker_pin: Pin<Output<PushPull>>,
    ) -> Option<SoundTimer> {
        let mut initialized = false;
        cortex_m::interrupt::free(|cs| {
            if let (Some(_), Some(_)) = (
                TIMER.borrow(cs).borrow().as_ref(),
                SPEAKER.borrow(cs).borrow().as_ref(),
            ) {
                initialized = true;
            }
        });
        if initialized {
            return None;
        }

        let mut hal_timer = HalTimer::one_shot(hal_timer);
        hal_timer.enable_interrupt();
        unsafe {
            NVIC::unmask(Interrupt::TIMER1);
        }

        let speaker = Pwm::new(pwm);
        speaker
            .set_output_pin(pwm::Channel::C0, speaker_pin)
            .set_period(Hertz(440u32))
            .set_prescaler(pwm::Prescaler::Div4)
            .set_duty_on_common(speaker.max_duty() / 2);
        speaker.stop();

        cortex_m::interrupt::free(|cs| {
            TIMER.borrow(cs).replace(Some(hal_timer));
            SPEAKER.borrow(cs).replace(Some(speaker));
        });

        Some(SoundTimer {})
    }

    pub fn start(&mut self, value: u8) {
        cortex_m::interrupt::free(|cs| {
            if let (Some(ref mut timer), Some(speaker)) = (
                TIMER.borrow(cs).borrow_mut().deref_mut(),
                SPEAKER.borrow(cs).borrow().as_ref(),
            ) {
                timer.start(HARDWARE_FREQUENCY_HZ * value as u32 / FREQUENCY_HZ);
                speaker.start_seq(Seq::Seq0);
            }
        });
    }
}

#[interrupt]
fn TIMER1() {
    cortex_m::interrupt::free(|cs| {
        if let (Some(ref mut timer), Some(speaker)) = (
            TIMER.borrow(cs).borrow_mut().deref_mut(),
            SPEAKER.borrow(cs).borrow().as_ref(),
        ) {
            speaker.stop();
            timer.cancel().unwrap();
        }
    });
}
