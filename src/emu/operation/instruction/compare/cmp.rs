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
pub enum CMP {
    Immediate,
    ZeroPage,
    ZeroPageX,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    IndirectX,
    IndirectY,
}

impl Operation for CMP {
    fn get_events(&self) -> VecDeque<Event> {
        match *self {
            CMP::Immediate => panic!("cmp immediate not implemented"),
            CMP::ZeroPageX => VecDeque::from([
                fetch_effective_zero_page_address,
                do_effective_zero_page_address_x_index,
                cmp,
            ]),
            CMP::ZeroPage => VecDeque::from([fetch_effective_zero_page_address, cmp]),
            CMP::Absolute => VecDeque::from([
                fetch_low_effective_address_byte,
                fetch_high_effective_address_byte,
                cmp,
            ]),
            CMP::AbsoluteX => panic!("cmp absolute x not implemented"),
            CMP::AbsoluteY => panic!("cmp absolute y not implemented"),
            CMP::IndirectX => panic!("cmp indirect x not implemented"),
            CMP::IndirectY => panic!("cmp indirect y not implemented"),
        }
    }
}

fn cmp(state: &mut State) {
    read_from_effective_address(state);

    let data = state.cycle_data.acting_data;
    let (difference, overflow) = state.registers.accumulator.overflowing_sub(data);
    state.set_carry_flag(overflow);
    state.set_zero_flag(difference == 0);
    state.set_negative_flag((difference & 0b_1000_0000) != 0);
}
