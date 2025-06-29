use std::collections::VecDeque;

use crate::emu::{
    Event,
    operation::{
        Operation,
        addressing::{
            get_effective_zero_page_y_indexed_address, write_to_effective_absolute_address,
            write_to_effective_zero_page_address, write_to_effective_zero_page_y_indexed_address,
        },
        instruction::{fetch_high_operand, fetch_low_operand},
    },
    state::State,
};

#[derive(Debug)]
pub enum STX {
    ZeroPage,
    ZeroPageY,
    Absolute,
}

impl Operation for STX {
    fn get_events(&self) -> VecDeque<Event> {
        match *self {
            STX::ZeroPage => VecDeque::from([fetch_low_operand, stx_zero_page]),
            STX::ZeroPageY => VecDeque::from([
                fetch_low_operand,
                get_effective_zero_page_y_indexed_address,
                stx_zero_page_y_indexed,
            ]),
            STX::Absolute => VecDeque::from([fetch_low_operand, fetch_high_operand, stx_absolute]),
        }
    }
}

fn stx(state: &mut State) {
    let data = state.registers.x_index;
    state.cycle_data.acting_data = data;
}

fn stx_absolute(state: &mut State) {
    stx(state);
    write_to_effective_absolute_address(state);
}

fn stx_zero_page(state: &mut State) {
    stx(state);
    write_to_effective_zero_page_address(state);
}

fn stx_zero_page_y_indexed(state: &mut State) {
    stx(state);
    write_to_effective_zero_page_y_indexed_address(state);
}
