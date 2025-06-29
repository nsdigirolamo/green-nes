use std::collections::VecDeque;

use crate::{
    did_signed_overflow,
    emu::{
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
    },
};

#[derive(Debug)]
pub enum SBC {
    Immediate,
    ZeroPage,
    ZeroPageX,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    IndirectX,
    IndirectY,
}

impl Operation for SBC {
    fn get_events(&self) -> VecDeque<Event> {
        match *self {
            SBC::Immediate => panic!("sbc immediate not implemented"),
            SBC::ZeroPageX => VecDeque::from([
                fetch_low_operand,
                get_effective_zero_page_x_indexed_address,
                sbc_zero_page_x_indexed,
            ]),
            SBC::ZeroPage => VecDeque::from([fetch_low_operand, sbc_zero_page]),
            SBC::Absolute => VecDeque::from([fetch_low_operand, fetch_high_operand, sbc_absolute]),
            SBC::AbsoluteX => panic!("sbc absolute x not implemented"),
            SBC::AbsoluteY => panic!("sbc absolute y not implemented"),
            SBC::IndirectX => panic!("sbc indirect x not implemented"),
            SBC::IndirectY => panic!("sbc indirect y not implemented"),
        }
    }
}

fn sbc(state: &mut State) {
    let data = !state.cycle_data.acting_data;
    let accumulator = state.registers.accumulator;
    let carry = state.get_carry_flag() as u8;

    let (sum1, overflow1) = accumulator.overflowing_add(data);
    let (sum2, overflow2) = sum1.overflowing_add(carry);
    let did_unsigned_overflow = overflow1 | overflow2;
    let did_signed_overflow = did_signed_overflow!(accumulator, data, sum2);

    state.set_carry_flag(did_unsigned_overflow);
    state.set_zero_flag(data == 0);
    state.set_overflow_flag(did_signed_overflow);
    state.set_negative_flag((data & 0b_1000_0000) != 0);
}

fn sbc_absolute(state: &mut State) {
    read_at_effective_absolute_address(state);
    sbc(state);
}

fn sbc_zero_page(state: &mut State) {
    read_at_effective_zero_page_address(state);
    sbc(state);
}

fn sbc_zero_page_x_indexed(state: &mut State) {
    read_at_effective_zero_page_x_indexed_address(state);
    sbc(state);
}
