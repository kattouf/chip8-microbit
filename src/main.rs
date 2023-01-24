#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::rtt_init_print;
use panic_rtt_target as _;
// use panic_halt as _;

mod cpu;
mod peripheral;
mod test_roms;

use cpu::CPU;
use peripheral::Peripheral;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = microbit::Board::take().unwrap();
    let peripheral = Peripheral::new(board);
    let mut cpu = CPU::new(false, peripheral);
    cpu.load_data(&test_roms::IBM_LOGO);
    cpu.run();
}
