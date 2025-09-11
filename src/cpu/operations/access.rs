use crate::cpu::{half_cycles::get_effective_address, state::State};

pub fn lda(state: &mut State) {
    let data = state.buses.read(state.buses.addr);

    state.registers.a = data;
    state.set_zero_flag(data == 0);
    state.set_negative_flag(data >> 7 == 1);
}

pub fn lda_indirect_y(state: &mut State) {
    lda(state);

    if state.abstracts.crossed_page {
        state
            .abstracts
            .cycle_queue
            .push_back([get_effective_address, lda]);
    }
}

pub fn lda_absolute_indexed(state: &mut State) {
    lda(state);

    if state.abstracts.crossed_page {
        state
            .abstracts
            .cycle_queue
            .push_back([get_effective_address, lda]);
    }
}

pub fn ldx(state: &mut State) {
    let data = state.buses.read(state.buses.addr);

    state.registers.x_index = data;
    state.set_zero_flag(data == 0);
    state.set_negative_flag(data >> 7 == 1);
}

pub fn ldx_absolute_indexed(state: &mut State) {
    ldx(state);

    if state.abstracts.crossed_page {
        state
            .abstracts
            .cycle_queue
            .push_back([get_effective_address, ldx]);
    }
}

pub fn ldy(state: &mut State) {
    let data = state.buses.read(state.buses.addr);

    state.registers.y_index = data;
    state.set_zero_flag(data == 0);
    state.set_negative_flag(data >> 7 == 1);
}

pub fn ldy_absolute_indexed(state: &mut State) {
    ldy(state);

    if state.abstracts.crossed_page {
        state
            .abstracts
            .cycle_queue
            .push_back([get_effective_address, ldy]);
    }
}

pub fn sta(state: &mut State) {
    let data = state.registers.a;

    state.buses.write(state.buses.addr, data);
}

pub fn stx(state: &mut State) {
    let data = state.registers.x_index;

    state.buses.write(state.buses.addr, data);
}

pub fn sty(state: &mut State) {
    let data = state.registers.y_index;

    state.buses.write(state.buses.addr, data);
}
