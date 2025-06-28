use std::collections::VecDeque;

use crate::emu::{
    Event,
    operation::{
        Operation,
        addressing::{read_at_effective_absolute_address, write_to_effective_absolute_address},
        instruction::{fetch_high_operand, fetch_low_operand},
    },
    state::State,
};

#[derive(Debug)]
pub enum INC {
    ZeroPage,
    ZeroPageX,
    Absolute,
    AbsoluteX,
}

impl Operation for INC {
    fn get_events(&self) -> VecDeque<Event> {
        match *self {
            INC::ZeroPageX => panic!("inc zero page x not implemented"),
            INC::ZeroPage => panic!("inc zero page not implemented"),
            INC::Absolute => VecDeque::from([
                fetch_low_operand,
                fetch_high_operand,
                read_at_effective_absolute_address,
                inc,
                write_to_effective_absolute_address,
            ]),
            INC::AbsoluteX => panic!("inc absolute x not implemented"),
        }
    }
}

fn inc(state: &mut State) {
    let address = state.cycle_data.effective_address;
    let data = state.cycle_data.acting_data;
    state.write_to_memory(address, data);

    let result = data + 1;
    state.cycle_data.acting_data = result;

    state.set_zero_flag(result == 0);
    state.set_negative_flag((result & 0b_1000_0000) != 0);
}
