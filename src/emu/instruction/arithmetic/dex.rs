use crate::emu::{Operation, State};

pub enum DecrementX {
    Implied,
}

impl Operation for DecrementX {
    fn execute_on(&self, state: State) -> State {
        state
    }

    fn get_size(&self) -> u8 {
        match self {
            Self::Implied => 2,
        }
    }
}
