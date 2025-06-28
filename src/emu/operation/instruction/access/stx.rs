use std::collections::VecDeque;

use crate::emu::{
    Event,
    operation::{
        Operation,
        addressing::write_to_effective_absolute_address,
        instruction::{fetch_high_operand, fetch_low_operand},
    },
    state::State,
};

#[derive(Debug)]
pub enum STX {
    ZeroPage,
    ZeroPageY,
    Absolute,
}

impl Operation for STX {
    fn get_events(&self) -> VecDeque<Event> {
        match *self {
            STX::ZeroPage => panic!("stx zero page not implemented"),
            STX::ZeroPageY => panic!("stx zero page y not implemented"),
            STX::Absolute => VecDeque::from([fetch_low_operand, fetch_high_operand, stx_absolute]),
        }
    }
}

fn stx_absolute(state: &mut State) {
    let data = state.registers.x_index;

    state.cycle_data.acting_data = data;
    write_to_effective_absolute_address(state);
}
