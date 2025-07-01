use std::collections::VecDeque;

use crate::emu::{
    Event,
    operation::{
        Operation,
        instruction::{
            fetch_effective_zero_page_address, fetch_high_effective_address_byte,
            fetch_low_effective_address_byte, read_from_effective_address,
        },
    },
    state::State,
};

#[derive(Debug)]
pub enum BIT {
    ZeroPage,
    Absolute,
}

impl Operation for BIT {
    fn get_events(&self) -> VecDeque<Event> {
        match *self {
            BIT::ZeroPage => VecDeque::from([fetch_effective_zero_page_address, bit]),
            BIT::Absolute => VecDeque::from([
                fetch_low_effective_address_byte,
                fetch_high_effective_address_byte,
                bit,
            ]),
        }
    }
}

fn bit(state: &mut State) {
    read_from_effective_address(state);

    let data = state.cycle_data.acting_data;
    let result = state.registers.accumulator & data;
    state.set_zero_flag(result == 0);
    state.set_overflow_flag((data & 0b_0100_0000) != 0);
    state.set_negative_flag((data & 0b_1000_0000) != 0);
}
