#![no_main]
#![no_std]
#![feature(alloc_error_handler)]
#![feature(result_flattening)]
#![feature(never_type)]
#![feature(error_in_core)]

use common::{SimpleError, SimpleResult};
use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
// use panic_halt as _;

mod allocator;
mod common;
mod cpu;
mod peripheral;

use allocator::init_heap_allocator;
use cpu::CPU;
use peripheral::Peripheral;

#[entry]
fn main() -> ! {
    init_heap_allocator();
    rtt_init_print!();

    run_interpreter().unwrap_or_else(|err| {
        rprintln!("{}", err);
        loop {}
    });
}

fn run_interpreter() -> SimpleResult<!> {
    let board = microbit::Board::take().ok_or(SimpleError("Board can't initialize"))?;
    let peripheral = Peripheral::new(board)?;
    let mut cpu = CPU::new(true, peripheral);
    cpu.load_data()?;
    cpu.run()?;
}
