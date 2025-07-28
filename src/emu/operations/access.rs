use crate::emu::{half_cycles::get_effective_address, state::State};

pub fn lda(state: &mut State) {
    let data = state.read_from_memory(state.address_bus);

    state.accumulator = data;
    state.set_zero_flag(data == 0);
    state.set_negative_flag(data >> 7 == 1);
}

pub fn lda_indirect_y(state: &mut State) {
    lda(state);

    if state.crossed_page {
        state.cycle_queue.push_back([get_effective_address, lda]);
    }
}

pub fn lda_absolute_indexed(state: &mut State) {
    lda(state);

    if state.crossed_page {
        state.cycle_queue.push_back([get_effective_address, lda]);
    }
}

pub fn ldx(state: &mut State) {
    let data = state.read_from_memory(state.address_bus);

    state.x_index_register = data;
    state.set_zero_flag(data == 0);
    state.set_negative_flag(data >> 7 == 1);
}

pub fn ldx_absolute_indexed(state: &mut State) {
    ldx(state);

    if state.crossed_page {
        state.cycle_queue.push_back([get_effective_address, ldx]);
    }
}

pub fn ldy(state: &mut State) {
    let data = state.read_from_memory(state.address_bus);

    state.y_index_register = data;
    state.set_zero_flag(data == 0);
    state.set_negative_flag(data >> 7 == 1);
}

pub fn ldy_absolute_indexed(state: &mut State) {
    ldy(state);

    if state.crossed_page {
        state.cycle_queue.push_back([get_effective_address, ldy]);
    }
}

pub fn lax(state: &mut State) {
    lda(state);
    ldx(state);
}

pub fn lax_indirect_y(state: &mut State) {
    lax(state);

    if state.crossed_page {
        state.cycle_queue.push_back([get_effective_address, lax]);
    }
}

pub fn lax_absolute_indexed(state: &mut State) {
    lax(state);

    if state.crossed_page {
        state.cycle_queue.push_back([get_effective_address, lax]);
    }
}

pub fn sta(state: &mut State) {
    let data = state.accumulator;

    state.write_to_memory(state.address_bus, data);
}

pub fn stx(state: &mut State) {
    let data = state.x_index_register;

    state.write_to_memory(state.address_bus, data);
}

pub fn sty(state: &mut State) {
    let data = state.y_index_register;

    state.write_to_memory(state.address_bus, data);
}
