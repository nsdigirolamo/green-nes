use std::collections::VecDeque;

use crate::emu::{
    Event,
    operation::{
        Operation,
        addressing::{write_to_effective_absolute_address, write_to_effective_zero_page_address},
        instruction::{fetch_high_operand, fetch_low_operand},
    },
    state::State,
};

#[derive(Debug)]
pub enum STA {
    ZeroPage,
    ZeroPageX,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    IndirectX,
    IndirectY,
}

impl Operation for STA {
    fn get_events(&self) -> VecDeque<Event> {
        match *self {
            STA::ZeroPageX => panic!("sta zero page x not implemented"),
            STA::ZeroPage => VecDeque::from([fetch_low_operand, fetch_high_operand, sta_zero_page]),
            STA::Absolute => VecDeque::from([fetch_low_operand, fetch_high_operand, sta_absolute]),
            STA::AbsoluteX => panic!("sta absolute x not implemented"),
            STA::AbsoluteY => panic!("sta absolute y not implemented"),
            STA::IndirectX => panic!("sta indirect x not implemented"),
            STA::IndirectY => panic!("sta indirect y not implemented"),
        }
    }
}

fn sta(state: &mut State) {
    let data = state.registers.accumulator;
    state.cycle_data.acting_data = data;
}

fn sta_absolute(state: &mut State) {
    sta(state);
    write_to_effective_absolute_address(state);
}

fn sta_zero_page(state: &mut State) {
    sta(state);
    write_to_effective_zero_page_address(state);
}
