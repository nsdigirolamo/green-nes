use crate::emu::{State, instruction::Operation};

pub enum PullAccumulator {
    Implied,
}

impl Operation for PullAccumulator {
    fn execute_on(&self, state: State) -> State {
        state
    }

    fn get_size(&self) -> u8 {
        match self {
            Self::Implied => 1,
        }
    }
}
