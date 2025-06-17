use crate::emu::{Operation, State};

#[derive(Debug)]
pub enum JumpToSubroutine {
    Absolute { operand: u16 },
}

impl Operation for JumpToSubroutine {
    fn execute_on(&self, state: State) -> State {
        state
    }

    fn get_size(&self) -> u16 {
        match self {
            Self::Absolute { operand: _ } => 3,
        }
    }
}
