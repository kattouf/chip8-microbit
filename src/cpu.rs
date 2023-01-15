#![allow(dead_code)]

use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

const FONT_SPRITES_LOCATION: usize = 0x50;
const FONT_SPRITES: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct CPU {
    pub registers: [u8; 16],
    pub position_in_memory: usize,
    pub memory: [u8; 0x1000], // 4kb
    stack: [u16; 16],
    stack_pointer: usize,
    index_register: u16,
    modern_mode: bool,
    peripheral: Peripheral,
}

struct Timer();
impl Timer {
    fn start(&self, value: u8) {
        unimplemented!();
    }

    fn current_value(&self) -> u8 {
        unimplemented!();
    }
}

struct Display();
struct Keypad();

pub struct Peripheral {
    delay_timer: Timer,
    sound_timer: Timer,
    display: Display,
    keypad: Keypad,
}
impl Peripheral {
    pub fn new() -> Self {
        Peripheral { delay_timer: Timer {}, sound_timer: Timer {}, display: Display {}, keypad: Keypad {} }
    }
}

impl CPU {
    pub fn new(peripheral: Peripheral) -> Self {
        CPU {
            registers: [0; 16],
            position_in_memory: 0x200,
            memory: [0; 4096],
            stack: [0; 16],
            stack_pointer: 0,
            index_register: 0,
            modern_mode: false,
            peripheral,
        }
    }

    pub fn run(&mut self) {
        for (offset, byte) in FONT_SPRITES.iter().enumerate() {
            self.memory[FONT_SPRITES_LOCATION + offset] = *byte;
        }

        loop {
            let opcode = self.read_opcode();
            self.position_in_memory += 2;

            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let n = ((opcode & 0x000F) >> 0) as u8;

            let nn = (opcode & 0x00FF) as u8;
            let nnn = opcode & 0xFFF;

            match (c, x, y, n) {
                (0, 0, 0, 0) => { todo!(); },
                (0, 0, 0xE, 0) => { todo!("00E0"); },
                (0, 0, 0xE, 0xE) => self.ret(),
                (0x1, _, _, _) => self.jump(nnn),
                (0x2, _, _, _) => self.call(nnn),
                (0x3, _, _, _) => self.skip_next_if_equal_xnn(x, nn),
                (0x4, _, _, _) => self.skip_next_if_not_equal_xnn(x, nn),
                (0x5, _, _, 0) => self.skip_next_if_equal_xy(x, y),
                (0x6, _, _, _) => self.set_xnn(x, nn),
                (0x7, _, _, _) => self.add_xnn(x, nn),
                (0x8, _, _, 0) => self.set_xy(x, y),
                (0x8, _, _, 0x1) => self.or_xy(x, y),
                (0x8, _, _, 0x2) => self.and_xy(x, y),
                (0x8, _, _, 0x3) => self.xor_xy(x, y),
                (0x8, _, _, 0x4) => self.add_xy(x, y),
                (0x8, _, _, 0x5) => self.sub_xy(x, y),
                (0x8, _, _, 0x6) => self.shift_y_right(x, y),
                (0x8, _, _, 0x7) => self.sub_yx(x, y),
                (0x8, _, _, 0xE) => self.shift_y_left(x, y),
                (0x9, _, _, 0) => self.skip_next_if_not_equal_xy(x, y),
                (0xA, _, _, _) => self.set_innn(nnn),
                (0xB, _, _, _) => self.jump_with_offset(x, nnn),
                (0xC, _, _, _) => self.random(x, nn),
                (0xD, _, _, _) => todo!("DXYN"),
                (0xE, _, 0x9, 0xE) => todo!("EX9E"),
                (0xE, _, 0xA, 0x1) => todo!("EXA1"),
                (0xF, _, 0x0, 0x7) => self.set_x_delay_timer(x),
                (0xF, _, 0x0, 0xA) => todo!("FX0A"),
                (0xF, _, 0x1, 0x5) => self.set_delay_timer_x(x),
                (0xF, _, 0x1, 0x8) => self.set_sound_timer_x(x),
                (0xF, _, 0x1, 0xE) => self.add_ix(x),
                (0xF, _, 0x2, 0x9) => self.set_i_to_address_of_hex_sprite_stored_in_x(x),
                (0xF, _, 0x3, 0x3) => self.bcd_x(x),
                (0xF, _, 0x5, 0x5) => self.save_regs_to_memory(x),
                (0xF, _, 0x6, 0x5) => self.load_regs_from_memory(x),
                _ => panic!("undefined opcode {:04x}", opcode),
            }
        }
    }

    fn set_x_delay_timer(&mut self, x: u8) {
        let timer_value = self.peripheral.delay_timer.current_value();
        self.registers[x as usize] = timer_value;
    }

    fn set_delay_timer_x(&mut self, x: u8) {
        let x_val = self.registers[x as usize];
        self.peripheral.delay_timer.start(x_val);
    }

    fn set_sound_timer_x(&mut self, x: u8) {
        let x_val = self.registers[x as usize];
        self.peripheral.sound_timer.start(x_val);
    }

    fn set_i_to_address_of_hex_sprite_stored_in_x(&mut self, x: u8) {
        self.index_register =
            FONT_SPRITES_LOCATION as u16 + (self.registers[x as usize] & 0x0F) as u16;
    }

    fn bcd_x(&mut self, x: u8) {
        let x_val = self.registers[x as usize];
        self.memory[self.index_register as usize] = (x_val / 100) % 10;
        self.memory[self.index_register as usize + 1] = (x_val / 10) % 10;
        self.memory[self.index_register as usize + 2] = (x_val / 1) % 10;
    }

