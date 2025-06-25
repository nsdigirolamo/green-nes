use std::collections::VecDeque;

use crate::emu::{
    Event,
    operation::{
        Operation,
        addressing::get_absolute_address,
        instruction::{fetch_high_operand, fetch_low_operand},
    },
    state::State,
};

#[derive(Debug)]
pub enum LDX {
    Immediate,
    ZeroPage,
    ZeroPageX,
    Absolute,
    AbsoluteY,
}

impl Operation for LDX {
    fn get_events(&self) -> VecDeque<Event> {
        match *self {
            LDX::Immediate => panic!("ldx immediate not implemented"),
            LDX::ZeroPageX => panic!("ldx zero page x not implemented"),
            LDX::ZeroPage => panic!("ldx zero page not implemented"),
            LDX::Absolute => VecDeque::from([fetch_low_operand, fetch_high_operand, ldx_absolute]),
            LDX::AbsoluteY => panic!("ldx absolute y not implemented"),
        }
    }
}

fn ldx_absolute(state: &mut State) {
    let address = get_absolute_address(state);
    let data = state.read_from_memory(address);

    state.registers.x_index = data;
    state.set_zero_flag(data == 0);
    state.set_negative_flag(data >> 7 == 1);
}
