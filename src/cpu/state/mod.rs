use std::fmt;

use crate::{
    concat_u8,
    cpu::state::{abstracts::Abstracts, buses::Buses, registers::Registers},
    emu::ines::Cartridge,
    split_u16,
};

pub mod abstracts;
pub mod buses;
pub mod memory;
pub mod registers;

pub const MAX_MEMORY_ADDRESS: u16 = 0xFFFF;
pub const PROGRAM_START_ADDRESS: u16 = 0xC000;
pub const STACK_PAGE_HIGH_ADDRESS: u8 = 0x01;

pub const MEMORY_SIZE: usize = MAX_MEMORY_ADDRESS as usize + 1;

pub type HalfCycle = fn(&mut State);
pub type Cycle = [HalfCycle; 2];

pub struct State {
    pub abstracts: Abstracts,
    pub registers: Registers,
    pub buses: Buses,
}

impl State {
    pub fn new(cartrige: Cartridge) -> Self {
        State {
            abstracts: Abstracts::default(),
            registers: Registers::default(),
            buses: Buses::new(cartrige),
        }
    }

    pub fn mem_peek(&self, address: (u8, u8)) -> u8 {
        self.buses.peek(address)
    }

    pub fn mem_read(&mut self, address: (u8, u8)) -> u8 {
        self.buses.read(address)
    }

    pub fn mem_write(&mut self, address: (u8, u8), data: u8) {
        self.buses.write(address, data);
    }

    pub fn increment_pc(&mut self) {
        let address = concat_u8!(self.registers.pc.0, self.registers.pc.1);
        self.registers.pc = split_u16!(address.wrapping_add(1));
    }

    pub fn push_stack(&mut self) {
        self.registers.sp = self.registers.sp.wrapping_sub(1);
    }

    pub fn pop_stack(&mut self) {
        self.registers.sp = self.registers.sp.wrapping_add(1);
    }

    pub fn get_negative_flag(&self) -> bool {
        (self.registers.psr & 0b10000000) != 0
    }

    pub fn set_negative_flag(&mut self, flag: bool) {
        let new_status = if flag {
            self.registers.psr | 0b10000000
        } else {
            self.registers.psr & 0b01111111
        };

        self.registers.psr = new_status;
    }

    pub fn get_overflow_flag(&self) -> bool {
        (self.registers.psr & 0b01000000) != 0
    }

    pub fn set_overflow_flag(&mut self, flag: bool) {
        let new_status = if flag {
            self.registers.psr | 0b01000000
        } else {
            self.registers.psr & 0b10111111
        };

        self.registers.psr = new_status;
    }

    pub fn get_break_flag(&self) -> bool {
        (self.registers.psr & 0b00010000) != 0
    }

    pub fn set_break_flag(&mut self, flag: bool) {
        let new_status = if flag {
            self.registers.psr | 0b00010000
        } else {
            self.registers.psr & 0b11101111
        };

        self.registers.psr = new_status;
    }

    pub fn get_decimal_mode_flag(&self) -> bool {
        (self.registers.psr & 0b00001000) != 0
    }

    pub fn set_decimal_mode_flag(&mut self, flag: bool) {
        let new_status = if flag {
            self.registers.psr | 0b00001000
        } else {
            self.registers.psr & 0b11110111
        };

        self.registers.psr = new_status;
    }

    pub fn get_interrupt_disable_flag(&self) -> bool {
        (self.registers.psr & 0b00000100) != 0
    }

    pub fn set_interrupt_disable_flag(&mut self, flag: bool) {
        let new_status = if flag {
            self.registers.psr | 0b00000100
        } else {
            self.registers.psr & 0b11111011
        };

        self.registers.psr = new_status;
    }

    pub fn get_zero_flag(&self) -> bool {
        (self.registers.psr & 0b00000010) != 0
    }

    pub fn set_zero_flag(&mut self, flag: bool) {
        let new_status = if flag {
            self.registers.psr | 0b00000010
        } else {
            self.registers.psr & 0b11111101
        };

        self.registers.psr = new_status;
    }

