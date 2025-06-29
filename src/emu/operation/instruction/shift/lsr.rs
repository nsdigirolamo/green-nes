use std::collections::VecDeque;

use crate::emu::{
    Event,
    operation::{
        Operation,
        addressing::{
            read_at_effective_absolute_address, read_at_effective_zero_page_address,
            write_to_effective_absolute_address, write_to_effective_zero_page_address,
        },
        instruction::{fetch_high_operand, fetch_low_operand},
    },
    state::State,
};

#[derive(Debug)]
pub enum LSR {
    Accumulator,
    ZeroPage,
    ZeroPageX,
    Absolute,
    AbsoluteX,
}

impl Operation for LSR {
    fn get_events(&self) -> VecDeque<Event> {
        match *self {
            LSR::Accumulator => panic!("lsr accumulator not implemented"),
            LSR::ZeroPageX => panic!("lsr zero page x not implemented"),
            LSR::ZeroPage => VecDeque::from([
                fetch_low_operand,
                fetch_high_operand,
                read_at_effective_zero_page_address,
                lsr,
                write_to_effective_zero_page_address,
            ]),
            LSR::Absolute => VecDeque::from([
                fetch_low_operand,
                fetch_high_operand,
                read_at_effective_absolute_address,
                lsr,
                write_to_effective_absolute_address,
            ]),
            LSR::AbsoluteX => panic!("lsr absolute x not implemented"),
        }
    }
}

fn lsr(state: &mut State) {
    let address = state.cycle_data.effective_address;
    let data = state.cycle_data.acting_data;
    state.write_to_memory(address, data);

    let result = data >> 1;
    state.cycle_data.acting_data = result;

    state.set_carry_flag((data & 0b_0000_0001) != 0);
    state.set_zero_flag(result == 0);
    state.set_negative_flag(false);
}
