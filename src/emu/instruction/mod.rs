use crate::emu::{
    State,
    instruction::access::{lda::LoadAccumulator, ldx::LoadX},
};

pub mod access;

pub trait Operation {
    fn execute_on(&self, state: State) -> State;
    fn get_size(&self) -> u8;
}

pub enum Instruction {
    LDA(LoadAccumulator),
    LDX(LoadX),
}

impl Operation for Instruction {
    fn execute_on(&self, state: State) -> State {
        match self {
            Instruction::LDA(instruction) => instruction.execute_on(state),
            Instruction::LDX(instruction) => instruction.execute_on(state),
        }
    }

    fn get_size(&self) -> u8 {
        match self {
            Instruction::LDA(instruction) => instruction.get_size(),
            Instruction::LDX(instruction) => instruction.get_size(),
        }
    }
}
