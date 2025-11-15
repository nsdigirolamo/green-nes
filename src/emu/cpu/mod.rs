pub mod cycles;
pub mod half_cycles;
pub mod instructions;

use std::collections::VecDeque;

use crate::{
    concat_u8,
    emu::{
        buses::Buses as ExternalBuses,
        cpu::{
            cycles::{Cycle, FETCH_INSTRUCTION, get_cycles},
            half_cycles::{
                get_high_irq_vector, get_high_nmi_vector, get_low_irq_vector, get_low_nmi_vector,
                get_pc, push_stack, read_data, read_high_effective_address_byte,
                read_low_effective_address_byte, write_pc_high, write_pc_low, write_status,
            },
        },
    },
    split_u16,
};

const PC_DEFAULT: (u8, u8) = (0xC0, 0x00);
const SP_DEFAULT: u8 = 0xFD;
const PSR_DEFAULT: u8 = 0x24;

#[derive(Default)]
pub struct Registers {
    pub a: u8,        // Accumulator
    pub x_index: u8,  // X Index Register
    pub y_index: u8,  // Y Index Register
    pub pc: (u8, u8), // Program Counter (PCH, PCL)
    pub sp: u8,       // Stack Pointer
    pub psr: u8,      // Processor Status Register
    pub ir: u8,       // Instruction Register
}

#[derive(Default)]
pub struct Buses {
    pub base_addr: (u8, u8),      // Base Address Buses (BAH, BAL)
    pub effective_addr: (u8, u8), // Effective Address Buses (ADH, ADL)
    pub indirect_addr: (u8, u8),  // Indirect Address Buses (IAH, IAL)
}

pub struct CPU {
    pub cycle_queue: VecDeque<Cycle>,
    pub half_cycle_count: u64,
    pub is_halted: bool,
    pub registers: Registers,
    pub buses: Buses, // Internal CPU Busesess
    crossed_page: bool,
    pub irq_occurred: bool,
    pub nmi_occurred: bool,
    pub nmi_flip_flop: bool, // Stores the previous state of NMI on the bus.
}

impl Default for CPU {
    fn default() -> Self {
        Self {
            cycle_queue: VecDeque::default(),
            half_cycle_count: 14,
            is_halted: false,
            registers: Registers {
                a: 0,
                x_index: 0,
                y_index: 0,
                pc: PC_DEFAULT,
                sp: SP_DEFAULT,
                psr: PSR_DEFAULT,
                ir: 0,
            },
            buses: Buses::default(),
            crossed_page: false,
            irq_occurred: false,
            nmi_occurred: false,
            nmi_flip_flop: false,
        }
    }
}

impl CPU {
    pub fn set_irq(&mut self, buses: &ExternalBuses) {
        self.irq_occurred = !buses.get_irq()
    }

    pub fn set_nmi(&mut self, buses: &ExternalBuses) {
        if self.nmi_flip_flop && !buses.get_nmi() {
            self.nmi_occurred = true;
        }
    }

    pub fn do_cycle(&mut self, buses: &mut ExternalBuses, cycle: Cycle) {
        let [phase1, phase2] = cycle;

        phase1(self, buses);
        phase2(self, buses);

        self.set_irq(buses);
        self.set_nmi(buses);

        self.half_cycle_count += 2;
    }

    pub fn handle_irq(&mut self) {
        let interrupt = [
            FETCH_INSTRUCTION,
            [get_pc, read_data],
            [push_stack, write_pc_high],
            [push_stack, write_pc_low],
            [push_stack, write_status],
            [get_low_irq_vector, read_high_effective_address_byte],
            [get_high_irq_vector, read_low_effective_address_byte],
        ];

        self.cycle_queue.extend(interrupt.iter())
    }

    pub fn handle_nmi(&mut self) {
        let interrupt = [
            FETCH_INSTRUCTION,
            [get_pc, read_data],
            [push_stack, write_pc_high],
            [push_stack, write_pc_low],
            [push_stack, write_status],
            [get_low_nmi_vector, read_high_effective_address_byte],
            [get_high_nmi_vector, read_low_effective_address_byte],
        ];

        self.cycle_queue.extend(interrupt.iter());
    }

    pub fn tick(&mut self, buses: &mut ExternalBuses) {
        let cycle = self.cycle_queue.pop_front();
        match cycle {
            Some(cycle) => self.do_cycle(buses, cycle),
            None => {
                if self.nmi_occurred {
                    self.handle_nmi();
                    return;
                }

                if self.irq_occurred {
                    self.handle_irq();
                    return;
                }

                self.do_cycle(buses, FETCH_INSTRUCTION);
                let new_cycles = get_cycles(self.registers.ir);
                self.cycle_queue.extend(new_cycles.iter());
            }
        }
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
