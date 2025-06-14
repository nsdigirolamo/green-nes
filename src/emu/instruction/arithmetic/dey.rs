use crate::emu::{Operation, State};

pub enum DecrementY {
    Implied,
}

impl Operation for DecrementY {
    fn execute_on(&self, state: State) -> State {
        state
    }

    fn get_size(&self) -> u8 {
        match self {
            Self::Implied => 2,
        }
    }
}
