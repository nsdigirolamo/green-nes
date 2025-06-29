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
pub enum CMP {
    Immediate,
    ZeroPage,
    ZeroPageX,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    IndirectX,
    IndirectY,
}

impl Operation for CMP {
    fn get_events(&self) -> VecDeque<Event> {
        match *self {
            CMP::Immediate => panic!("cmp immediate not implemented"),
            CMP::ZeroPageX => panic!("cmp zero page x not implemented"),
            CMP::ZeroPage => VecDeque::from([fetch_low_operand, fetch_high_operand, cmp_zero_page]),
            CMP::Absolute => VecDeque::from([fetch_low_operand, fetch_high_operand, cmp_absolute]),
            CMP::AbsoluteX => panic!("cmp absolute x not implemented"),
            CMP::AbsoluteY => panic!("cmp absolute y not implemented"),
            CMP::IndirectX => panic!("cmp indirect x not implemented"),
            CMP::IndirectY => panic!("cmp indirect y not implemented"),
        }
    }
}

fn cmp(state: &mut State) {
    let data = state.cycle_data.acting_data;
    let (difference, overflow) = state.registers.accumulator.overflowing_sub(data);
    state.set_carry_flag(overflow);
    state.set_zero_flag(difference == 0);
    state.set_negative_flag((difference & 0b_1000_0000) != 0);
}

fn cmp_absolute(state: &mut State) {
    read_at_effective_absolute_address(state);
    cmp(state);
}

fn cmp_zero_page(state: &mut State) {
    read_at_effective_zero_page_address(state);
    cmp(state);
}
