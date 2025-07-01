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
pub enum EOR {
    Immediate,
    ZeroPage,
    ZeroPageX,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    IndirectX,
    IndirectY,
}

impl Operation for EOR {
    fn get_events(&self) -> VecDeque<Event> {
        match *self {
            EOR::Immediate => panic!("eor immediate not implemented"),
            EOR::ZeroPageX => VecDeque::from([
                fetch_effective_zero_page_address,
                do_effective_zero_page_address_x_index,
                eor,
            ]),
            EOR::ZeroPage => VecDeque::from([fetch_effective_zero_page_address, eor]),
            EOR::Absolute => VecDeque::from([
                fetch_low_effective_address_byte,
                fetch_high_effective_address_byte,
                eor,
            ]),
            EOR::AbsoluteX => panic!("eor absolute x not implemented"),
            EOR::AbsoluteY => panic!("eor absolute y not implemented"),
            EOR::IndirectX => panic!("eor indirect x not implemented"),
            EOR::IndirectY => panic!("eor indirect y not implemented"),
        }
    }
}

fn eor(state: &mut State) {
    read_from_effective_address(state);

    let data = state.cycle_data.acting_data;
    state.registers.accumulator ^= data;
    state.set_zero_flag(data == 0);
    state.set_negative_flag((data & 0b_1000_0000) != 0);
}
