use crate::emu::{State, instruction::Operation};

#[derive(Debug)]
pub enum StoreAccumulator {
    ZeroPage { operand: u8 },
    ZeroPageX { operand: u8 },
    Absolute { operand: u16 },
    AbsoluteX { operand: u16 },
    AbsoluteY { operand: u16 },
    IndirectX { operand: u8 },
    IndirectY { operand: u8 },
}

impl Operation for StoreAccumulator {
    fn execute_on(&self, state: State) -> State {
        state
    }

    fn get_size(&self) -> u16 {
        match self {
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