    fn save_regs_to_memory(&mut self, x: u8) {
        if self.modern_mode {
            for offset in 0..=x {
                self.memory[self.index_register as usize + offset as usize] =
                    self.registers[offset as usize];
            }
        } else {
            for offset in 0..=x {
                self.memory[self.index_register as usize] = self.registers[offset as usize];
                self.index_register += 1;
            }
        }
    }

    fn load_regs_from_memory(&mut self, x: u8) {
        if self.modern_mode {
            for offset in 0..=x {
                self.registers[offset as usize] =
                    self.memory[self.index_register as usize + offset as usize];
            }
        } else {
            for offset in 0..=x {
                self.registers[offset as usize] = self.memory[self.index_register as usize];
                self.index_register += 1;
            }
        }
    }

    fn add_ix(&mut self, x: u8) {
        let x_val = self.registers[x as usize] as u16;

        let (val, overflow) = self.index_register.overflowing_add(x_val);

        self.index_register = val;
        self.registers[0xF] = if overflow { 1 } else { 0 };
    }

    fn random(&mut self, x: u8, nn: u8) {
        let mut small_rng = SmallRng::seed_from_u64(1337);
        self.registers[x as usize] = small_rng.gen::<u8>() & nn;
    }

    fn set_innn(&mut self, nnn: u16) {
        self.index_register = nnn;
    }

    fn shift_y_left(&mut self, x: u8, y: u8) {
        let val = if self.modern_mode {
            self.registers[x as usize]
        } else {
            self.registers[y as usize]
        };
        self.registers[0xF] = (val & 0b1000_0000) >> 7;
        self.registers[x as usize] = val << 1;
    }

    fn shift_y_right(&mut self, x: u8, y: u8) {
        let val = if self.modern_mode {
            self.registers[x as usize]
        } else {
            self.registers[y as usize]
        };
        self.registers[0xF] = val & 0b0000_0001;
        self.registers[x as usize] = val >> 1;
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let x_val = self.registers[x as usize];
        let y_val = self.registers[y as usize];

        let (val, overflow) = x_val.overflowing_add(y_val);

        self.registers[x as usize] = val;
        self.registers[0xF] = if overflow { 1 } else { 0 };
    }

    fn sub_xy(&mut self, x: u8, y: u8) {
        let x_val = self.registers[x as usize];
        let y_val = self.registers[y as usize];

        let (val, underflow) = x_val.overflowing_sub(y_val);

        self.registers[x as usize] = val;
        self.registers[0xF] = if underflow { 0 } else { 1 };
    }

    fn sub_yx(&mut self, x: u8, y: u8) {
        let x_val = self.registers[x as usize];
        let y_val = self.registers[y as usize];

        let (val, underflow) = y_val.overflowing_sub(x_val);

        self.registers[x as usize] = val;
        self.registers[0xF] = if underflow { 0 } else { 1 };
    }

    fn or_xy(&mut self, x: u8, y: u8) {
        self.registers[x as usize] |= self.registers[y as usize];
    }

    fn and_xy(&mut self, x: u8, y: u8) {
        self.registers[x as usize] &= self.registers[y as usize];
    }

    fn xor_xy(&mut self, x: u8, y: u8) {
        self.registers[x as usize] ^= self.registers[y as usize];
    }

    fn set_xy(&mut self, x: u8, y: u8) {
        self.registers[x as usize] = self.registers[y as usize];
    }

    fn set_xnn(&mut self, x: u8, nn: u8) {
        self.registers[x as usize] = nn;
    }

    fn add_xnn(&mut self, x: u8, nn: u8) {
        self.registers[x as usize] += nn;
    }

    fn skip_next_if_equal_xnn(&mut self, x: u8, nn: u8) {
        let x_val = self.registers[x as usize];

        if x_val == nn {
            self.position_in_memory += 2;
        }
    }

    fn skip_next_if_not_equal_xnn(&mut self, x: u8, nn: u8) {
        let x_val = self.registers[x as usize];

        if x_val != nn {
            self.position_in_memory += 2;
        }
    }

    fn skip_next_if_equal_xy(&mut self, x: u8, y: u8) {
        let x_val = self.registers[x as usize];
        let y_val = self.registers[y as usize];

        if x_val == y_val {
            self.position_in_memory += 2;
        }
    }

    fn skip_next_if_not_equal_xy(&mut self, x: u8, y: u8) {
        let x_val = self.registers[x as usize];
        let y_val = self.registers[y as usize];

        if x_val != y_val {
            self.position_in_memory += 2;
        }
    }

    fn jump(&mut self, addr: u16) {
        self.position_in_memory = addr as usize;
    }

    fn jump_with_offset(&mut self, x: u8, nnn: u16) {
        if self.modern_mode {
            self.position_in_memory = nnn as usize + self.registers[x as usize] as usize;
        } else {
            self.position_in_memory = nnn as usize + self.registers[0] as usize;
        }
    }

    fn call(&mut self, addr: u16) {
        let sp = self.stack_pointer;
        let stack = &mut self.stack;

        if sp > stack.len() {
            panic!("Stack overflow");
        }

        stack[sp] = self.position_in_memory as u16;
        self.stack_pointer += 1;
        self.position_in_memory = addr as usize;
    }

    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Stack underflow");
        }

        self.stack_pointer -= 1;
        let addr = self.stack[self.stack_pointer];
        self.position_in_memory = addr as usize;
    }

    fn read_opcode(&self) -> u16 {
        let p = self.position_in_memory;
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p + 1] as u16;

        op_byte1 << 8 | op_byte2
    }
}
