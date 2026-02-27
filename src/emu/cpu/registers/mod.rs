use crate::emu::cpu::registers::flags::Flags;

pub mod flags;

#[derive(Default, Clone, Copy)]
/// Internal CPU Registers
pub struct Registers {
    /// Accumulator
    pub a: u8,
    /// X Index Register
    pub x_index: u8,
    /// Y Index Register
    pub y_index: u8,
    /// Program Counter (PCH, PCL)
    pub pc: (u8, u8),
    /// Stack Pointer
    pub sp: u8,
    /// Processor Status Register
    pub psr: Flags,
    /// Instruction Register
    pub ir: u8,
}

/// The state of the CPU registers after power up.
pub const REGISTERS_AT_POWERON: Registers = Registers {
    a: 0x00,
    x_index: 0x00,
    y_index: 0x00,
    pc: (0xFF, 0xFC),
    sp: 0xFD,
    psr: Flags(0b_0010_0100),
    ir: 0x00,
};
