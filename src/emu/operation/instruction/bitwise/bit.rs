use std::collections::VecDeque;

use crate::emu::{
    Event,
    operation::{
        Operation,
        addressing::get_absolute_address,
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
            BIT::ZeroPage => panic!("bit zero page not implemented"),
            BIT::Absolute => VecDeque::from([fetch_low_operand, fetch_high_operand, bit_absolute]),
        }
    }
}

fn bit_absolute(state: &mut State) {
    let address = get_absolute_address(state);
    let data = state.read_from_memory(address);

    let result = state.registers.accumulator & data;

    state.set_zero_flag(result == 0);
    state.registers.processor_status &= data & 0b01000000;
    state.registers.processor_status &= data & 0b10000000;
}
