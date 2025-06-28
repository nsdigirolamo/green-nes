use std::collections::VecDeque;

use crate::{
    did_signed_overflow,
    emu::{
        Event,
        operation::{
            Operation,
            addressing::read_at_effective_absolute_address,
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
            SBC::ZeroPageX => panic!("sbc zero page x not implemented"),
            SBC::ZeroPage => panic!("sbc zero page not implemented"),
            SBC::Absolute => VecDeque::from([fetch_low_operand, fetch_high_operand, sbc_absolute]),
            SBC::AbsoluteX => panic!("sbc absolute x not implemented"),
            SBC::AbsoluteY => panic!("sbc absolute y not implemented"),
            SBC::IndirectX => panic!("sbc indirect x not implemented"),
            SBC::IndirectY => panic!("sbc indirect y not implemented"),
        }
    }
}

fn sbc_absolute(state: &mut State) {
    read_at_effective_absolute_address(state);
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
    state.set_negative_flag(data >> 7 == 1);
}
