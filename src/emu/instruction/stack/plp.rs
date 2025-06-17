use crate::emu::{State, instruction::Operation};

#[derive(Debug)]
pub enum PullProcessorStatus {
    Implied,
}

impl Operation for PullProcessorStatus {
    fn execute_on(&self, state: State) -> State {
        state
    }

    fn get_size(&self) -> u16 {
        match self {
            Self::Implied => 1,
        }
    }
}
