#[derive(Default, Clone, Copy)]
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

pub const REGISTERS_AT_POWERON: Registers = Registers {
    a: 0x00,
    x_index: 0x00,
    y_index: 0x00,
    pc: (0xFF, 0xFC),
    sp: 0xFD,
    psr: 0b_0000_0100,
    ir: 0x00,
};
