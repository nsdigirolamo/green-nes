use crate::emu::{Operation, State};

pub enum RotateLeft {
    Accumulator,
    ZeroPage { operand: u8 },
    ZeroPageX { operand: u8 },
    Absolute { operand: u16 },
    AbsoluteX { operand: u16 },
}

impl Operation for RotateLeft {
    fn execute_on(&self, state: State) -> State {
        state
    }

    fn get_size(&self) -> u8 {
        match self {
            Self::Accumulator => 1,
            Self::ZeroPage { operand: _ } => 2,
            Self::ZeroPageX { operand: _ } => 2,
            Self::Absolute { operand: _ } => 3,
            Self::AbsoluteX { operand: _ } => 3,
        }
    }
}
