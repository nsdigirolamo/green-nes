use crate::emu::{Operation, State};

#[derive(Debug)]
pub enum DecrementY {
    Implied,
}

impl Operation for DecrementY {
    fn execute_on(&self, state: State) -> State {
        state
    }

    fn get_size(&self) -> u16 {
        match self {
            Self::Implied => 2,
        }
    }
}
