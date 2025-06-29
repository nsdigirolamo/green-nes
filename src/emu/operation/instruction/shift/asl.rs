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
pub enum ASL {
    Accumulator,
    ZeroPage,
    ZeroPageX,
    Absolute,
    AbsoluteX,
}

impl Operation for ASL {
    fn get_events(&self) -> VecDeque<Event> {
        match *self {
            ASL::Accumulator => panic!("asl accumulator not implemented"),
            ASL::ZeroPageX => VecDeque::from([
                fetch_low_operand,
                get_effective_zero_page_x_indexed_address,
                read_at_effective_zero_page_x_indexed_address,
                asl,
                write_to_effective_zero_page_address,
            ]),
            ASL::ZeroPage => VecDeque::from([
                fetch_low_operand,
                read_at_effective_zero_page_address,
                asl,
                write_to_effective_zero_page_address,
            ]),
            ASL::Absolute => VecDeque::from([
                fetch_low_operand,
                fetch_high_operand,
                read_at_effective_absolute_address,
                asl,
                write_to_effective_absolute_address,
            ]),
            ASL::AbsoluteX => panic!("asl absolute x not implemented"),
        }
    }
}

fn asl(state: &mut State) {
    let address = state.cycle_data.effective_address;
    let data = state.cycle_data.acting_data;
    state.write_to_memory(address, data);

    let result = data << 1;
    state.cycle_data.acting_data = result;

    state.set_carry_flag((data & 0b_1000_0000) != 0);
    state.set_zero_flag(result == 0);
    state.set_negative_flag((result & 0b_1000_0000) != 0);
}
