use crate::emu::{State, instruction::Operation};

#[derive(Debug)]
pub enum ClearOverflow {
    Implied,
}

impl Operation for ClearOverflow {
    fn execute_on(&self, state: State) -> State {
        state
    }

    fn get_size(&self) -> u16 {
        match self {
            Self::Implied => 1,
        }
    }
}
