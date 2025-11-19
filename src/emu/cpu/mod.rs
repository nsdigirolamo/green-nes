pub mod cycles;
pub mod half_cycles;
pub mod instructions;

use std::collections::VecDeque;

use crate::{
    concat_u8,
    emu::{
        buses::Buses as ExternalBuses,
        cpu::cycles::{Cycle, FETCH_INSTRUCTION, HANDLE_IRQ, HANDLE_NMI, get_cycles, run_cycle},
    },
    split_u16,
};

const PC_DEFAULT: (u8, u8) = (0xC0, 0x00);
const SP_DEFAULT: u8 = 0xFD;
const PSR_DEFAULT: u8 = 0x24;

#[derive(Clone, Copy)]
// Internal CPU Registers
pub struct Registers {
    pub a: u8,        // Accumulator
    pub x_index: u8,  // X Index Register
    pub y_index: u8,  // Y Index Register
    pub pc: (u8, u8), // Program Counter (PCH, PCL)
    pub sp: u8,       // Stack Pointer
    pub psr: u8,      // Processor Status Register
    pub ir: u8,       // Instruction Register
}

impl Default for Registers {
    fn default() -> Self {
        Self {
            a: 0,
            x_index: 0,
            y_index: 0,
            pc: PC_DEFAULT,
            sp: SP_DEFAULT,
            psr: PSR_DEFAULT,
            ir: 0,
        }
    }
}

#[derive(Default, Clone, Copy)]
/// Internal CPU Buses
pub struct Buses {
    pub base_addr: (u8, u8),      // Base Address Buses (BAH, BAL)
    pub effective_addr: (u8, u8), // Effective Address Buses (ADH, ADL)
    pub indirect_addr: (u8, u8),  // Indirect Address Buses (IAH, IAL)
}

#[derive(Clone)]
pub struct CPU {
    // Cycle Controls
    cycle_queue: VecDeque<Cycle>,
    pub half_cycle_count: u64,
    is_halted: bool,
    // Data
    registers: Registers,
    buses: Buses,
    // Misc
    crossed_page: bool,
    pub irq: bool,
    pub nmi: bool,
}

impl Default for CPU {
    fn default() -> Self {
        Self {
            cycle_queue: VecDeque::default(),
            half_cycle_count: 14,
            is_halted: false,
            registers: Registers::default(),
            buses: Buses::default(),
            crossed_page: false,
            irq: false,
            nmi: false,
        }
    }
}

impl CPU {
    pub fn tick(&mut self, buses: &mut ExternalBuses) {
        let cycle = self.cycle_queue.pop_front();
        match cycle {
            Some(cycle) => run_cycle(self, buses, cycle),
            None => {
                run_cycle(self, buses, FETCH_INSTRUCTION);

                let new_cycles = if self.nmi {
                    HANDLE_NMI.to_vec()
                } else if self.irq {
                    HANDLE_IRQ.to_vec()
                } else {
                    get_cycles(self.registers.ir)
                };

                self.cycle_queue.extend(new_cycles.iter());
            }
        }
    }

    pub fn get_cycle_queue(&self) -> VecDeque<Cycle> {
        self.cycle_queue.clone()
    }

    pub fn is_halted(&self) -> bool {
        self.is_halted
    }

    pub fn get_registers(&self) -> Registers {
        self.registers
    }

    pub fn get_buses(&self) -> Buses {
        self.buses
    }

    pub fn crossed_page(&self) -> bool {
        self.crossed_page
    }

    pub fn increment_pc(&mut self) {
        let address = concat_u8!(self.registers.pc.0, self.registers.pc.1);
        self.registers.pc = split_u16!(address.wrapping_add(1));
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
