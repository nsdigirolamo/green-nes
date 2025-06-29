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
                fetch_low_operand,
                get_effective_zero_page_x_indexed_address,
                and_zero_page_x_indexed,
            ]),
            AND::ZeroPage => VecDeque::from([fetch_low_operand, and_zero_page]),
            AND::Absolute => VecDeque::from([fetch_low_operand, fetch_high_operand, and_absolute]),
            AND::AbsoluteX => panic!("and absolute x not implemented"),
            AND::AbsoluteY => panic!("and absolute y not implemented"),
            AND::IndirectX => panic!("and indirect x not implemented"),
            AND::IndirectY => panic!("and indirect y not implemented"),
        }
    }
}

fn and(state: &mut State) {
    let data = state.cycle_data.acting_data;
    state.registers.accumulator &= data;
    state.set_zero_flag(data == 0);
    state.set_negative_flag((data & 0b_1000_0000) != 0);
}

fn and_absolute(state: &mut State) {
    read_at_effective_absolute_address(state);
    and(state);
}

fn and_zero_page(state: &mut State) {
    read_at_effective_zero_page_address(state);
    and(state);
}

fn and_zero_page_x_indexed(state: &mut State) {
    read_at_effective_zero_page_x_indexed_address(state);
    and(state);
}
