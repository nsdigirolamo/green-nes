use crate::emu::{Operation, State};

#[derive(Debug)]
pub enum TransferAccumulatorToX {
    Implied,
}

impl Operation for TransferAccumulatorToX {
    fn execute_on(&self, state: State) -> State {
        state
    }

    fn get_size(&self) -> u16 {
        match self {
            Self::Implied => 1,
        }
    }
}
