use crate::emu::{Operation, State};

pub enum TransferYToAccumulator {
    Implied,
}

impl Operation for TransferYToAccumulator {
    fn execute_on(&self, state: State) -> State {
        state
    }

    fn get_size(&self) -> u8 {
        match self {
            Self::Implied => 1,
        }
    }
}
