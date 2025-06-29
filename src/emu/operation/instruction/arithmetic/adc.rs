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
pub enum ADC {
    Immediate,
    ZeroPage,
    ZeroPageX,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    IndirectX,
    IndirectY,
}

impl Operation for ADC {
    fn get_events(&self) -> VecDeque<Event> {
        match *self {
            ADC::Immediate => panic!("adc immediate not implemented"),
            ADC::ZeroPageX => VecDeque::from([
                fetch_low_operand,
                get_effective_zero_page_x_indexed_address,
                adc_zero_page_x_indexed,
            ]),
            ADC::ZeroPage => VecDeque::from([fetch_low_operand, adc_zero_page]),
            ADC::Absolute => VecDeque::from([fetch_low_operand, fetch_high_operand, adc_absolute]),
            ADC::AbsoluteX => panic!("adc absolute x not implemented"),
            ADC::AbsoluteY => panic!("adc absolute y not implemented"),
            ADC::IndirectX => panic!("adc indirect x not implemented"),
            ADC::IndirectY => panic!("adc indirect y not implemented"),
        }
    }
}

fn adc(state: &mut State) {
    let data = state.cycle_data.acting_data;
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

fn adc_absolute(state: &mut State) {
    read_at_effective_absolute_address(state);
    adc(state);
}

fn adc_zero_page(state: &mut State) {
    read_at_effective_zero_page_address(state);
    adc(state);
}

fn adc_zero_page_x_indexed(state: &mut State) {
    read_at_effective_zero_page_x_indexed_address(state);
    adc(state);
}
