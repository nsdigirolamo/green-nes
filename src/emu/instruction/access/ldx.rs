use crate::emu::{State, instruction::Operation};

pub enum LoadX {
    Immediate { operand: u8 },
    ZeroPage { operand: u8 },
    ZeroPageX { operand: u8 },
    Absolute { operand: u16 },
    AbsoluteX { operand: u16 },
}

impl Operation for LoadX {
    fn execute_on(&self, state: State) -> State {
        match self {
            _ => state,
        }
    }

    fn get_size(&self) -> u8 {
        match self {
            Self::Immediate { operand: _ } => 2,
            Self::ZeroPage { operand: _ } => 2,
            Self::ZeroPageX { operand: _ } => 2,
            Self::Absolute { operand: _ } => 3,
            Self::AbsoluteX { operand: _ } => 3,
        }
    }
}
