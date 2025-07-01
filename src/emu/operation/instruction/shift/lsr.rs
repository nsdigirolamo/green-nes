use std::collections::VecDeque;

use crate::{
    concat_u8,
    emu::{
        Event,
        operation::{
            Operation,
            instruction::{
                do_effective_zero_page_address_x_index, fetch_effective_zero_page_address,
                fetch_high_effective_address_byte, fetch_low_effective_address_byte,
                read_from_effective_address, write_to_effective_address,
            },
        },
        state::State,
    },
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
            LSR::ZeroPageX => VecDeque::from([
                fetch_effective_zero_page_address,
                do_effective_zero_page_address_x_index,
                read_from_effective_address,
                lsr,
                write_to_effective_address,
            ]),
            LSR::ZeroPage => VecDeque::from([
                fetch_effective_zero_page_address,
                read_from_effective_address,
                lsr,
                write_to_effective_address,
            ]),
            LSR::Absolute => VecDeque::from([
                fetch_low_effective_address_byte,
                fetch_high_effective_address_byte,
                read_from_effective_address,
                lsr,
                write_to_effective_address,
            ]),
            LSR::AbsoluteX => panic!("lsr absolute x not implemented"),
        }
    }
}

fn lsr(state: &mut State) {
    let address = concat_u8!(
        state.cycle_data.effective_address.0,
        state.cycle_data.effective_address.1
    );
    let data = state.cycle_data.acting_data;
    state.write_to_memory(address, data);

    let result = data >> 1;
    state.cycle_data.acting_data = result;

    state.set_carry_flag((data & 0b_0000_0001) != 0);
    state.set_zero_flag(result == 0);
    state.set_negative_flag(false);
}
