use crate::emu::{
    State,
    instruction::access::{lda::LoadAccumulator, ldx::LoadX, ldy::LoadY},
};

pub mod access;

pub trait Operation {
    fn execute_on(&self, state: State) -> State;
    fn get_size(&self) -> u8;
}

pub enum Instruction {
    LDA(LoadAccumulator),
    LDX(LoadX),
    LDY(LoadY),
}

impl Operation for Instruction {
    fn execute_on(&self, state: State) -> State {
        match self {
            Instruction::LDA(lda) => lda.execute_on(state),
            Instruction::LDX(ldx) => ldx.execute_on(state),
            Instruction::LDY(ldy) => ldy.execute_on(state),
        }
    }

    fn get_size(&self) -> u8 {
        match self {
            Instruction::LDA(lda) => lda.get_size(),
            Instruction::LDX(ldx) => ldx.get_size(),
            Instruction::LDY(ldy) => ldy.get_size(),
        }
    }
}
