use crate::emu::cpu::{half_cycles::get_effective_address, state::State};

pub fn and(state: &mut State) {
    let data = state.mem_read(state.buses.addr);
    let result = state.registers.a & data;

    state.registers.a = result;
    state.set_zero_flag(result == 0);
    state.set_negative_flag((result & 0b_1000_0000) != 0);
}

pub fn and_indirect_y(state: &mut State) {
    and(state);

    if state.abstracts.crossed_page {
        state
            .abstracts
            .cycle_queue
            .push_back([get_effective_address, and]);
    }
}

pub fn and_absolute_indexed(state: &mut State) {
    and(state);

    if state.abstracts.crossed_page {
        state
            .abstracts
            .cycle_queue
            .push_back([get_effective_address, and]);
    }
}

pub fn bit(state: &mut State) {
    let data = state.mem_read(state.buses.addr);
    let result = state.registers.a & data;

    state.set_zero_flag(result == 0);
    state.set_overflow_flag((data & 0b_0100_0000) != 0);
    state.set_negative_flag((data & 0b_1000_0000) != 0);
}

pub fn eor(state: &mut State) {
    let data = state.mem_read(state.buses.addr);
    let result = state.registers.a ^ data;

    state.registers.a = result;
    state.set_zero_flag(result == 0);
    state.set_negative_flag((result & 0b_1000_0000) != 0);
}

pub fn eor_indirect_y(state: &mut State) {
    eor(state);

    if state.abstracts.crossed_page {
        state
            .abstracts
            .cycle_queue
            .push_back([get_effective_address, eor]);
    }
}

pub fn eor_absolute_indexed(state: &mut State) {
    eor(state);

    if state.abstracts.crossed_page {
        state
            .abstracts
            .cycle_queue
            .push_back([get_effective_address, eor]);
    }
}

pub fn ora(state: &mut State) {
    let data = state.mem_read(state.buses.addr);
    let result = state.registers.a | data;

    state.registers.a = result;
    state.set_zero_flag(result == 0);
    state.set_negative_flag((result & 0b_1000_0000) != 0);
}

pub fn ora_indirect_y(state: &mut State) {
    ora(state);

    if state.abstracts.crossed_page {
        state
            .abstracts
            .cycle_queue
            .push_back([get_effective_address, ora]);
    }
}

pub fn ora_absolute_indexed(state: &mut State) {
    ora(state);

    if state.abstracts.crossed_page {
        state
            .abstracts
            .cycle_queue
            .push_back([get_effective_address, ora]);
    }
}
