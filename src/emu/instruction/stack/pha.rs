use crate::emu::{State, instruction::Operation};

pub enum PushAccumulator {
    Implied,
}

impl Operation for PushAccumulator {
    fn execute_on(&self, state: State) -> State {
        state
    }

    fn get_size(&self) -> u8 {
        match self {
            Self::Implied => 1,
        }
    }
}
