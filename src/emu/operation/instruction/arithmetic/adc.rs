use std::collections::VecDeque;

use crate::{
    did_signed_overflow,
    emu::{
        Event,
        operation::{
            Operation,
            instruction::{
                do_effective_zero_page_address_x_index, fetch_effective_zero_page_address,
                fetch_high_effective_address_byte, fetch_low_effective_address_byte,
                read_from_effective_address,
            },
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
                fetch_effective_zero_page_address,
                do_effective_zero_page_address_x_index,
                adc,
            ]),
            ADC::ZeroPage => VecDeque::from([fetch_effective_zero_page_address, adc]),
            ADC::Absolute => VecDeque::from([
                fetch_low_effective_address_byte,
                fetch_high_effective_address_byte,
                adc,
            ]),
            ADC::AbsoluteX => panic!("adc absolute x not implemented"),
            ADC::AbsoluteY => panic!("adc absolute y not implemented"),
            ADC::IndirectX => panic!("adc indirect x not implemented"),
            ADC::IndirectY => panic!("adc indirect y not implemented"),
        }
    }
}

fn adc(state: &mut State) {
    read_from_effective_address(state);

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
