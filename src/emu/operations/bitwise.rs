use crate::emu::{operations::get_effective_absolute_address, state::State};

pub fn and(state: &mut State) {
    let data = state.read_from_memory(state.address_bus);
    state.data_bus = data;

    let result = state.registers.accumulator & data;

    state.registers.accumulator = result;
    state.set_zero_flag(data == 0);
    state.set_negative_flag((data & 0b_1000_0000) != 0);
}

pub fn bit(state: &mut State) {
    let data = state.read_from_memory(state.address_bus);
    state.data_bus = data;

    let result = state.registers.accumulator & data;

    state.set_zero_flag(result == 0);
    state.set_overflow_flag((data & 0b_0100_0000) != 0);
    state.set_negative_flag((data & 0b_1000_0000) != 0);
}

pub fn eor(state: &mut State) {
    let data = state.read_from_memory(state.address_bus);
    state.data_bus = data;

    let result = state.registers.accumulator ^ data;

    state.registers.accumulator = result;
    state.set_zero_flag(data == 0);
    state.set_negative_flag((data & 0b_1000_0000) != 0);
}

pub fn ora(state: &mut State) {
    let data = state.read_from_memory(state.address_bus);
    state.data_bus = data;

    let result = state.registers.accumulator | data;

    state.registers.accumulator = result;
    state.set_zero_flag(data == 0);
    state.set_negative_flag((data & 0b_1000_0000) != 0);
}

pub fn and_absolute_indexed(state: &mut State) {
    and(state);
    if state.crossed_page {
        state.address_high = state.address_high.wrapping_add(1);
        state
            .cycle_queue
            .push_back([get_effective_absolute_address, and]);
    }
}

pub fn bit_absolute_indexed(state: &mut State) {
    bit(state);
    if state.crossed_page {
        state.address_high = state.address_high.wrapping_add(1);
        state
            .cycle_queue
            .push_back([get_effective_absolute_address, bit]);
    }
}

pub fn eor_absolute_indexed(state: &mut State) {
    eor(state);
    if state.crossed_page {
        state.address_high = state.address_high.wrapping_add(1);
        state
            .cycle_queue
            .push_back([get_effective_absolute_address, eor]);
    }
}

pub fn ora_absolute_indexed(state: &mut State) {
    ora(state);
    if state.crossed_page {
        state.address_high = state.address_high.wrapping_add(1);
        state
            .cycle_queue
            .push_back([get_effective_absolute_address, ora]);
    }
}
