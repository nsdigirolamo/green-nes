use crate::emu::{
    Operation, State,
    instruction::access::{
        lda::LoadAccumulator, ldx::LoadX, ldy::LoadY, sta::StoreAccumulator, stx::StoreX,
        sty::StoreY,
    },
};

pub mod access;

pub enum Instruction {
    LDA(LoadAccumulator),
    STA(StoreAccumulator),
    LDX(LoadX),
    STX(StoreX),
    LDY(LoadY),
    STY(StoreY),
}

impl Operation for Instruction {
    fn execute_on(&self, state: State) -> State {
        match self {
            Instruction::LDA(lda) => lda.execute_on(state),
            Instruction::STA(sta) => sta.execute_on(state),
            Instruction::LDX(ldx) => ldx.execute_on(state),
            Instruction::STX(stx) => stx.execute_on(state),
            Instruction::LDY(ldy) => ldy.execute_on(state),
            Instruction::STY(sty) => sty.execute_on(state),
        }
    }

    fn get_size(&self) -> u8 {
        match self {
            Instruction::LDA(lda) => lda.get_size(),
            Instruction::STA(sta) => sta.get_size(),
            Instruction::LDX(ldx) => ldx.get_size(),
            Instruction::STX(stx) => stx.get_size(),
            Instruction::LDY(ldy) => ldy.get_size(),
            Instruction::STY(sty) => sty.get_size(),
        }
    }
}
