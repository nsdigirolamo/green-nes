use crate::emu::{State, instruction::Operation};

#[derive(Debug)]
pub enum PushProcessorStatus {
    Implied,
}

impl Operation for PushProcessorStatus {
    fn execute_on(&self, state: State) -> State {
        state
    }

    fn get_size(&self) -> u16 {
        match self {
            Self::Implied => 1,
        }
    }
}
