use std::collections::VecDeque;

use crate::emu::{
    Event,
    operation::{
        Operation,
        instruction::{
            do_effective_zero_page_address_x_index, fetch_effective_zero_page_address,
            fetch_high_effective_address_byte, fetch_low_effective_address_byte,
            read_from_effective_address,
        },
    },
    state::State,
};

#[derive(Debug)]
pub enum ORA {
    Immediate,
    ZeroPage,
    ZeroPageX,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    IndirectX,
    IndirectY,
}

impl Operation for ORA {
    fn get_events(&self) -> VecDeque<Event> {
        match *self {
            ORA::Immediate => panic!("ora immediate not implemented"),
            ORA::ZeroPageX => VecDeque::from([
                fetch_effective_zero_page_address,
                do_effective_zero_page_address_x_index,
                ora,
            ]),
            ORA::ZeroPage => VecDeque::from([fetch_effective_zero_page_address, ora]),
            ORA::Absolute => VecDeque::from([
                fetch_low_effective_address_byte,
                fetch_high_effective_address_byte,
                ora,
            ]),
            ORA::AbsoluteX => panic!("ora absolute x not implemented"),
            ORA::AbsoluteY => panic!("ora absolute y not implemented"),
            ORA::IndirectX => panic!("ora indirect x not implemented"),
            ORA::IndirectY => panic!("ora indirect y not implemented"),
        }
    }
}

fn ora(state: &mut State) {
    read_from_effective_address(state);

    let data = state.cycle_data.acting_data;
    state.registers.accumulator |= data;
    state.set_zero_flag(data == 0);
    state.set_negative_flag((data & 0b_1000_0000) != 0);
}
