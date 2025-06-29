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
pub enum ORA {
    Immediate,
    ZeroPage,
    ZeroPageX,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    IndirectX,
    IndirectY,
}

impl Operation for ORA {
    fn get_events(&self) -> VecDeque<Event> {
        match *self {
            ORA::Immediate => panic!("ora immediate not implemented"),
            ORA::ZeroPageX => VecDeque::from([
                fetch_low_operand,
                get_effective_zero_page_x_indexed_address,
                ora_zero_page_x_indexed,
            ]),
            ORA::ZeroPage => VecDeque::from([fetch_low_operand, ora_zero_page]),
            ORA::Absolute => VecDeque::from([fetch_low_operand, fetch_high_operand, ora_absolute]),
            ORA::AbsoluteX => panic!("ora absolute x not implemented"),
            ORA::AbsoluteY => panic!("ora absolute y not implemented"),
            ORA::IndirectX => panic!("ora indirect x not implemented"),
            ORA::IndirectY => panic!("ora indirect y not implemented"),
        }
    }
}

fn ora(state: &mut State) {
    let data = state.cycle_data.acting_data;
    state.registers.accumulator |= data;
    state.set_zero_flag(data == 0);
    state.set_negative_flag((data & 0b_1000_0000) != 0);
}

fn ora_absolute(state: &mut State) {
    read_at_effective_absolute_address(state);
    ora(state);
}

fn ora_zero_page(state: &mut State) {
    read_at_effective_zero_page_address(state);
    ora(state);
}

fn ora_zero_page_x_indexed(state: &mut State) {
    read_at_effective_zero_page_x_indexed_address(state);
    ora(state);
}
