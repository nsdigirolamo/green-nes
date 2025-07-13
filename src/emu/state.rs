use std::{collections::VecDeque, fmt};

use crate::{concat_u8, split_u16};

pub const MAX_MEMORY_ADDRESS: u16 = 0xFFFF;
pub const PROGRAM_START_ADDRESS: u16 = 0xC000;

pub const MEMORY_LENGTH: usize = MAX_MEMORY_ADDRESS as usize + 1;

pub type HalfCycle = fn(&mut State);
pub type Cycle = [HalfCycle; 2];

pub struct State {
    // Abstracted Fields
    pub cycle_queue: VecDeque<Cycle>,
    pub half_cycle_count: u64,
    pub is_halted: bool,
    memory: [u8; MEMORY_LENGTH],
    pub crossed_page: bool,
    // Registers
    pub accumulator: u8,               // A
    pub x_index_register: u8,          // X
    pub y_index_register: u8,          // Y
    pub program_counter: (u8, u8),     // (PCH, PCL)
    pub stack_pointer: u8,             // SP
    pub processor_status_register: u8, // PSR
    pub instruction_register: u8,      // IR
    // External Buses
    pub address_bus: (u8, u8), // (ABH, ABL)
    pub data_bus: u8,
    // Internal Buses
    pub base_address: (u8, u8),      // (BAH, BAL)
    pub effective_address: (u8, u8), // (ADH, ADL)
    pub indirect_address: (u8, u8),  // (IAH, IAL)
}

impl Default for State {
    fn default() -> Self {
        State {
            cycle_queue: VecDeque::default(),
            half_cycle_count: 0,
            is_halted: false,
            memory: [0u8; MEMORY_LENGTH],
            crossed_page: false,
            accumulator: 0,
            x_index_register: 0,
            y_index_register: 0,
            program_counter: (0, 0),
            stack_pointer: 0xFD,
            processor_status_register: 0b_0010_0100, // NV1B_DIZC
            instruction_register: 0,
            address_bus: (0, 0),
            data_bus: 0,
            base_address: (0, 0),
            effective_address: (0, 0),
            indirect_address: (0, 0),
        }
    }
}

impl State {
    pub fn read_from_memory(&mut self, address: (u8, u8)) -> u8 {
        let data = self.memory[concat_u8!(address.0, address.1) as usize];
        self.data_bus = data;

        data
    }

    pub fn write_to_memory(&mut self, address: (u8, u8), data: u8) {
        self.data_bus = data;
        self.memory[concat_u8!(address.0, address.1) as usize] = data;
    }

    pub fn increment_pc(&mut self) {
        let address = concat_u8!(self.program_counter.0, self.program_counter.1);
        self.program_counter = split_u16!(address.wrapping_add(1));
    }

    pub fn push_stack(&mut self) {
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
    }

    pub fn pop_stack(&mut self) {
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
    }

    pub fn get_negative_flag(&self) -> bool {
        (self.processor_status_register & 0b10000000) != 0
    }

    pub fn set_negative_flag(&mut self, flag: bool) {
        let new_status = if flag {
            self.processor_status_register | 0b10000000
        } else {
            self.processor_status_register & 0b01111111
        };

        self.processor_status_register = new_status;
    }

    pub fn get_overflow_flag(&self) -> bool {
        (self.processor_status_register & 0b01000000) != 0
    }

    pub fn set_overflow_flag(&mut self, flag: bool) {
        let new_status = if flag {
            self.processor_status_register | 0b01000000
        } else {
            self.processor_status_register & 0b10111111
        };

        self.processor_status_register = new_status;
    }

    pub fn get_break_flag(&self) -> bool {
        (self.processor_status_register & 0b00010000) != 0
    }

    pub fn set_break_flag(&mut self, flag: bool) {
        let new_status = if flag {
            self.processor_status_register | 0b00010000
        } else {
            self.processor_status_register & 0b11101111
        };

        self.processor_status_register = new_status;
    }

    pub fn get_decimal_mode_flag(&self) -> bool {
        (self.processor_status_register & 0b00001000) != 0
    }

    pub fn set_decimal_mode_flag(&mut self, flag: bool) {
        let new_status = if flag {
            self.processor_status_register | 0b00001000
        } else {
            self.processor_status_register & 0b11110111
        };

        self.processor_status_register = new_status;
    }

    pub fn get_interrupt_disable_flag(&self) -> bool {
        (self.processor_status_register & 0b00000100) != 0
    }

    pub fn set_interrupt_disable_flag(&mut self, flag: bool) {
        let new_status = if flag {
            self.processor_status_register | 0b00000100
        } else {
            self.processor_status_register & 0b11111011
        };

        self.processor_status_register = new_status;
    }

    pub fn get_zero_flag(&self) -> bool {
        (self.processor_status_register & 0b00000010) != 0
    }

    pub fn set_zero_flag(&mut self, flag: bool) {
        let new_status = if flag {
            self.processor_status_register | 0b00000010
        } else {
            self.processor_status_register & 0b11111101
        };

        self.processor_status_register = new_status;
    }

    pub fn get_carry_flag(&self) -> bool {
        (self.processor_status_register & 0b00000001) != 0
    }

    pub fn set_carry_flag(&mut self, flag: bool) {
        let new_status = if flag {
            self.processor_status_register | 0b00000001
        } else {
            self.processor_status_register & 0b11111110
        };

        self.processor_status_register = new_status;
    }
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (pch, pcl) = self.program_counter;

        let pc0 = concat_u8!(self.program_counter.0, self.program_counter.1);
        let pc1 = pc0.wrapping_add(1);
        let pc2 = pc0.wrapping_add(2);
        let pc_mem0 = self.memory[pc0 as usize];
        let pc_mem1 = self.memory[pc1 as usize];
        let pc_mem2 = self.memory[pc2 as usize];

        let (adh, adl) = self.effective_address;
        let (bah, bal) = self.base_address;
        let (iah, ial) = self.indirect_address;
        let (addr_bus_high, addr_bus_low) = self.address_bus;
        let data_bus = self.data_bus;

        let ir = self.instruction_register;
        let accumulator = self.accumulator;
        let x_index = self.x_index_register;
        let y_index = self.y_index_register;
        let psr = self.processor_status_register;
        let sp = self.stack_pointer;
        let cycle_count = self.half_cycle_count / 2;

        let sp0 = concat_u8!(0x10, self.stack_pointer);
        let sp1 = sp0.wrapping_add(1);
        let sp2 = sp0.wrapping_add(2);
        let sp_mem0 = self.memory[sp0 as usize];
        let sp_mem1 = self.memory[sp1 as usize];
        let sp_mem2 = self.memory[sp2 as usize];

        write!(
            f,
            "{pch:02X}{pcl:02X} [{pc_mem0:02X} {pc_mem1:02X} {pc_mem2:02X}] \
            AD: {adh:02X}{adl:02X} BA: {bah:02X}{bal:02X} IA: {iah:02X}{ial:02X} \
            ADDR_BUS: {addr_bus_high:02X}{addr_bus_low:02X} \
            DATA_BUS: {data_bus:02X} \
            IR:{ir:02X} A:{accumulator:02X} X:{x_index:02X} Y:{y_index:02X} \
            P:{psr:02X} SP:{sp:02X} [{sp_mem0:02X} {sp_mem1:02X} {sp_mem2:02X}] \
            CYC:{cycle_count:5}"
        )
    }
}
