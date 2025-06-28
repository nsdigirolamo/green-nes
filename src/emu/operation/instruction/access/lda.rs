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
pub enum LDA {
    Immediate,
    ZeroPage,
    ZeroPageX,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    IndirectX,
    IndirectY,
}

impl Operation for LDA {
    fn get_events(&self) -> VecDeque<Event> {
        match *self {
            LDA::Immediate => panic!("lda immediate not implemented"),
            LDA::ZeroPageX => panic!("lda zero page x not implemented"),
            LDA::ZeroPage => panic!("lda zero page not implemented"),
            LDA::Absolute => VecDeque::from([fetch_low_operand, fetch_high_operand, lda_absolute]),
            LDA::AbsoluteX => panic!("lda absolute x not implemented"),
            LDA::AbsoluteY => panic!("lda absolute y not implemented"),
            LDA::IndirectX => panic!("lda indirect x not implemented"),
            LDA::IndirectY => panic!("lda indirect y not implemented"),
        }
    }
}

fn lda_absolute(state: &mut State) {
    read_at_effective_absolute_address(state);
    let data = state.cycle_data.acting_data;

    state.registers.accumulator = data;
    state.set_zero_flag(data == 0);
    state.set_negative_flag(data >> 7 == 1);
}
