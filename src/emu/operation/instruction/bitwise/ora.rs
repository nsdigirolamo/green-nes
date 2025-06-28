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
            ORA::ZeroPageX => panic!("ora zero page x not implemented"),
            ORA::ZeroPage => panic!("ora zero page not implemented"),
            ORA::Absolute => VecDeque::from([fetch_low_operand, fetch_high_operand, ora_absolute]),
            ORA::AbsoluteX => panic!("ora absolute x not implemented"),
            ORA::AbsoluteY => panic!("ora absolute y not implemented"),
            ORA::IndirectX => panic!("ora indirect x not implemented"),
            ORA::IndirectY => panic!("ora indirect y not implemented"),
        }
    }
}

fn ora_absolute(state: &mut State) {
    read_at_effective_absolute_address(state);
    let data = state.cycle_data.acting_data;

    state.registers.accumulator |= data;
    state.set_zero_flag(data == 0);
    state.set_negative_flag(data >> 7 == 1);
}
