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
pub enum ROL {
    Accumulator,
    ZeroPage,
    ZeroPageX,
    Absolute,
    AbsoluteX,
}

impl Operation for ROL {
    fn get_events(&self) -> VecDeque<Event> {
        match *self {
            ROL::Accumulator => panic!("rol accumulator not implemented"),
            ROL::ZeroPageX => VecDeque::from([
                fetch_effective_zero_page_address,
                do_effective_zero_page_address_x_index,
                read_from_effective_address,
                rol,
                write_to_effective_address,
            ]),
            ROL::ZeroPage => VecDeque::from([
                fetch_effective_zero_page_address,
                read_from_effective_address,
                rol,
                write_to_effective_address,
            ]),
            ROL::Absolute => VecDeque::from([
                fetch_low_effective_address_byte,
                fetch_high_effective_address_byte,
                read_from_effective_address,
                rol,
                write_to_effective_address,
            ]),
            ROL::AbsoluteX => panic!("rol absolute x not implemented"),
        }
    }
}

fn rol(state: &mut State) {
    let address = concat_u8!(
        state.cycle_data.effective_address.0,
        state.cycle_data.effective_address.1
    );
    let data = state.cycle_data.acting_data;
    state.write_to_memory(address, data);

    let result = (data << 1) & (state.get_carry_flag() as u8);
    state.cycle_data.acting_data = result;

    state.set_carry_flag((data & 0b_1000_0000) != 0);
    state.set_zero_flag(result == 0);
    state.set_negative_flag((result & 0b_1000_0000) != 0);
}
