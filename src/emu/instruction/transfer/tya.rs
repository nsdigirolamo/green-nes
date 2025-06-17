use crate::emu::{Operation, State};

#[derive(Debug)]
pub enum TransferYToAccumulator {
    Implied,
}

impl Operation for TransferYToAccumulator {
    fn execute_on(&self, state: State) -> State {
        state
    }

    fn get_size(&self) -> u16 {
        match self {
            Self::Implied => 1,
        }
    }
}
