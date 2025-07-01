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
pub enum STY {
    ZeroPage,
    ZeroPageX,
    Absolute,
}

impl Operation for STY {
    fn get_events(&self) -> VecDeque<Event> {
        match *self {
            STY::ZeroPage => VecDeque::from([fetch_effective_zero_page_address, sty]),
            STY::ZeroPageX => VecDeque::from([
                fetch_effective_zero_page_address,
                do_effective_zero_page_address_x_index,
                sty,
            ]),
            STY::Absolute => VecDeque::from([
                fetch_low_effective_address_byte,
                fetch_high_effective_address_byte,
                sty,
            ]),
        }
    }
}

fn sty(state: &mut State) {
    let data = state.registers.y_index;
    state.cycle_data.acting_data = data;

    write_to_effective_address(state);
}
