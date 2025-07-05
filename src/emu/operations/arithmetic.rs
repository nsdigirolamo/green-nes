use crate::{did_signed_overflow, emu::state::State};

pub fn inc(state: &mut State) {
    let data = state.read_from_memory(state.address_bus);
    let result = data.wrapping_add(1);

    state.data_bus = result;
    state.set_zero_flag(result == 0);
    state.set_negative_flag((result & 0b_1000_0000) != 0);
}

pub fn dec(state: &mut State) {
    let data = state.read_from_memory(state.address_bus);
    let result = data.wrapping_sub(1);

    state.data_bus = result;
    state.set_zero_flag(result == 0);
    state.set_negative_flag((result & 0b_1000_0000) != 0);
}

pub fn adc(state: &mut State) {
    let data = state.read_from_memory(state.address_bus);
    state.data_bus = data;

    let accumulator = state.registers.accumulator;
    let (sum, overflow1) = accumulator.overflowing_add(data);
    let (result, overflow2) = sum.overflowing_add(state.get_carry_flag() as u8);
    let did_unsigned_overflow = overflow1 | overflow2;
    let did_signed_overflow = did_signed_overflow!(accumulator, data, result);

    state.registers.accumulator = result;
    state.set_carry_flag(did_unsigned_overflow);
    state.set_zero_flag(data == 0);
    state.set_overflow_flag(did_signed_overflow);
    state.set_negative_flag((data & 0b_1000_0000) != 0);
}

pub fn sbc(state: &mut State) {
    let data = state.read_from_memory(state.address_bus);
    state.data_bus = data;

    let accumulator = state.registers.accumulator;
    let (sum, overflow1) = accumulator.overflowing_add(!data);
    let (result, overflow2) = sum.overflowing_add(state.get_carry_flag() as u8);
    let did_unsigned_overflow = overflow1 | overflow2;
    let did_signed_overflow = did_signed_overflow!(accumulator, data, result);

    state.registers.accumulator = result;
    state.set_carry_flag(did_unsigned_overflow);
    state.set_zero_flag(data == 0);
    state.set_overflow_flag(did_signed_overflow);
    state.set_negative_flag((data & 0b_1000_0000) != 0);
}
