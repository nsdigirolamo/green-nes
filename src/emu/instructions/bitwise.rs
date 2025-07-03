use crate::{
    cycle,
    emu::{
        addressing::{
            fix_high_effective_address_byte_absolute_indexed, read_from_effective_address,
        },
        state::State,
    },
};

pub fn and(state: &mut State) {
    let data = state.cycle_data.acting_data;
    state.registers.accumulator &= data;
    state.set_zero_flag(data == 0);
    state.set_negative_flag((data & 0b_1000_0000) != 0);
}

pub fn and_absolute_indexed(state: &mut State) {
    and(state);
    if state.cycle_data.crossed_page {
        fix_high_effective_address_byte_absolute_indexed(state);
        state
            .cycle_queue
            .push_back(cycle![read_from_effective_address, and]);
    }
}

pub fn bit(state: &mut State) {
    let data = state.cycle_data.acting_data;
    let result = state.registers.accumulator & data;
    state.set_zero_flag(result == 0);
    state.set_overflow_flag((data & 0b_0100_0000) != 0);
    state.set_negative_flag((data & 0b_1000_0000) != 0);
}

pub fn bit_absolute_indexed(state: &mut State) {
    bit(state);
    if state.cycle_data.crossed_page {
        fix_high_effective_address_byte_absolute_indexed(state);
        state
            .cycle_queue
            .push_back(cycle![read_from_effective_address, bit]);
    }
}

pub fn eor(state: &mut State) {
    let data = state.cycle_data.acting_data;
    state.registers.accumulator ^= data;
    state.set_zero_flag(data == 0);
    state.set_negative_flag((data & 0b_1000_0000) != 0);
}

pub fn eor_absolute_indexed(state: &mut State) {
    eor(state);
    if state.cycle_data.crossed_page {
        fix_high_effective_address_byte_absolute_indexed(state);
        state
            .cycle_queue
            .push_back(cycle![read_from_effective_address, eor]);
    }
}

pub fn ora(state: &mut State) {
    let data = state.cycle_data.acting_data;
    state.registers.accumulator |= data;
    state.set_zero_flag(data == 0);
    state.set_negative_flag((data & 0b_1000_0000) != 0);
}

pub fn ora_absolute_indexed(state: &mut State) {
    ora(state);
    if state.cycle_data.crossed_page {
        fix_high_effective_address_byte_absolute_indexed(state);
        state
            .cycle_queue
            .push_back(cycle![read_from_effective_address, ora]);
    }
}
