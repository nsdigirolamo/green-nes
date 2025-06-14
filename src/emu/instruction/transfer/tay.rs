use crate::emu::{Operation, State};

pub enum TransferAccumulatorToY {
    Implied,
}

impl Operation for TransferAccumulatorToY {
    fn execute_on(&self, state: State) -> State {
        state
    }

    fn get_size(&self) -> u8 {
        match self {
            Self::Implied => 1,
        }
    }
}
