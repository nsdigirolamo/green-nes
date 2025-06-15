use crate::emu::{Operation, State};

pub enum Break {
    Implied,
}

impl Operation for Break {
    fn execute_on(&self, state: State) -> State {
        state
    }

    fn get_size(&self) -> u8 {
        match self {
            Self::Implied => 2,
        }
    }
}
