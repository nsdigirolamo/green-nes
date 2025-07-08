use crate::emu::{half_cycles::get_effective_absolute_address, state::State};

pub fn and(state: &mut State) {
    let data = state.read_from_memory(state.address_bus);
    let result = state.accumulator & data;

    state.accumulator = result;
    state.set_zero_flag(data == 0);
    state.set_negative_flag((data & 0b_1000_0000) != 0);
}

pub fn and_indirect_y(state: &mut State) {
    and(state);

    if state.crossed_page {
        state
            .cycle_queue
            .push_back([get_effective_absolute_address, and]);
    }
}

pub fn bit(state: &mut State) {
    let data = state.read_from_memory(state.address_bus);
    let result = state.accumulator & data;

    state.set_zero_flag(result == 0);
    state.set_overflow_flag((data & 0b_0100_0000) != 0);
    state.set_negative_flag((data & 0b_1000_0000) != 0);
}

pub fn eor(state: &mut State) {
    let data = state.read_from_memory(state.address_bus);
    let result = state.accumulator ^ data;

    state.accumulator = result;
    state.set_zero_flag(data == 0);
    state.set_negative_flag((data & 0b_1000_0000) != 0);
}

pub fn eor_indirect_y(state: &mut State) {
    eor(state);

    if state.crossed_page {
        state
            .cycle_queue
            .push_back([get_effective_absolute_address, eor]);
    }
}

pub fn ora(state: &mut State) {
    let data = state.read_from_memory(state.address_bus);
    let result = state.accumulator | data;

    state.accumulator = result;
    state.set_zero_flag(data == 0);
    state.set_negative_flag((data & 0b_1000_0000) != 0);
}

pub fn ora_indirect_y(state: &mut State) {
    ora(state);

    if state.crossed_page {
        state
            .cycle_queue
            .push_back([get_effective_absolute_address, ora]);
    }
}
