use crate::emu::{Operation, State};

pub enum BranchIfOverflowClear {
    Relative { operand: u8 },
}

impl Operation for BranchIfOverflowClear {
    fn execute_on(&self, state: State) -> State {
        state
    }

    fn get_size(&self) -> u8 {
        match self {
            Self::Relative { operand: _ } => 2,
        }
    }
}
