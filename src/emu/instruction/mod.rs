use crate::emu::{
    Operation, State,
    instruction::access::{
        lda::LoadAccumulator, ldx::LoadX, ldy::LoadY, sta::StoreAccumulator, stx::StoreX,
        sty::StoreY,
    },
};

pub mod access;

pub enum Instruction {
    // Access Operations
    LDA(LoadAccumulator),
    STA(StoreAccumulator),
    LDX(LoadX),
    STX(StoreX),
    LDY(LoadY),
    STY(StoreY),

    NOP(NoOperation),
}

impl Operation for Instruction {
    fn execute_on(&self, state: State) -> State {
        match self {
            // Access Operations
            Instruction::LDA(lda) => lda.execute_on(state),
            Instruction::STA(sta) => sta.execute_on(state),
            Instruction::LDX(ldx) => ldx.execute_on(state),
            Instruction::STX(stx) => stx.execute_on(state),
            Instruction::LDY(ldy) => ldy.execute_on(state),
            Instruction::STY(sty) => sty.execute_on(state),

            Instruction::NOP(nop) => nop.execute_on(state),
        }
    }

    fn get_size(&self) -> u8 {
        match self {
            // Access Operations
            Instruction::LDA(lda) => lda.get_size(),
            Instruction::STA(sta) => sta.get_size(),
            Instruction::LDX(ldx) => ldx.get_size(),
            Instruction::STX(stx) => stx.get_size(),
            Instruction::LDY(ldy) => ldy.get_size(),
            Instruction::STY(sty) => sty.get_size(),

            Instruction::NOP(nop) => nop.get_size(),
        }
    }
}

pub enum NoOperation {
    Implied,
}

impl Operation for NoOperation {
    fn execute_on(&self, state: State) -> State {
        state
    }

    fn get_size(&self) -> u8 {
        1
    }
}
