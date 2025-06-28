use std::collections::VecDeque;

use crate::emu::{Event, operation::Operation, state::State};

pub mod access;
pub mod arithmetic;
pub mod bitwise;
pub mod compare;
pub mod jump;
pub mod other;
pub mod shift;

#[derive(Debug)]
pub enum Instruction {
    LDA(access::lda::LDA),
    STA,
    LDX(access::ldx::LDX),
    STX,
    LDY(access::ldy::LDY),
    STY,
    TAX,
    TXA,
    TAY,
    TYA,
    ADC(arithmetic::adc::ADC),
    SBC(arithmetic::sbc::SBC),
    INC,
    DEC,
    INX,
    DEX,
    INY,
    DEY,
    ASL,
    LSR,
    ROL,
    ROR,
    AND(bitwise::and::AND),
    ORA(bitwise::ora::ORA),
    EOR(bitwise::eor::EOR),
    BIT(bitwise::bit::BIT),
    CMP(compare::cmp::CMP),
    CPX,
    CPY,
    BCC,
    BCS,
    BEQ,
    BNE,
    BPL,
    BMI,
    BVC,
    BVS,
    JMP(jump::jmp::JMP),
    JSR,
    RTS,
    BRK,
    RTI,
    PHA,
    PLA,
    PHP,
    PLP,
    TXS,
    TSX,
    CLC,
    SEC,
    CLI,
    SEI,
    CLD,
    SED,
    CLV,
    NOP(other::nop::NOP),
}

impl Operation for Instruction {
    fn get_events(&self) -> VecDeque<Event> {
        match self {
            Instruction::LDA(lda) => lda.get_events(),
            Instruction::STA => VecDeque::new(),
            Instruction::LDX(ldx) => ldx.get_events(),
            Instruction::STX => VecDeque::new(),
            Instruction::LDY(lda) => lda.get_events(),
            Instruction::STY => VecDeque::new(),
            Instruction::TAX => VecDeque::new(),
            Instruction::TXA => VecDeque::new(),
            Instruction::TAY => VecDeque::new(),
            Instruction::TYA => VecDeque::new(),
            Instruction::ADC(adc) => adc.get_events(),
            Instruction::SBC(sbc) => sbc.get_events(),
            Instruction::INC => VecDeque::new(),
            Instruction::DEC => VecDeque::new(),
            Instruction::INX => VecDeque::new(),
            Instruction::DEX => VecDeque::new(),
            Instruction::INY => VecDeque::new(),
            Instruction::DEY => VecDeque::new(),
            Instruction::ASL => VecDeque::new(),
            Instruction::LSR => VecDeque::new(),
            Instruction::ROL => VecDeque::new(),
            Instruction::ROR => VecDeque::new(),
            Instruction::AND(and) => and.get_events(),
            Instruction::ORA(ora) => ora.get_events(),
            Instruction::EOR(eor) => eor.get_events(),
            Instruction::BIT(bit) => bit.get_events(),
            Instruction::CMP(cmp) => cmp.get_events(),
            Instruction::CPX => VecDeque::new(),
            Instruction::CPY => VecDeque::new(),
            Instruction::BCC => VecDeque::new(),
            Instruction::BCS => VecDeque::new(),
            Instruction::BEQ => VecDeque::new(),
            Instruction::BNE => VecDeque::new(),
            Instruction::BPL => VecDeque::new(),
            Instruction::BMI => VecDeque::new(),
            Instruction::BVC => VecDeque::new(),
            Instruction::BVS => VecDeque::new(),
            Instruction::JMP(jmp) => jmp.get_events(),
            Instruction::JSR => VecDeque::new(),
            Instruction::RTS => VecDeque::new(),
            Instruction::BRK => VecDeque::new(),
            Instruction::RTI => VecDeque::new(),
            Instruction::PHA => VecDeque::new(),
            Instruction::PLA => VecDeque::new(),
            Instruction::PHP => VecDeque::new(),
            Instruction::PLP => VecDeque::new(),
            Instruction::TXS => VecDeque::new(),
            Instruction::TSX => VecDeque::new(),
            Instruction::CLC => VecDeque::new(),
            Instruction::SEC => VecDeque::new(),
            Instruction::CLI => VecDeque::new(),
            Instruction::SEI => VecDeque::new(),
            Instruction::CLD => VecDeque::new(),
            Instruction::SED => VecDeque::new(),
            Instruction::CLV => VecDeque::new(),
            Instruction::NOP(nop) => nop.get_events(),
        }
    }
}

fn fetch_high_operand(state: &mut State) {
    let data = state.read_from_pc_address();
    state.cycle_data.high_operand = data;

    let pc = state.registers.program_counter;
    state.registers.program_counter = pc.wrapping_add(1);
}

fn fetch_low_operand(state: &mut State) {
    let data = state.read_from_pc_address();
    state.cycle_data.low_operand = data;

    let pc = state.registers.program_counter;
    state.registers.program_counter = pc.wrapping_add(1);
}
