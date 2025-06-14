use crate::emu::{Operation, State};

pub enum IncrementY {
    Implied,
}

impl Operation for IncrementY {
    fn execute_on(&self, state: State) -> State {
        state
    }

    fn get_size(&self) -> u8 {
        match self {
            Self::Implied => 2,
        }
    }
}
