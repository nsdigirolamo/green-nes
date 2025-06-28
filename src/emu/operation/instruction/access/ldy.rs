use std::collections::VecDeque;

use crate::emu::{
    Event,
    operation::{
        Operation,
        addressing::read_at_effective_absolute_address,
        instruction::{fetch_high_operand, fetch_low_operand},
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
            LDY::ZeroPageX => panic!("ldy zero page x not implemented"),
            LDY::ZeroPage => panic!("ldy zero page not implemented"),
            LDY::Absolute => VecDeque::from([fetch_low_operand, fetch_high_operand, ldy_absolute]),
            LDY::AbsoluteX => panic!("ldy absolute x not implemented"),
        }
    }
}

fn ldy_absolute(state: &mut State) {
    read_at_effective_absolute_address(state);
    let data = state.cycle_data.acting_data;

    state.registers.y_index = data;
    state.set_zero_flag(data == 0);
    state.set_negative_flag(data >> 7 == 1);
}
