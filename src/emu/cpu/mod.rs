pub mod cycles;
pub mod half_cycles;
pub mod instructions;

use std::collections::VecDeque;

use crate::{
    concat_u8,
    emu::{
        buses::Buses as ExternalBuses,
        cpu::cycles::{Cycle, FETCH_INSTRUCTION, HANDLE_IRQ, HANDLE_NMI, get_cycles},
    },
    split_u16,
};

const PC_DEFAULT: (u8, u8) = (0x80, 0x00); // (0xC0, 0x00);
const SP_DEFAULT: u8 = 0xFD;
const PSR_DEFAULT: u8 = 0b100100;

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
/// Internal CPU buses.
pub struct Buses {
    /// Base (BAH, BAL) address bus.
    pub base_addr: (u8, u8),
    /// Effective (ADH, ADL) address bus.
    pub effective_addr: (u8, u8), //
    /// Indirect (IAH, IAL) address bus.
    pub indirect_addr: (u8, u8),
}

#[derive(Clone)]
pub struct CPU {
    cycle_queue: VecDeque<Cycle>,
    pub half_cycle_count: u64,
    is_halted: bool,
    registers: Registers,
    buses: Buses,
    crossed_page: bool,
    /// The state of the NMI pin on the external buses during the previous
    /// cycle.
    prev_nmi: bool,
    /// Indicates that the NMI handler needs to be invoked.
    nmi_detected: bool,
    /// Indicates that the IRQ handler needs to be invoked.
    irq_detected: bool,
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
            prev_nmi: false,
            nmi_detected: false,
            irq_detected: false,
        }
    }
}

impl CPU {
    pub fn tick(&mut self, buses: &mut ExternalBuses) {
        let cycle = self.cycle_queue.pop_front();

        match cycle {
            Some(cycle) => self.run_cycle(buses, cycle),
            None => {
                self.run_cycle(buses, FETCH_INSTRUCTION);

                if self.nmi_detected {
                    self.cycle_queue.extend(HANDLE_NMI.to_vec());
                    self.nmi_detected = false;
                    return;
                }

                if self.irq_detected {
                    self.cycle_queue.extend(HANDLE_IRQ.to_vec());
                    return;
                }

                self.cycle_queue.extend(get_cycles(self.registers.ir));
            }
        }
    }

    fn run_cycle(&mut self, buses: &mut ExternalBuses, cycle: Cycle) {
        let [phase1, phase2] = cycle;

        phase1(self, buses);
        phase2(self, buses);

        self.irq_detected = buses.get_irq();

        let old_nmi = self.prev_nmi;
        let new_nmi = buses.get_nmi();

        if !old_nmi && new_nmi {
            self.nmi_detected = true;
        }

        self.prev_nmi = new_nmi;

        self.half_cycle_count += 2;
    }

    pub fn reset(&mut self, buses: &mut ExternalBuses) {
        self.registers.a = 0x00;
        self.registers.x_index = 0x00;
        self.registers.sp = SP_DEFAULT;
        self.registers.psr = PSR_DEFAULT;

        let pcl = buses.peek(0xFFFC);
        let pch = buses.peek(0xFFFD);

        println!("Starting At: &{pch:02X}{pcl:02X}");

        self.registers.pc = (pch, pcl);
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
