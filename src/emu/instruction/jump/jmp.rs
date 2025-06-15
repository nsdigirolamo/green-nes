use crate::emu::{Operation, State};

pub enum Jump {
    Absolute { operand: u16 },
    Indirect { operand: u16 },
}

impl Operation for Jump {
    fn execute_on(&self, state: State) -> State {
        state
    }

    fn get_size(&self) -> u8 {
        match self {
            Self::Absolute { operand: _ } => 3,
            Self::Indirect { operand: _ } => 3,
        }
    }
}
