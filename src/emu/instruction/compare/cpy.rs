use crate::emu::{Operation, State};

pub enum CompareY {
    Immediate { operand: u8 },
    ZeroPage { operand: u8 },
    Absolute { operand: u16 },
}

impl Operation for CompareY {
    fn execute_on(&self, state: State) -> State {
        state
    }

    fn get_size(&self) -> u8 {
        match self {
            Self::Immediate { operand: _ } => 2,
            Self::ZeroPage { operand: _ } => 2,
            Self::Absolute { operand: _ } => 3,
        }
    }
}
