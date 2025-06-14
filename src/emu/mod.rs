use crate::emu::instruction::{
    Instruction,
    access::{lda::LoadAccumulator, ldx::LoadX},
};

pub mod instruction;

const MEMORY_LOCATION_COUNT: usize = 65535;

pub trait Operation {
    fn execute_on(&self, state: State) -> State;
    fn get_size(&self) -> u8;
}

#[macro_export]
macro_rules! concat_u8 {
    ($high:expr, $low:expr) => {
        (($high as u16) << 8) | ($low as u16)
    };
}

#[macro_export]
macro_rules! split_u16 {
    ($value:expr) => {
        (($value >> 8) as u8, $value as u8)
    };
}

pub struct Registers {
    pub a: u8,   // Acumulator
    pub x: u8,   // X Index
    pub y: u8,   // Y Index
    pub pc: u16, // Program Counter
    pub s: u8,   // Stack Pointer
    pub p: u8,   // Processor Status Register
}

pub struct State {
    pub registers: Registers,
    pub memory: [u8; MEMORY_LOCATION_COUNT],
}

impl State {
    pub fn fetch_next_operation(state: State) -> impl Operation {
        let program_counter = state.registers.pc as usize;
        let byte_one = state.memory[program_counter];
        let byte_two = state.memory[program_counter + 1];
        let byte_three = state.memory[program_counter + 2];

        match byte_one {
            0xA9 => Instruction::LDA(LoadAccumulator::Immediate { operand: byte_two }),
            0xA5 => Instruction::LDA(LoadAccumulator::ZeroPage { operand: byte_two }),
            0xB5 => Instruction::LDA(LoadAccumulator::ZeroPageX { operand: byte_two }),
            0xAD => Instruction::LDA(LoadAccumulator::Absolute {
                operand: concat_u8!(byte_three, byte_two),
            }),
            0xBD => Instruction::LDA(LoadAccumulator::AbsoluteX {
                operand: concat_u8!(byte_three, byte_two),
            }),
            0xB9 => Instruction::LDA(LoadAccumulator::AbsoluteY {
                operand: concat_u8!(byte_three, byte_two),
            }),
            0xA1 => Instruction::LDA(LoadAccumulator::IndirectX { operand: byte_two }),
            0xB1 => Instruction::LDA(LoadAccumulator::IndirectY { operand: byte_two }),
            0xA0 => Instruction::LDX(LoadX::Immediate { operand: byte_two }),
            0xA4 => Instruction::LDX(LoadX::ZeroPage { operand: byte_two }),
            0xB4 => Instruction::LDX(LoadX::ZeroPageX { operand: byte_two }),
            0xAC => Instruction::LDX(LoadX::Absolute {
                operand: concat_u8!(byte_three, byte_two),
            }),
            0xBC => Instruction::LDX(LoadX::AbsoluteX {
                operand: concat_u8!(byte_three, byte_two),
            }),
            _ => Instruction::LDA(LoadAccumulator::Immediate { operand: byte_two }), // @TODO: Remove this once all opcodes are matched.
        }
    }
}
