use crate::emu::{Operation, State};

#[derive(Debug)]
pub enum IncrementX {
    Implied,
}

impl Operation for IncrementX {
    fn execute_on(&self, state: State) -> State {
        state
    }

    fn get_size(&self) -> u16 {
        match self {
            Self::Implied => 2,
        }
    }
}
