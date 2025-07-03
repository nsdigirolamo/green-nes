use crate::{
    concat_u8, cycle, did_signed_overflow,
    emu::{
        addressing::{
            fix_high_effective_address_byte_absolute_indexed, read_from_effective_address,
        },
        state::State,
    },
};

pub fn adc(state: &mut State) {
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

pub fn adc_absolute_indexed(state: &mut State) {
    adc(state);
    if state.cycle_data.crossed_page {
        fix_high_effective_address_byte_absolute_indexed(state);
        state
            .cycle_queue
            .push_back(cycle![read_from_effective_address, adc]);
    }
}

pub fn dec(state: &mut State) {
    let address = concat_u8!(
        state.cycle_data.effective_address.0,
        state.cycle_data.effective_address.1
    );
    let data = state.cycle_data.acting_data;
    state.write_to_memory(address, data);

    let result = data - 1;
    state.cycle_data.acting_data = result;

    state.set_zero_flag(result == 0);
    state.set_negative_flag((result & 0b_1000_0000) != 0);
}

pub fn inc(state: &mut State) {
    let address = concat_u8!(
        state.cycle_data.effective_address.0,
        state.cycle_data.effective_address.1
    );
    let data = state.cycle_data.acting_data;
    state.write_to_memory(address, data);

    let result = data + 1;
    state.cycle_data.acting_data = result;

    state.set_zero_flag(result == 0);
    state.set_negative_flag((result & 0b_1000_0000) != 0);
}

pub fn sbc(state: &mut State) {
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

pub fn sbc_absolute_indexed(state: &mut State) {
    sbc(state);
    if state.cycle_data.crossed_page {
        fix_high_effective_address_byte_absolute_indexed(state);
        state
            .cycle_queue
            .push_back(cycle![read_from_effective_address, sbc]);
    }
}
