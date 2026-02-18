use std::collections::VecDeque;

use crate::{
    emu::{
        buses::Buses as ExternalBuses,
        cpu::{
            cycles::{Cycle, HANDLE_IRQ, HANDLE_NMI},
            half_cycles::{get_pc, read_opcode},
            registers::{REGISTERS_AT_POWERON, Registers},
        },
    },
    split_u16,
};

pub mod cycles;
pub mod flags;
pub mod half_cycles;
pub mod instructions;
pub mod registers;

#[derive(Default, Clone, Copy)]
/// Internal CPU buses.
pub struct Buses {
    /// Base (BAH, BAL) address bus.
    pub base_addr: (u8, u8),
    /// Effective (ADH, ADL) address bus.
    pub effective_addr: (u8, u8),
    /// Indirect (IAH, IAL) address bus.
    pub indirect_addr: (u8, u8),
}

#[derive(Default, Clone)]
pub struct CPU {
    cycle_queue: VecDeque<Cycle>,
    half_cycle_count: u64,
    is_halted: bool,
    registers: Registers,
    /// Buses internal to the CPU.
    buses: Buses,
    /// Indicates that a memory page was crossed in a previous cycle.
    crossed_page: bool,
    /// The state of the NMI pin on the external buses during the previous
    /// cycle.
    prev_nmi: bool,
    /// Indicates that the NMI handler needs to be invoked.
    nmi_detected: bool,
    /// Indicates that the IRQ handler needs to be invoked.
    irq_detected: bool,
    /// Indicates that the interrupt disable flag needs to be set, such as
    /// following the PLP instruction.
    interrupt_disabled: Option<bool>,
}

impl CPU {
    pub fn new(half_cycle_count: u64, registers: Registers) -> Self {
        Self {
            cycle_queue: VecDeque::default(),
            half_cycle_count,
            is_halted: false,
            registers,
            buses: Buses::default(),
            crossed_page: false,
            prev_nmi: false,
            nmi_detected: false,
            irq_detected: false,
            interrupt_disabled: None,
        }
    }

    pub fn tick(&mut self, buses: &mut ExternalBuses) {
        let cycle = self.cycle_queue.pop_front();
        match cycle {
            Some(cycle) => self.run_cycle(buses, cycle),
            None => {
                let interrupted = self.handle_interrupts(buses);

                if interrupted {
                    return;
                }

                self.get_cycles(buses);

                if let Some(interrupt_disable) = self.interrupt_disabled.take() {
                    self.registers.psr.set_interrupt_disable(interrupt_disable);
                }
            }
        }
    }

    fn handle_interrupts(&mut self, buses: &mut ExternalBuses) -> bool {
        let nmi = self.nmi_detected;
        let irq = self.irq_detected && !self.registers.psr.get_interrupt_disable();

        if !(nmi || irq) {
            return false;
        }

        if nmi {
            self.run_cycle(buses, [get_pc, read_opcode]);
            self.cycle_queue.extend(HANDLE_NMI.to_vec());
            self.nmi_detected = false;
        } else if irq {
            self.run_cycle(buses, [get_pc, read_opcode]);
            self.cycle_queue.extend(HANDLE_IRQ.to_vec());
            self.irq_detected = false;
        }

        true
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

    pub fn poweron(&mut self, buses: &mut ExternalBuses, initial_pc: Option<u16>) {
        self.registers = REGISTERS_AT_POWERON;
        self.registers.pc = match initial_pc {
            Some(addr) => split_u16!(addr),
            None => {
                let pcl = buses.peek(0xFFFC);
                let pch = buses.peek(0xFFFD);
                (pch, pcl)
            }
        };
    }

    pub fn reset(&mut self, buses: &mut ExternalBuses, initial_pc: Option<u16>) {
        self.registers.psr.set_interrupt_disable(true);
        self.registers.pc = match initial_pc {
            Some(addr) => split_u16!(addr),
            None => {
                let pcl = buses.peek(0xFFFC);
                let pch = buses.peek(0xFFFD);
                (pch, pcl)
            }
        };
    }

    pub fn get_cycle_queue(&self) -> VecDeque<Cycle> {
        self.cycle_queue.clone()
    }

    pub fn get_cycle_count(&self) -> u64 {
        self.half_cycle_count / 2
    }

    pub fn is_halted(&self) -> bool {
        self.is_halted
    }

    pub fn get_registers(&self) -> Registers {
        self.registers
    }
}
