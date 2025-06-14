use crate::emu::instruction::{
    Instruction, NoOperation,
    access::{
        lda::LoadAccumulator, ldx::LoadX, ldy::LoadY, sta::StoreAccumulator, stx::StoreX,
        sty::StoreY,
    },
    arithmetic::{
        adc::AddWithCarry, dec::Decrement, dex::DecrementX, dey::DecrementY, inc::Increment,
        inx::IncrementX, iny::IncrementY, sbc::SubtractWithCarry,
    },
    transfer::{
        tax::TransferAccumulatorToX, tay::TransferAccumulatorToY, txa::TransferXToAccumulator,
        tya::TransferYToAccumulator,
    },
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

            0x85 => Instruction::STA(StoreAccumulator::ZeroPage { operand: byte_two }),
            0x95 => Instruction::STA(StoreAccumulator::ZeroPageX { operand: byte_two }),
            0x8D => Instruction::STA(StoreAccumulator::Absolute {
                operand: concat_u8!(byte_three, byte_two),
            }),
            0x9D => Instruction::STA(StoreAccumulator::AbsoluteX {
                operand: concat_u8!(byte_three, byte_two),
            }),
            0x99 => Instruction::STA(StoreAccumulator::AbsoluteY {
                operand: concat_u8!(byte_three, byte_two),
            }),
            0x81 => Instruction::STA(StoreAccumulator::IndirectX { operand: byte_two }),
            0x91 => Instruction::STA(StoreAccumulator::IndirectY { operand: byte_two }),

            0xA2 => Instruction::LDX(LoadX::Immediate { operand: byte_two }),
            0xA6 => Instruction::LDX(LoadX::ZeroPage { operand: byte_two }),
            0xB6 => Instruction::LDX(LoadX::ZeroPageX { operand: byte_two }),
            0xAE => Instruction::LDX(LoadX::Absolute {
                operand: concat_u8!(byte_three, byte_two),
            }),
            0xBE => Instruction::LDX(LoadX::AbsoluteX {
                operand: concat_u8!(byte_three, byte_two),
            }),

            0x86 => Instruction::STX(StoreX::ZeroPage { operand: byte_two }),
            0x96 => Instruction::STX(StoreX::ZeroPageY { operand: byte_two }),
            0x8E => Instruction::STX(StoreX::Absolute {
                operand: concat_u8!(byte_three, byte_two),
            }),

            0xA0 => Instruction::LDY(LoadY::Immediate { operand: byte_two }),
            0xA4 => Instruction::LDY(LoadY::ZeroPage { operand: byte_two }),
            0xB4 => Instruction::LDY(LoadY::ZeroPageX { operand: byte_two }),
            0xAC => Instruction::LDY(LoadY::Absolute {
                operand: concat_u8!(byte_three, byte_two),
            }),
            0xBC => Instruction::LDY(LoadY::AbsoluteX {
                operand: concat_u8!(byte_three, byte_two),
            }),

            0x84 => Instruction::STY(StoreY::ZeroPage { operand: byte_two }),
            0x94 => Instruction::STY(StoreY::ZeroPageX { operand: byte_two }),
            0x8C => Instruction::STY(StoreY::Absolute {
                operand: concat_u8!(byte_three, byte_two),
            }),

            0xAA => Instruction::TAX(TransferAccumulatorToX::Implied),

            0x8A => Instruction::TXA(TransferXToAccumulator::Implied),

            0xA8 => Instruction::TAY(TransferAccumulatorToY::Implied),

            0x98 => Instruction::TYA(TransferYToAccumulator::Implied),

            0x69 => Instruction::ADC(AddWithCarry::Immediate { operand: byte_two }),
            0x65 => Instruction::ADC(AddWithCarry::ZeroPage { operand: byte_two }),
            0x75 => Instruction::ADC(AddWithCarry::ZeroPageX { operand: byte_two }),
            0x6D => Instruction::ADC(AddWithCarry::Absolute {
                operand: concat_u8!(byte_three, byte_two),
            }),
            0x7D => Instruction::ADC(AddWithCarry::AbsoluteX {
                operand: concat_u8!(byte_three, byte_two),
            }),
            0x79 => Instruction::ADC(AddWithCarry::AbsoluteY {
                operand: concat_u8!(byte_three, byte_two),
            }),
            0x61 => Instruction::ADC(AddWithCarry::IndirectX { operand: byte_two }),
            0x71 => Instruction::ADC(AddWithCarry::IndirectY { operand: byte_two }),

            0xE9 => Instruction::SBC(SubtractWithCarry::Immediate { operand: byte_two }),
            0xE5 => Instruction::SBC(SubtractWithCarry::ZeroPage { operand: byte_two }),
            0xF5 => Instruction::SBC(SubtractWithCarry::ZeroPageX { operand: byte_two }),
            0xED => Instruction::SBC(SubtractWithCarry::Absolute {
                operand: concat_u8!(byte_three, byte_two),
            }),
            0xFD => Instruction::SBC(SubtractWithCarry::AbsoluteX {
                operand: concat_u8!(byte_three, byte_two),
            }),
            0xF9 => Instruction::SBC(SubtractWithCarry::AbsoluteY {
                operand: concat_u8!(byte_three, byte_two),
            }),
            0xE1 => Instruction::SBC(SubtractWithCarry::IndirectX { operand: byte_two }),
            0xF1 => Instruction::SBC(SubtractWithCarry::IndirectY { operand: byte_two }),

            0xE6 => Instruction::INC(Increment::ZeroPage { operand: byte_two }),
            0xF6 => Instruction::INC(Increment::ZeroPageX { operand: byte_two }),
            0xEE => Instruction::INC(Increment::Absolute {
                operand: concat_u8!(byte_three, byte_two),
            }),
            0xFE => Instruction::INC(Increment::AbsoluteX {
                operand: concat_u8!(byte_three, byte_two),
            }),

            0xC6 => Instruction::DEC(Decrement::ZeroPage { operand: byte_two }),
            0xD6 => Instruction::DEC(Decrement::ZeroPageX { operand: byte_two }),
            0xCE => Instruction::DEC(Decrement::Absolute {
                operand: concat_u8!(byte_three, byte_two),
            }),
            0xDE => Instruction::DEC(Decrement::AbsoluteX {
                operand: concat_u8!(byte_three, byte_two),
            }),

            0xE8 => Instruction::INX(IncrementX::Implied),

            0xCA => Instruction::DEX(DecrementX::Implied),

            0xC8 => Instruction::INY(IncrementY::Implied),

            0x88 => Instruction::DEY(DecrementY::Implied),

            _ => Instruction::NOP(NoOperation::Implied), // @TODO: Remove this once all opcodes are matched.
        }
    }
}
