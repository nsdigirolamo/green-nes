use std::collections::VecDeque;

use crate::emu::{
    Event,
    operation::{
        Operation,
        addressing::{read_at_effective_absolute_address, read_at_effective_zero_page_address},
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
            LDX::ZeroPage => VecDeque::from([fetch_low_operand, fetch_high_operand, ldx_zero_page]),
            LDX::Absolute => VecDeque::from([fetch_low_operand, fetch_high_operand, ldx_absolute]),
            LDX::AbsoluteY => panic!("ldx absolute y not implemented"),
        }
    }
}

fn ldx(state: &mut State) {
    let data = state.cycle_data.acting_data;
    state.registers.x_index = data;
    state.set_zero_flag(data == 0);
    state.set_negative_flag(data >> 7 == 1);
}

fn ldx_absolute(state: &mut State) {
    read_at_effective_absolute_address(state);
    ldx(state);
}

fn ldx_zero_page(state: &mut State) {
    read_at_effective_zero_page_address(state);
    ldx(state);
}
