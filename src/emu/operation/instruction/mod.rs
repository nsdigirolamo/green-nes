use std::collections::VecDeque;

use crate::{
    concat_u8,
    emu::{Event, operation::Operation, state::State},
};

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

/**
Reads the contents of the program counter, loads it into the high byte of the
effective address, and then increments the program counter.
*/
fn fetch_high_effective_address_byte(state: &mut State) {
    let high_address_byte = state.read_from_pc_address();
    state.cycle_data.effective_address.0 = high_address_byte;
    state.increment_pc_address();
}

/**
Reads the contents of the program counter, loads it into the low byte of the
effective address, and then increments the program counter.
*/
fn fetch_low_effective_address_byte(state: &mut State) {
    let low_address_byte = state.read_from_pc_address();
    state.cycle_data.effective_address.1 = low_address_byte;
    state.increment_pc_address();
}

/**
Reads the contents of the program counter, loads it into the effective address
as a location in page zero, and then increments the program counter.
*/
fn fetch_effective_zero_page_address(state: &mut State) {
    let low_address_byte = state.read_from_pc_address();
    state.cycle_data.effective_address = (0x00, low_address_byte);
}

/**
Adds the contents of the X Index register to the zero page effective address.
*/
fn do_effective_zero_page_address_x_index(state: &mut State) {
    let address = concat_u8!(
        state.cycle_data.effective_address.0,
        state.cycle_data.effective_address.1
    );

    // TODO: Determine if the data read from memory needs to be stored in the
    // location being operated on. Example: Does `LDA ($01, X)` need to load the
    // contents of 0x0001 into the accumulator here? Or does the accumulator
    // only need to be loaded during the final cycle?

    // This memory read is required for cycle accuracy.
    let _ = state.read_from_memory(address);
    // Add X Index to low byte. High byte is always 0x00.
    let low_address_byte = state.cycle_data.effective_address.1;
    let offset = state.registers.x_index;
    let low_address_byte = low_address_byte.wrapping_add(offset);
    state.cycle_data.effective_address = (0x00, low_address_byte);
}

/**
Adds the contents of the Y Index register to the zero page effective address.
*/
fn do_effective_zero_page_address_y_index(state: &mut State) {
    let address = concat_u8!(
        state.cycle_data.effective_address.0,
        state.cycle_data.effective_address.1
    );

    // TODO: Determine if the data read from memory needs to be stored in the
    // location being operated on. Example: Does `LDX ($01, Y)` need to load the
    // contents of 0x0001 into the x index register here? Or does the x index
    // only need to be loaded during the final cycle?

    // This memory read is required for cycle accuracy.
    let _ = state.read_from_memory(address);
    // Add Y Index to low byte. High byte is always 0x00.
    let low_address_byte = state.cycle_data.effective_address.1;
    let offset = state.registers.y_index;
    let low_address_byte = low_address_byte.wrapping_add(offset);
    state.cycle_data.effective_address = (0x00, low_address_byte);
}

fn read_from_effective_address(state: &mut State) {
    let address = concat_u8!(
        state.cycle_data.effective_address.0,
        state.cycle_data.effective_address.1
    );
    let data = state.read_from_memory(address);
    state.cycle_data.acting_data = data;
}

fn write_to_effective_address(state: &mut State) {
    let address = concat_u8!(
        state.cycle_data.effective_address.0,
        state.cycle_data.effective_address.1
    );
    let data = state.cycle_data.acting_data;
    state.write_to_memory(address, data);
}
