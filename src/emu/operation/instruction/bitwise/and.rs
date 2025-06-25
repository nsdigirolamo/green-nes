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
            AND::ZeroPageX => panic!("and zero page x not implemented"),
            AND::ZeroPage => panic!("and zero page not implemented"),
            AND::Absolute => VecDeque::from([fetch_low_operand, fetch_high_operand, and_absolute]),
            AND::AbsoluteX => panic!("and absolute x not implemented"),
            AND::AbsoluteY => panic!("and absolute y not implemented"),
            AND::IndirectX => panic!("and indirect x not implemented"),
            AND::IndirectY => panic!("and indirect y not implemented"),
        }
    }
}

fn and_absolute(state: &mut State) {
    let address = get_absolute_address(state);
    let data = state.read_from_memory(address);

    state.registers.accumulator &= data;
    state.set_zero_flag(data == 0);
    state.set_negative_flag(data >> 7 == 1);
}
