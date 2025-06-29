use std::collections::VecDeque;

use crate::emu::{
    Event,
    operation::{
        Operation,
        addressing::{read_at_effective_absolute_address, read_at_effective_zero_page_address},
        instruction::{fetch_high_operand, fetch_low_operand},
    },
    state::State,
};

#[derive(Debug)]
pub enum EOR {
    Immediate,
    ZeroPage,
    ZeroPageX,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    IndirectX,
    IndirectY,
}

impl Operation for EOR {
    fn get_events(&self) -> VecDeque<Event> {
        match *self {
            EOR::Immediate => panic!("eor immediate not implemented"),
            EOR::ZeroPageX => panic!("eor zero page x not implemented"),
            EOR::ZeroPage => VecDeque::from([fetch_low_operand, fetch_high_operand, eor_zero_page]),
            EOR::Absolute => VecDeque::from([fetch_low_operand, fetch_high_operand, eor_absolute]),
            EOR::AbsoluteX => panic!("eor absolute x not implemented"),
            EOR::AbsoluteY => panic!("eor absolute y not implemented"),
            EOR::IndirectX => panic!("eor indirect x not implemented"),
            EOR::IndirectY => panic!("eor indirect y not implemented"),
        }
    }
}

fn eor(state: &mut State) {
    let data = state.cycle_data.acting_data;
    state.registers.accumulator ^= data;
    state.set_zero_flag(data == 0);
    state.set_negative_flag((data & 0b_1000_0000) != 0);
}

fn eor_absolute(state: &mut State) {
    read_at_effective_absolute_address(state);
    eor(state);
}

fn eor_zero_page(state: &mut State) {
    read_at_effective_zero_page_address(state);
    eor(state);
}
