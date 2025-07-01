use std::collections::VecDeque;

use crate::emu::{
    Event,
    operation::{
        Operation,
        instruction::{
            do_effective_zero_page_address_x_index, fetch_effective_zero_page_address,
            fetch_high_effective_address_byte, fetch_low_effective_address_byte,
            write_to_effective_address,
        },
    },
    state::State,
};

#[derive(Debug)]
pub enum STA {
    ZeroPage,
    ZeroPageX,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    IndirectX,
    IndirectY,
}

impl Operation for STA {
    fn get_events(&self) -> VecDeque<Event> {
        match *self {
            STA::ZeroPageX => VecDeque::from([
                fetch_effective_zero_page_address,
                do_effective_zero_page_address_x_index,
                sta,
            ]),
            STA::ZeroPage => VecDeque::from([fetch_effective_zero_page_address, sta]),
            STA::Absolute => VecDeque::from([
                fetch_low_effective_address_byte,
                fetch_high_effective_address_byte,
                sta,
            ]),
            STA::AbsoluteX => panic!("sta absolute x not implemented"),
            STA::AbsoluteY => panic!("sta absolute y not implemented"),
            STA::IndirectX => panic!("sta indirect x not implemented"),
            STA::IndirectY => panic!("sta indirect y not implemented"),
        }
    }
}

fn sta(state: &mut State) {
    let data = state.registers.accumulator;
    state.cycle_data.acting_data = data;

    write_to_effective_address(state);
}
