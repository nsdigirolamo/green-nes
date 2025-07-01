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
pub enum LDY {
    Immediate,
    ZeroPage,
    ZeroPageX,
    Absolute,
    AbsoluteX,
}

impl Operation for LDY {
    fn get_events(&self) -> VecDeque<Event> {
        match *self {
            LDY::Immediate => panic!("ldy immediate not implemented"),
            LDY::ZeroPageX => VecDeque::from([
                fetch_effective_zero_page_address,
                do_effective_zero_page_address_x_index,
                ldy,
            ]),
            LDY::ZeroPage => VecDeque::from([fetch_effective_zero_page_address, ldy]),
            LDY::Absolute => VecDeque::from([
                fetch_low_effective_address_byte,
                fetch_high_effective_address_byte,
                ldy,
            ]),
            LDY::AbsoluteX => panic!("ldy absolute x not implemented"),
        }
    }
}

fn ldy(state: &mut State) {
    read_from_effective_address(state);

    let data = state.cycle_data.acting_data;
    state.registers.y_index = data;
    state.set_zero_flag(data == 0);
    state.set_negative_flag(data >> 7 == 1);
}
