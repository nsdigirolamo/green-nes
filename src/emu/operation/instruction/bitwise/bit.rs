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
pub enum BIT {
    ZeroPage,
    Absolute,
}

impl Operation for BIT {
    fn get_events(&self) -> VecDeque<Event> {
        match *self {
            BIT::ZeroPage => VecDeque::from([fetch_low_operand, bit_zero_page]),
            BIT::Absolute => VecDeque::from([fetch_low_operand, fetch_high_operand, bit_absolute]),
        }
    }
}

fn bit(state: &mut State) {
    let data = state.cycle_data.acting_data;
    let result = state.registers.accumulator & data;
    state.set_zero_flag(result == 0);
    state.set_overflow_flag((data & 0b_0100_0000) != 0);
    state.set_negative_flag((data & 0b_1000_0000) != 0);
}

fn bit_absolute(state: &mut State) {
    read_at_effective_absolute_address(state);
    bit(state);
}

fn bit_zero_page(state: &mut State) {
    read_at_effective_zero_page_address(state);
    bit(state);
}
