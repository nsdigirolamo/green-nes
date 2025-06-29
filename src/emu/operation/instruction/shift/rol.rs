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
            ROL::ZeroPageX => panic!("rol zero page x not implemented"),
            ROL::ZeroPage => VecDeque::from([
                fetch_low_operand,
                fetch_high_operand,
                read_at_effective_zero_page_address,
                rol,
                write_to_effective_zero_page_address,
            ]),
            ROL::Absolute => VecDeque::from([
                fetch_low_operand,
                fetch_high_operand,
                read_at_effective_absolute_address,
                rol,
                write_to_effective_absolute_address,
            ]),
            ROL::AbsoluteX => panic!("rol absolute x not implemented"),
        }
    }
}

fn rol(state: &mut State) {
    let address = state.cycle_data.effective_address;
    let data = state.cycle_data.acting_data;
    state.write_to_memory(address, data);

    let result = (data << 1) & (state.get_carry_flag() as u8);
    state.cycle_data.acting_data = result;

    state.set_carry_flag((data & 0b_1000_0000) != 0);
    state.set_zero_flag(result == 0);
    state.set_negative_flag((result & 0b_1000_0000) != 0);
}
