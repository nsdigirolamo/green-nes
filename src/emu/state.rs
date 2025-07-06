use std::collections::VecDeque;

use crate::{concat_u8, split_u16};

pub const MAX_MEMORY_ADDRESS: u16 = 65535;
pub const MAX_STACK_ADDRESS: u16 = 0x00FF;
pub const PROGRAM_START_ADDRESS: (u8, u8) = (0xC0, 0x00);

pub const MEMORY_LENGTH: usize = MAX_MEMORY_ADDRESS as usize + 1;

pub type HalfCycle = fn(&mut State);
pub type Cycle = [HalfCycle; 2];

#[derive(Default, Debug)]
pub struct Registers {
    pub accumulator: u8,
    pub x_index: u8,
    pub y_index: u8,
    pub program_counter: (u8, u8), // (high pc byte, low pc byte)
    pub stack_pointer: u8,
    pub processor_status: u8,
    pub instruction: u8,
}

#[derive(Debug)]
pub struct State {
    pub cycle_queue: VecDeque<Cycle>,
    memory: [u8; MEMORY_LENGTH],
    pub registers: Registers,
    pub address_bus: (u8, u8), // external effective address (high address byte, low address byte)
    pub data_bus: u8,          // external data
    pub address_low: u8,       // internal address low byte
    pub address_high: u8,      // internal address high byte
    pub crossed_page: bool,
}

impl Default for State {
    fn default() -> Self {
        State {
            cycle_queue: VecDeque::default(),
            memory: [0u8; MEMORY_LENGTH],
            registers: Registers::default(),
            address_bus: (0x00, 0x00),
            data_bus: 0x00,
            address_low: 0x00,
            address_high: 0x00,
            crossed_page: false,
        }
    }
}

impl State {
    pub fn read_from_memory(&self, address: (u8, u8)) -> u8 {
        self.memory[concat_u8!(address.0, address.1) as usize]
    }

    pub fn write_to_memory(&mut self, address: (u8, u8), data: u8) {
        self.memory[concat_u8!(address.0, address.1) as usize] = data;
    }

    pub fn increment_pc_address(&mut self) {
        let address = concat_u8!(
            self.registers.program_counter.0,
            self.registers.program_counter.1
        );
        let address = address.wrapping_add(1);
        self.registers.program_counter = split_u16!(address);
    }

    pub fn get_negative_flag(&self) -> bool {
        (self.registers.processor_status & 0b10000000) != 0
    }

    pub fn set_negative_flag(&mut self, flag: bool) {
        let new_status = if flag {
            self.registers.processor_status | 0b10000000
        } else {
            self.registers.processor_status & 0b01111111
        };

        self.registers.processor_status = new_status;
    }

    pub fn get_overflow_flag(&self) -> bool {
        (self.registers.processor_status & 0b01000000) != 0
    }

    pub fn set_overflow_flag(&mut self, flag: bool) {
        let new_status = if flag {
            self.registers.processor_status | 0b01000000
        } else {
            self.registers.processor_status & 0b10111111
        };

        self.registers.processor_status = new_status;
    }

    pub fn get_break_command_flag(&self) -> bool {
        (self.registers.processor_status & 0b00010000) != 0
    }

    pub fn set_break_command_flag(&mut self, flag: bool) {
        let new_status = if flag {
            self.registers.processor_status | 0b00010000
        } else {
            self.registers.processor_status & 0b11101111
        };

        self.registers.processor_status = new_status;
    }

    pub fn get_decimal_mode_flag(&self) -> bool {
        (self.registers.processor_status & 0b00001000) != 0
    }

    pub fn set_decimal_mode_command_flag(&mut self, flag: bool) {
        let new_status = if flag {
            self.registers.processor_status | 0b00001000
        } else {
            self.registers.processor_status & 0b11110111
        };

        self.registers.processor_status = new_status;
    }

    pub fn get_interrupt_disable_flag(&self) -> bool {
        (self.registers.processor_status & 0b00000100) != 0
    }

    pub fn set_interrupt_disable_command_flag(&mut self, flag: bool) {
        let new_status = if flag {
            self.registers.processor_status | 0b00000100
        } else {
            self.registers.processor_status & 0b11111011
        };

        self.registers.processor_status = new_status;
    }

    pub fn get_zero_flag(&self) -> bool {
        (self.registers.processor_status & 0b00000010) != 0
    }

    pub fn set_zero_flag(&mut self, flag: bool) {
        let new_status = if flag {
            self.registers.processor_status | 0b00000010
        } else {
            self.registers.processor_status & 0b11111101
        };

        self.registers.processor_status = new_status;
    }

    pub fn get_carry_flag(&self) -> bool {
        (self.registers.processor_status & 0b00000001) != 0
    }

    pub fn set_carry_flag(&mut self, flag: bool) {
        let new_status = if flag {
            self.registers.processor_status | 0b00000001
        } else {
            self.registers.processor_status & 0b11111110
        };

        self.registers.processor_status = new_status;
    }
}
