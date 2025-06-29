use std::collections::VecDeque;

use crate::emu::{
    Event,
    operation::{
        Operation,
        addressing::{
            get_effective_zero_page_x_indexed_address, read_at_effective_absolute_address,
            read_at_effective_zero_page_address, read_at_effective_zero_page_x_indexed_address,
            write_to_effective_absolute_address, write_to_effective_zero_page_address,
        },
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
            INC::ZeroPageX => VecDeque::from([
                fetch_low_operand,
                get_effective_zero_page_x_indexed_address,
                read_at_effective_zero_page_x_indexed_address,
                inc,
                write_to_effective_zero_page_address,
            ]),
            INC::ZeroPage => VecDeque::from([
                fetch_low_operand,
                read_at_effective_zero_page_address,
                inc,
                write_to_effective_zero_page_address,
            ]),
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
