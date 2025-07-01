use std::collections::VecDeque;

use crate::emu::{
    Event,
    operation::{
        Operation,
        instruction::{
            do_effective_zero_page_address_y_index, fetch_effective_zero_page_address,
            fetch_high_effective_address_byte, fetch_low_effective_address_byte,
            read_from_effective_address,
        },
    },
    state::State,
};

#[derive(Debug)]
pub enum LDX {
    Immediate,
    ZeroPage,
    ZeroPageY,
    Absolute,
    AbsoluteY,
}

impl Operation for LDX {
    fn get_events(&self) -> VecDeque<Event> {
        match *self {
            LDX::Immediate => panic!("ldx immediate not implemented"),
            LDX::ZeroPageY => VecDeque::from([
                fetch_effective_zero_page_address,
                do_effective_zero_page_address_y_index,
                ldx,
            ]),
            LDX::ZeroPage => VecDeque::from([fetch_effective_zero_page_address, ldx]),
            LDX::Absolute => VecDeque::from([
                fetch_low_effective_address_byte,
                fetch_high_effective_address_byte,
                ldx,
            ]),
            LDX::AbsoluteY => panic!("ldx absolute y not implemented"),
        }
    }
}

fn ldx(state: &mut State) {
    read_from_effective_address(state);

    let data = state.cycle_data.acting_data;
    state.registers.x_index = data;
    state.set_zero_flag(data == 0);
    state.set_negative_flag(data >> 7 == 1);
}
