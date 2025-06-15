use crate::emu::{State, instruction::Operation};

pub enum SetInterruptDisable {
    Implied,
}

impl Operation for SetInterruptDisable {
    fn execute_on(&self, state: State) -> State {
        state
    }

    fn get_size(&self) -> u8 {
        match self {
            Self::Implied => 1,
        }
    }
}
