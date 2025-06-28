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
            STA::ZeroPage => panic!("sta zero page not implemented"),
            STA::Absolute => VecDeque::from([fetch_low_operand, fetch_high_operand, sta_absolute]),
            STA::AbsoluteX => panic!("sta absolute x not implemented"),
            STA::AbsoluteY => panic!("sta absolute y not implemented"),
            STA::IndirectX => panic!("sta indirect x not implemented"),
            STA::IndirectY => panic!("sta indirect y not implemented"),
        }
    }
}

fn sta_absolute(state: &mut State) {
    let data = state.registers.accumulator;

    state.cycle_data.acting_data = data;
    write_to_effective_absolute_address(state);
}
