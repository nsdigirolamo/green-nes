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
            LDA::ZeroPageX => VecDeque::from([
                fetch_low_operand,
                get_effective_zero_page_x_indexed_address,
                lda_zero_page_x_indexed,
            ]),
            LDA::ZeroPage => VecDeque::from([fetch_low_operand, lda_zero_page]),
            LDA::Absolute => VecDeque::from([fetch_low_operand, fetch_high_operand, lda_absolute]),
            LDA::AbsoluteX => panic!("lda absolute x not implemented"),
            LDA::AbsoluteY => panic!("lda absolute y not implemented"),
            LDA::IndirectX => panic!("lda indirect x not implemented"),
            LDA::IndirectY => panic!("lda indirect y not implemented"),
        }
    }
}

fn lda(state: &mut State) {
    let data = state.cycle_data.acting_data;
    state.registers.accumulator = data;
    state.set_zero_flag(data == 0);
    state.set_negative_flag(data >> 7 == 1);
}

fn lda_absolute(state: &mut State) {
    read_at_effective_absolute_address(state);
    lda(state);
}

fn lda_zero_page(state: &mut State) {
    read_at_effective_zero_page_address(state);
    lda(state);
}

fn lda_zero_page_x_indexed(state: &mut State) {
    read_at_effective_zero_page_x_indexed_address(state);
    lda(state);
}
