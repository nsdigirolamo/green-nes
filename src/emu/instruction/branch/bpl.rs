use crate::emu::{Operation, State};

#[derive(Debug)]
pub enum BranchIfPlus {
    Relative { operand: u8 },
}

impl Operation for BranchIfPlus {
    fn execute_on(&self, state: State) -> State {
        state
    }

    fn get_size(&self) -> u16 {
        match self {
            Self::Relative { operand: _ } => 2,
        }
    }
}
