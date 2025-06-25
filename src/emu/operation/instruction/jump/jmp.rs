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
pub enum JMP {
    Absolute,
    Indirect,
}

impl Operation for JMP {
    fn get_events(&self) -> VecDeque<Event> {
        match *self {
            JMP::Absolute => VecDeque::from([fetch_low_operand, fetch_high_operand, jmp_absolute]),
            JMP::Indirect => panic!("jmp indirect not implemented"),
        }
    }
}

fn jmp_absolute(state: &mut State) {
    let address = get_absolute_address(state);

    state.registers.program_counter = address
}
