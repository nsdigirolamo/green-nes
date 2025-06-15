use crate::emu::{State, instruction::Operation};

pub enum SetDecimal {
    Implied,
}

impl Operation for SetDecimal {
    fn execute_on(&self, state: State) -> State {
        state
    }

    fn get_size(&self) -> u8 {
        match self {
            Self::Implied => 1,
        }
    }
}
