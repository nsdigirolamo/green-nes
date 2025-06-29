use std::collections::VecDeque;

use crate::emu::{
    Event,
    operation::{
        Operation,
        addressing::{
            get_effective_zero_page_x_indexed_address, read_at_effective_absolute_address,
            read_at_effective_zero_page_address, read_at_effective_zero_page_x_indexed_address,
        },
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
            LDY::ZeroPageX => VecDeque::from([
                fetch_low_operand,
                get_effective_zero_page_x_indexed_address,
                ldy_zero_page_x_indexed,
            ]),
            LDY::ZeroPage => VecDeque::from([fetch_low_operand, ldy_zero_page]),
            LDY::Absolute => VecDeque::from([fetch_low_operand, fetch_high_operand, ldy_absolute]),
            LDY::AbsoluteX => panic!("ldy absolute x not implemented"),
        }
    }
}

fn ldy(state: &mut State) {
    let data = state.cycle_data.acting_data;
    state.registers.y_index = data;
    state.set_zero_flag(data == 0);
    state.set_negative_flag(data >> 7 == 1);
}

fn ldy_absolute(state: &mut State) {
    read_at_effective_absolute_address(state);
    ldy(state);
}

fn ldy_zero_page(state: &mut State) {
    read_at_effective_zero_page_address(state);
    ldy(state);
}

fn ldy_zero_page_x_indexed(state: &mut State) {
    read_at_effective_zero_page_x_indexed_address(state);
    ldy(state);
}
