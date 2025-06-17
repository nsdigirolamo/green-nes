use crate::emu::{State, instruction::Operation};

#[derive(Debug)]
pub enum LoadY {
    Immediate { operand: u8 },
    ZeroPage { operand: u8 },
    ZeroPageX { operand: u8 },
    Absolute { operand: u16 },
    AbsoluteX { operand: u16 },
}

impl Operation for LoadY {
    fn execute_on(&self, state: State) -> State {
        state
    }

    fn get_size(&self) -> u16 {
        match self {
            Self::Immediate { operand: _ } => 2,
            Self::ZeroPage { operand: _ } => 2,
            Self::ZeroPageX { operand: _ } => 2,
            Self::Absolute { operand: _ } => 3,
            Self::AbsoluteX { operand: _ } => 3,
        }
    }
}