    pub fn get_carry_flag(&self) -> bool {
        (self.registers.psr & 0b00000001) != 0
    }

    pub fn set_carry_flag(&mut self, flag: bool) {
        let new_status = if flag {
            self.registers.psr | 0b00000001
        } else {
            self.registers.psr & 0b11111110
        };

        self.registers.psr = new_status;
    }
}

// @TODO: Fix the below functions.

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (pch, pcl) = self.registers.pc;

        let pc0 = concat_u8!(self.registers.pc.0, self.registers.pc.1);
        let pc1 = pc0.wrapping_add(1);
        let pc2 = pc0.wrapping_add(2);
        let pc_mem0 = self.mem_peek(split_u16!(pc0));
        let pc_mem1 = self.mem_peek(split_u16!(pc1));
        let pc_mem2 = self.mem_peek(split_u16!(pc2));

        let (addr_bus_high, addr_bus_low) = self.buses.addr;
        let ab0 = concat_u8!(addr_bus_high, addr_bus_low);
        let ab1 = ab0.wrapping_add(1);
        let ab2 = ab0.wrapping_add(2);
        let ab_mem0 = self.mem_peek(split_u16!(ab0));
        let ab_mem1 = self.mem_peek(split_u16!(ab1));
        let ab_mem2 = self.mem_peek(split_u16!(ab2));

        let data_bus = self.buses.data;

        let ir = self.registers.ir;
        let accumulator = self.registers.a;
        let x_index = self.registers.x_index;
        let y_index = self.registers.y_index;
        let psr = self.registers.psr;
        let sp = self.registers.sp;
        let cycle_count = self.abstracts.half_cycle_count / 2;

        let sp0 = concat_u8!(0x10, self.registers.sp);
        let sp1 = sp0.wrapping_add(1);
        let sp2 = sp0.wrapping_add(2);
        let sp_mem0 = self.mem_peek(split_u16!(sp0));
        let sp_mem1 = self.mem_peek(split_u16!(sp1));
        let sp_mem2 = self.mem_peek(split_u16!(sp2));

        write!(
            f,
            "{pch:02X}{pcl:02X} [{pc_mem0:02X} {pc_mem1:02X} {pc_mem2:02X}] \
            ADDR_BUS: {addr_bus_high:02X}{addr_bus_low:02X} \
            [{ab_mem0:02X} {ab_mem1:02X} {ab_mem2:02X}] \
            DATA_BUS: {data_bus:02X} \
            IR:{ir:02X} A:{accumulator:02X} X:{x_index:02X} Y:{y_index:02X} \
            P:{psr:02X} SP:{sp:02X} [{sp_mem0:02X} {sp_mem1:02X} {sp_mem2:02X}] \
            CYC:{cycle_count:}"
        )
    }
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (pch, pcl) = self.registers.pc;

        let pc0 = concat_u8!(self.registers.pc.0, self.registers.pc.1);
        let pc1 = pc0.wrapping_add(1);
        let pc2 = pc0.wrapping_add(2);
        let pc_mem0 = self.mem_peek(split_u16!(pc0));
        let pc_mem1 = self.mem_peek(split_u16!(pc1));
        let pc_mem2 = self.mem_peek(split_u16!(pc2));

        let accumulator = self.registers.a;
        let x_index = self.registers.x_index;
        let y_index = self.registers.y_index;
        let psr = self.registers.psr;
        let sp = self.registers.sp;
        let cycle_count = self.abstracts.half_cycle_count / 2;

        write!(
            f,
            "{pch:02X}{pcl:02X}  {pc_mem0:02X} {pc_mem1:02X} {pc_mem2:02X}  \
            \t\t\t\t\tA:{accumulator:02X} X:{x_index:02X} \
            Y:{y_index:02X} P:{psr:02X} SP:{sp:02X} CYC:{cycle_count:}"
        )
    }
}
