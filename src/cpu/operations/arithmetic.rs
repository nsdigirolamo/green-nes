use crate::{
    cpu::{half_cycles::get_effective_address, state::State},
    did_signed_overflow,
};

pub fn inc(state: &mut State) {
    let data = state.mem_read(state.buses.addr);
    let result = data.wrapping_add(1);

    state.set_zero_flag(result == 0);
    state.set_negative_flag((result & 0b_1000_0000) != 0);
    state.mem_write(state.buses.addr, result);
}

pub fn inx(state: &mut State) {
    let data = state.registers.x_index;
    let result = data.wrapping_add(1);

    state.set_zero_flag(result == 0);
    state.set_negative_flag((result & 0b_1000_0000) != 0);
    state.registers.x_index = result;
}

pub fn iny(state: &mut State) {
    let data = state.registers.y_index;
    let result = data.wrapping_add(1);

    state.set_zero_flag(result == 0);
    state.set_negative_flag((result & 0b_1000_0000) != 0);
    state.registers.y_index = result;
}

pub fn dec(state: &mut State) {
    let data = state.mem_read(state.buses.addr);
    let result = data.wrapping_sub(1);

    state.set_zero_flag(result == 0);
    state.set_negative_flag((result & 0b_1000_0000) != 0);
    state.mem_write(state.buses.addr, result);
}

pub fn dex(state: &mut State) {
    let data = state.registers.x_index;
    let result = data.wrapping_sub(1);

    state.set_zero_flag(result == 0);
    state.set_negative_flag((result & 0b_1000_0000) != 0);
    state.registers.x_index = result;
}

pub fn dey(state: &mut State) {
    let data = state.registers.y_index;
    let result = data.wrapping_sub(1);

    state.set_zero_flag(result == 0);
    state.set_negative_flag((result & 0b_1000_0000) != 0);
    state.registers.y_index = result;
}

pub fn adc(state: &mut State) {
    let data = state.mem_read(state.buses.addr);
    let accumulator = state.registers.a;
    let (sum, overflow1) = accumulator.overflowing_add(data);
    let (result, overflow2) = sum.overflowing_add(state.get_carry_flag() as u8);
    let did_unsigned_overflow = overflow1 | overflow2;
    let did_signed_overflow = did_signed_overflow!(accumulator, data, result);

    state.registers.a = result;
    state.set_carry_flag(did_unsigned_overflow);
    state.set_zero_flag(result == 0);
    state.set_overflow_flag(did_signed_overflow);
    state.set_negative_flag((result & 0b_1000_0000) != 0);
}

pub fn adc_indirect_y(state: &mut State) {
    adc(state);

    if state.abstracts.crossed_page {
        state
            .abstracts
            .cycle_queue
            .push_back([get_effective_address, adc]);
    }
}

pub fn adc_absolute_indexed(state: &mut State) {
    adc(state);

    if state.abstracts.crossed_page {
        state
            .abstracts
            .cycle_queue
            .push_back([get_effective_address, adc]);
    }
}

pub fn sbc(state: &mut State) {
    let data = state.mem_read(state.buses.addr);
    let accumulator = state.registers.a;
    let (sum, overflow1) = accumulator.overflowing_add(!data);
    let (result, overflow2) = sum.overflowing_add(state.get_carry_flag() as u8);
    let did_unsigned_overflow = overflow1 | overflow2;
    let did_signed_overflow = did_signed_overflow!(accumulator, !data, result);

    state.registers.a = result;
    state.set_carry_flag(did_unsigned_overflow);
    state.set_zero_flag(result == 0);
    state.set_overflow_flag(did_signed_overflow);
    state.set_negative_flag((result & 0b_1000_0000) != 0);
}

pub fn sbc_indirect_y(state: &mut State) {
    sbc(state);

    if state.abstracts.crossed_page {
        state
            .abstracts
            .cycle_queue
            .push_back([get_effective_address, sbc]);
    }
}

pub fn sbc_absolute_indexed(state: &mut State) {
    sbc(state);

    if state.abstracts.crossed_page {
        state
            .abstracts
            .cycle_queue
            .push_back([get_effective_address, sbc]);
    }
}
