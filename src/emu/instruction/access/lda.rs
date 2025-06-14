use crate::emu::{State, instruction::Operation};

pub enum LoadAccumulator {
    Immediate { operand: u8 },
    ZeroPage { operand: u8 },
    ZeroPageX { operand: u8 },
    Absolute { operand: u16 },
    AbsoluteX { operand: u16 },
    AbsoluteY { operand: u16 },
    IndirectX { operand: u8 },
    IndirectY { operand: u8 },
}

impl Operation for LoadAccumulator {
    fn execute_on(&self, state: State) -> State {
        state
    }

    fn get_size(&self) -> u8 {
        match self {
            Self::Immediate { operand: _ } => 2,
            Self::ZeroPage { operand: _ } => 2,
            Self::ZeroPageX { operand: _ } => 2,
            Self::Absolute { operand: _ } => 3,
            Self::AbsoluteX { operand: _ } => 3,
            Self::AbsoluteY { operand: _ } => 3,
            Self::IndirectX { operand: _ } => 2,
            Self::IndirectY { operand: _ } => 2,
        }
    }
}
