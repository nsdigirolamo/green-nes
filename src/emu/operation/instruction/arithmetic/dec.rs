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
pub enum DEC {
    ZeroPage,
    ZeroPageX,
    Absolute,
    AbsoluteX,
}

impl Operation for DEC {
    fn get_events(&self) -> VecDeque<Event> {
        match *self {
            DEC::ZeroPageX => VecDeque::from([
                fetch_low_operand,
                get_effective_zero_page_x_indexed_address,
                read_at_effective_zero_page_x_indexed_address,
                dec,
                write_to_effective_zero_page_address,
            ]),
            DEC::ZeroPage => VecDeque::from([
                fetch_low_operand,
                read_at_effective_zero_page_address,
                dec,
                write_to_effective_zero_page_address,
            ]),
            DEC::Absolute => VecDeque::from([
                fetch_low_operand,
                fetch_high_operand,
                read_at_effective_absolute_address,
                dec,
                write_to_effective_absolute_address,
            ]),
            DEC::AbsoluteX => panic!("dec absolute x not implemented"),
        }
    }
}

fn dec(state: &mut State) {
    let address = state.cycle_data.effective_address;
    let data = state.cycle_data.acting_data;
    state.write_to_memory(address, data);

    let result = data - 1;
    state.cycle_data.acting_data = result;

    state.set_zero_flag(result == 0);
    state.set_negative_flag((result & 0b_1000_0000) != 0);
}
