#![no_main]
#![no_std]
#![allow(dead_code)]
#![allow(unused_imports)]

use core::{cell::RefCell, ops::DerefMut};

use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln, rprint};
use panic_rtt_target as _;
use embedded_hal::{timer::*, digital::v2::{InputPin, StatefulOutputPin}};
// use panic_halt as _;

use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text}, primitives::{PrimitiveStyleBuilder, RoundedRectangle, Rectangle},
};
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

use microbit::{
    hal::{twim, timer::Instance, Timer, gpio::{PullUp, p0::*, p1::*, Input, Output, OpenDrain, p1::{P1_00, P1_03}, OpenDrainConfig, Level}},
    pac::{twim0::frequency::FREQUENCY_A, interrupt},
};

use keypad::{keypad_new, keypad_struct};
use void::Void;

use embedded_hal::{blocking::delay::DelayMs, digital::v2::OutputPin};

keypad_struct!{
    pub struct HexKeypad<Error = Void> {
        rows: (
            P0_17<Input<PullUp>>,
            P0_04<Input<PullUp>>,
            P0_09<Input<PullUp>>,
            P0_03<Input<PullUp>>,
        ),
        columns: (
            P0_10<Output<OpenDrain>>,
            P0_01<Output<OpenDrain>>,
            P0_13<Output<OpenDrain>>,
            P1_02<Output<OpenDrain>>,
        ),
    }
}

mod cpu;
use cpu::CPU;

static TIMER: Mutex<RefCell<Option<Timer<microbit::pac::TIMER0>>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    rtt_init_print!();

    // display_demo();
    // timer_demo();
    keypad_demo();

    loop {
        // cortex_m::asm::nop();
    }
}

fn keypad_demo() {
    let board = microbit::Board::take().unwrap();

    let pins = board.pins;
    let mut timer = Timer::new(board.TIMER0);

    let keypad = keypad_new!(HexKeypad {
        rows: (
            pins.p0_17.into_pullup_input(),
            pins.p0_04.into_pullup_input(),
            pins.p0_09.into_pullup_input(),
            pins.p0_03.into_pullup_input(),
        ),
        columns: (
            pins.p0_10.into_open_drain_output(OpenDrainConfig::HighDrive0Disconnect1, Level::Low),
            pins.p0_01.into_open_drain_output(OpenDrainConfig::HighDrive0Disconnect1, Level::Low),
            pins.p0_13.into_open_drain_output(OpenDrainConfig::HighDrive0Disconnect1, Level::Low),
            pins.p1_02.into_open_drain_output(OpenDrainConfig::HighDrive0Disconnect1, Level::Low),
        ),
    });

    let keys = keypad.decompose();

    loop {
        for (row_index, row) in keys.iter().enumerate() {
            rprint!("row {}: ", row_index);
            for key in row.iter() {
                let is_pressed = if key.is_low().unwrap() { 1 } else { 0 };
                rprint!(" {} ", is_pressed);
            }
            rprintln!();
        }

        timer.delay_ms(1000_u16);
    }

    // Give up ownership of the row and column pins.
    // let ((_r0, _r1, _r2, _r3), (_c0, _c1, _c2, _c3)) = keypad.release();
}

fn timer_demo() {
    let board = microbit::Board::take().unwrap();
    let mut timer = Timer::one_shot(board.TIMER0);
    timer.enable_interrupt();
    unsafe {
        microbit::pac::NVIC::unmask(microbit::pac::Interrupt::TIMER0);
    }

    rprintln!("Start");
    timer.start(1_000_000_u32);

    cortex_m::interrupt::free(move |cs| {
        *TIMER.borrow(cs).borrow_mut() = Some(timer);
    });
}

#[interrupt]
fn TIMER0() {
    cortex_m::interrupt::free(|cs| {
        rprintln!("End");
        if let Some(ref mut timer) = TIMER.borrow(cs).borrow_mut().deref_mut() {
            timer.cancel();
        }
    });
}

fn cpu_test() {
    let mut cpu = CPU::new();

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    let mem = &mut cpu.memory;
    mem[0x000] = 0x21; mem[0x001] = 0x00;
    mem[0x002] = 0x21; mem[0x003] = 0x00;
    mem[0x004] = 0x00; mem[0x005] = 0x00;

    mem[0x100] = 0x80; mem[0x101] = 0x14;
    mem[0x102] = 0x80; mem[0x103] = 0x14;
    mem[0x104] = 0x00; mem[0x105] = 0xEE;

    cpu.run();

    assert_eq!(cpu.registers[0], 45);

    rprintln!("Regiester '0' value: {}", cpu.registers[0]);
}

fn display_demo() {
    let board = microbit::Board::take().unwrap();
    let i2c = { twim::Twim::new(board.TWIM0, board.i2c_external.into(), FREQUENCY_A::K100) };

    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().unwrap();

    let rectangle_style = PrimitiveStyleBuilder::new()
        .stroke_color(BinaryColor::On)
        .stroke_width(3)
        .fill_color(BinaryColor::Off)
        .build();

    RoundedRectangle::with_equal_corners(
        Rectangle::new(Point::new(0, 20), Size::new(128, 44)),
        Size::new(10, 10),
    )
        .into_styled(rectangle_style)
        .draw(&mut display)
        .unwrap();


    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();

    Text::with_baseline("Hello world!", Point::new(16, 0), text_style, Baseline::Top)
        .draw(&mut display)
        .unwrap();

    Text::with_baseline("Hello Rust!", Point::new(16, 30), text_style, Baseline::Top)
        .draw(&mut display)
        .unwrap();


    display.flush().unwrap();
}
