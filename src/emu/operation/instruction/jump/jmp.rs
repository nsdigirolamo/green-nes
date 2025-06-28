use std::collections::VecDeque;

use crate::{
    concat_u8,
    emu::{
        Event,
        operation::{Operation, instruction::fetch_low_operand},
        state::State,
    },
};

#[derive(Debug)]
pub enum JMP {
    Absolute,
    Indirect,
}

impl Operation for JMP {
    fn get_events(&self) -> VecDeque<Event> {
        match *self {
            JMP::Absolute => VecDeque::from([fetch_low_operand, jmp_absolute]),
            JMP::Indirect => panic!("jmp indirect not implemented"),
        }
    }
}

fn jmp_absolute(state: &mut State) {
    let low_address_byte = state.cycle_data.low_operand;
    let high_address_byte = state.read_from_pc_address();

    state.registers.program_counter = concat_u8!(high_address_byte, low_address_byte)
}
