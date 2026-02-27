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

/// The NES's CPU.
#[derive(Default, Clone)]
pub struct CPU {
    /// The next cycles to be executed by the CPU.
    cycle_queue: VecDeque<Cycle>,
    /// The number of half-cycles executed by the CPU.
    half_cycle_count: u64,
    /// `true` if the CPU is halted, `false` otherwise.
    is_halted: bool,
    /// Registers internal to the CPU.
    registers: Registers,
    /// Buses internal to the CPU.
    buses: Buses,
    /// Indicates that a memory page was crossed in a previous cycle.
    crossed_page: bool,
    /// The state of the NMI pin on the external buses during the previous cycle.
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

    /// Advances the state of the CPU ahead by a single cycle.
    ///
    /// Executes a single cycle from the cycle queue. If there are no cycles in
    /// the cycle queue, then the CPU has completed an instruction. After the
    /// CPU completes an instruction it handles any pending interrupts and then
    /// extends the cycle queue with new cycles from the next instruction.
    ///
    /// # Arguments
    ///
    /// * `buses`: The external buses the CPU will use to access RAM and I/O.
    ///
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

    /// Handles any pending interrupt requests.
    ///
    /// Extends the cycle queue with the appropriate handlers depending on if
    /// there are any pending interrupt requests. Returns `true` if there were
    /// any requests that were handled and `false` otherwise.
    ///
    /// # Arguments
    ///
    /// * `buses`: The external buses the CPU will use to access RAM and I/O.
    ///
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

    /// Executes a cycle.
    ///
    /// Immediately executes the two half-cycles from the given cycle,
    /// increasing the half-cycle count by two. If the buses indicate that an
    /// interrupt request has occurred, then the CPU's internal interrupt
    /// request flags are set appropriately.
    ///
    /// # Arguments
    ///
    /// * `buses`: The external buses the CPU will use to access RAM and I/O.
    /// * `cycle`: The cycle to be executed.
    ///
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

    /// Powers on the CPU.
    ///
    /// Sets the CPU's registers to their appropriate values for a power up.
    ///
    /// # Arguments
    ///
    /// * `buses`: The external buses the CPU will use to access RAM and I/O.
    /// * `initial_pc`: An optional program counter value. If `None` then the
    ///   program counter is set to the vector stored at `$FFFC`.
    ///
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

    /// Resets the CPU.
    ///
    /// Sets the CPU's registers to their appropriate values following a power
    /// cycle.
    ///
    /// # Arguments
    ///
    /// * `buses`: The external buses the CPU will use to access RAM and I/O.
    /// * `initial_pc`: An optional program counter value. If `None` then the
    ///   program counter is set to the vector stored at `$FFFC`.
    ///
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
}
