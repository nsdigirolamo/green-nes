use crate::emu::{State, instruction::Operation};

#[derive(Debug)]
pub enum StoreX {
    ZeroPage { operand: u8 },
    ZeroPageY { operand: u8 },
    Absolute { operand: u16 },
}

impl Operation for StoreX {
    fn execute_on(&self, state: State) -> State {
        state
    }

    fn get_size(&self) -> u16 {
        match self {
            Self::ZeroPage { operand: _ } => 2,
            Self::ZeroPageY { operand: _ } => 2,
            Self::Absolute { operand: _ } => 3,
        }
    }
}
