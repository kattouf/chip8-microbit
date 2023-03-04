#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::rtt_init_print;
// use panic_halt as _;

mod common;
mod cpu;
mod peripheral;

use cpu::CPU;
use peripheral::Peripheral;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = microbit::Board::take().unwrap();
    let peripheral = Peripheral::new(board);
    let mut cpu = CPU::new(true, peripheral);
    cpu.load_data();
    cpu.run();
}
