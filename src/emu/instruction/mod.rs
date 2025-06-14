use crate::emu::{
    Operation, State,
    instruction::{
        access::{
            lda::LoadAccumulator, ldx::LoadX, ldy::LoadY, sta::StoreAccumulator, stx::StoreX,
            sty::StoreY,
        },
        arithmetic::{
            adc::AddWithCarry, dec::Decrement, dex::DecrementX, dey::DecrementY, inc::Increment,
            inx::IncrementX, iny::IncrementY, sbc::SubtractWithCarry,
        },
        transfer::{
            tax::TransferAccumulatorToX, tay::TransferAccumulatorToY, txa::TransferXToAccumulator,
            tya::TransferYToAccumulator,
        },
    },
};

pub mod access;
pub mod arithmetic;
pub mod transfer;

pub enum Instruction {
    // Access Operations
    LDA(LoadAccumulator),
    STA(StoreAccumulator),
    LDX(LoadX),
    STX(StoreX),
    LDY(LoadY),
    STY(StoreY),

    // Transfer Operations
    TAX(TransferAccumulatorToX),
    TXA(TransferXToAccumulator),
    TAY(TransferAccumulatorToY),
    TYA(TransferYToAccumulator),

    // Arithmetic Operations
    ADC(AddWithCarry),
    SBC(SubtractWithCarry),
    INC(Increment),
    DEC(Decrement),
    INX(IncrementX),
    DEX(DecrementX),
    INY(IncrementY),
    DEY(DecrementY),

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

            // Transfer Operations
            Instruction::TAX(tax) => tax.execute_on(state),
            Instruction::TXA(txa) => txa.execute_on(state),
            Instruction::TAY(tay) => tay.execute_on(state),
            Instruction::TYA(tya) => tya.execute_on(state),

            // Arithmetic Operations
            Instruction::ADC(adc) => adc.execute_on(state),
            Instruction::SBC(svc) => svc.execute_on(state),
            Instruction::INC(inc) => inc.execute_on(state),
            Instruction::DEC(dec) => dec.execute_on(state),
            Instruction::INX(inx) => inx.execute_on(state),
            Instruction::DEX(dex) => dex.execute_on(state),
            Instruction::INY(iny) => iny.execute_on(state),
            Instruction::DEY(dey) => dey.execute_on(state),

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

            // Transfer Operations
            Instruction::TAX(tax) => tax.get_size(),
            Instruction::TXA(txa) => txa.get_size(),
            Instruction::TAY(tay) => tay.get_size(),
            Instruction::TYA(tya) => tya.get_size(),

            // Arithmetic Operations
            Instruction::ADC(adc) => adc.get_size(),
            Instruction::SBC(svc) => svc.get_size(),
            Instruction::INC(inc) => inc.get_size(),
            Instruction::DEC(dec) => dec.get_size(),
            Instruction::INX(inx) => inx.get_size(),
            Instruction::DEX(dex) => dex.get_size(),
            Instruction::INY(iny) => iny.get_size(),
            Instruction::DEY(dey) => dey.get_size(),

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
