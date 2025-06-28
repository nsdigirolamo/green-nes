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
pub enum STY {
    ZeroPage,
    ZeroPageX,
    Absolute,
}

impl Operation for STY {
    fn get_events(&self) -> VecDeque<Event> {
        match *self {
            STY::ZeroPage => panic!("sty zero page not implemented"),
            STY::ZeroPageX => panic!("sty zero page x not implemented"),
            STY::Absolute => VecDeque::from([fetch_low_operand, fetch_high_operand, sty_absolute]),
        }
    }
}

fn sty_absolute(state: &mut State) {
    let data = state.registers.y_index;

    state.cycle_data.acting_data = data;
    write_to_effective_absolute_address(state);
}
