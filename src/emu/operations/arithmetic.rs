use crate::{did_signed_overflow, emu::state::State};

pub fn adc(state: &mut State) {
    let data = state.read_from_memory(state.address_bus);
    state.data_bus = data;

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

pub fn dec(state: &mut State) {
    let data = state.read_from_memory(state.address_bus);
    let result = data.wrapping_sub(1);

    state.data_bus = result;
    state.set_zero_flag(result == 0);
    state.set_negative_flag((result & 0b_1000_0000) != 0);
}

pub fn inc(state: &mut State) {
    let data = state.read_from_memory(state.address_bus);
    let result = data.wrapping_add(1);

    state.data_bus = result;
    state.set_zero_flag(result == 0);
    state.set_negative_flag((result & 0b_1000_0000) != 0);
}

pub fn sbc(state: &mut State) {
    let data = state.read_from_memory(state.address_bus);
    state.data_bus = data;

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
