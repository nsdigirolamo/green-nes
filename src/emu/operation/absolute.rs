use std::collections::VecDeque;

use crate::concat_u8;
use crate::emu::operation::{fetch_high_operand, fetch_low_operand};
use crate::emu::state::State;
use crate::emu::{Event, Operation};

macro_rules! concat_u8 {
    ($high:expr, $low:expr) => {
        (($high as u16) << 8) | ($low as u16)
    };
}

#[derive(Debug)]
pub enum AbsoluteAddressingMode {
    JMP,
    LDA,
    LDX,
    LDY,
    EOR,
    AND,
    ORA,
    ADC,
    SBC,
    CMP,
    BIT,
    LAX,
    NOP,
    ASL,
    LSR,
    ROL,
    ROR,
    INC,
    DEC,
    SLO,
    SRE,
    RLA,
    RRA,
    ISB,
    DCP,
    STA,
    STX,
    STY,
    SAX,
}

impl Operation for AbsoluteAddressingMode {
    fn get_events(&self) -> VecDeque<Event> {
        match *self {
            AbsoluteAddressingMode::JMP => {
                VecDeque::from([fetch_low_operand, fetch_high_operand, jmp])
            }
            AbsoluteAddressingMode::LDA => {
                VecDeque::from([fetch_low_operand, fetch_high_operand, lda])
            }
            _ => VecDeque::new(),
        }
    }
}

fn get_absolute_address(state: &State) -> u16 {
    let low_address_byte = state.cycle_data.low_operand;
    let high_address_byte = state.cycle_data.high_operand;

    concat_u8!(high_address_byte, low_address_byte)
}

fn jmp(state: &mut State) {
    let address = get_absolute_address(state);

    state.registers.program_counter = address
}

fn lda(state: &mut State) {
    let address = get_absolute_address(state);
    let data = state.read_from_memory(address);

    state.registers.accumulator = data;
}
