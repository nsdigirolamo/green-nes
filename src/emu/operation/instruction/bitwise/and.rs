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
pub enum AND {
    Immediate,
    ZeroPage,
    ZeroPageX,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    IndirectX,
    IndirectY,
}

impl Operation for AND {
    fn get_events(&self) -> VecDeque<Event> {
        match *self {
            AND::Immediate => panic!("and immediate not implemented"),
            AND::ZeroPageX => VecDeque::from([
                fetch_effective_zero_page_address,
                do_effective_zero_page_address_x_index,
                and,
            ]),
            AND::ZeroPage => VecDeque::from([fetch_effective_zero_page_address, and]),
            AND::Absolute => VecDeque::from([
                fetch_low_effective_address_byte,
                fetch_high_effective_address_byte,
                and,
            ]),
            AND::AbsoluteX => panic!("and absolute x not implemented"),
            AND::AbsoluteY => panic!("and absolute y not implemented"),
            AND::IndirectX => panic!("and indirect x not implemented"),
            AND::IndirectY => panic!("and indirect y not implemented"),
        }
    }
}

fn and(state: &mut State) {
    read_from_effective_address(state);

    let data = state.cycle_data.acting_data;
    state.registers.accumulator &= data;
    state.set_zero_flag(data == 0);
    state.set_negative_flag((data & 0b_1000_0000) != 0);
}
