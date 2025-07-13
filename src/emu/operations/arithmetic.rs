use crate::{
    did_signed_overflow,
    emu::{half_cycles::get_effective_address, state::State},
};

pub fn inc(state: &mut State) {
    let data = state.read_from_memory(state.address_bus);
    let result = data.wrapping_add(1);

    state.set_zero_flag(result == 0);
    state.set_negative_flag((result & 0b_1000_0000) != 0);
    state.write_to_memory(state.address_bus, result);
}

pub fn inx(state: &mut State) {
    let data = state.x_index_register;
    let result = data.wrapping_add(1);

    state.set_zero_flag(result == 0);
    state.set_negative_flag((result & 0b_1000_0000) != 0);
    state.x_index_register = result;
}

pub fn iny(state: &mut State) {
    let data = state.y_index_register;
    let result = data.wrapping_add(1);

    state.set_zero_flag(result == 0);
    state.set_negative_flag((result & 0b_1000_0000) != 0);
    state.y_index_register = result;
}

pub fn dec(state: &mut State) {
    let data = state.read_from_memory(state.address_bus);
    let result = data.wrapping_sub(1);

    state.set_zero_flag(result == 0);
    state.set_negative_flag((result & 0b_1000_0000) != 0);
    state.write_to_memory(state.address_bus, result);
}

pub fn dex(state: &mut State) {
    let data = state.x_index_register;
    let result = data.wrapping_sub(1);

    state.set_zero_flag(result == 0);
    state.set_negative_flag((result & 0b_1000_0000) != 0);
    state.x_index_register = result;
}

pub fn dey(state: &mut State) {
    let data = state.y_index_register;
    let result = data.wrapping_sub(1);

    state.set_zero_flag(result == 0);
    state.set_negative_flag((result & 0b_1000_0000) != 0);
    state.y_index_register = result;
}

pub fn adc(state: &mut State) {
    let data = state.read_from_memory(state.address_bus);
    let accumulator = state.accumulator;
    let (sum, overflow1) = accumulator.overflowing_add(data);
    let (result, overflow2) = sum.overflowing_add(state.get_carry_flag() as u8);
    let did_unsigned_overflow = overflow1 | overflow2;
    let did_signed_overflow = did_signed_overflow!(accumulator, data, result);

    state.accumulator = result;
    state.set_carry_flag(did_unsigned_overflow);
    state.set_zero_flag(result == 0);
    state.set_overflow_flag(did_signed_overflow);
    state.set_negative_flag((result & 0b_1000_0000) != 0);
}

pub fn adc_indirect_y(state: &mut State) {
    adc(state);

    if state.crossed_page {
        state.cycle_queue.push_back([get_effective_address, adc]);
    }
}

pub fn adc_absolute_indexed(state: &mut State) {
    adc(state);

    if state.crossed_page {
        state.cycle_queue.push_back([get_effective_address, adc]);
    }
}

pub fn sbc(state: &mut State) {
    let data = state.read_from_memory(state.address_bus);
    let accumulator = state.accumulator;
    let (sum, overflow1) = accumulator.overflowing_add(!data);
    let (result, overflow2) = sum.overflowing_add(state.get_carry_flag() as u8);
    let did_unsigned_overflow = overflow1 | overflow2;
    let did_signed_overflow = did_signed_overflow!(accumulator, !data, result);

    state.accumulator = result;
    state.set_carry_flag(did_unsigned_overflow);
    state.set_zero_flag(result == 0);
    state.set_overflow_flag(did_signed_overflow);
    state.set_negative_flag((result & 0b_1000_0000) != 0);
}

pub fn sbc_indirect_y(state: &mut State) {
    sbc(state);

    if state.crossed_page {
        state.cycle_queue.push_back([get_effective_address, sbc]);
    }
}

pub fn sbc_absolute_indexed(state: &mut State) {
    sbc(state);

    if state.crossed_page {
        state.cycle_queue.push_back([get_effective_address, sbc]);
    }
}
