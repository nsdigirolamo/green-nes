use crate::{
    cycle,
    emu::{
        addressing::{
            fix_high_effective_address_byte_absolute_indexed, read_from_effective_address,
        },
        state::State,
    },
};

pub fn lda(state: &mut State) {
    let data = state.cycle_data.acting_data;
    state.registers.accumulator = data;
    state.set_zero_flag(data == 0);
    state.set_negative_flag(data >> 7 == 1);
}

pub fn lda_absolute_indexed(state: &mut State) {
    lda(state);
    if state.cycle_data.crossed_page {
        fix_high_effective_address_byte_absolute_indexed(state);
        state
            .cycle_queue
            .push_back(cycle![read_from_effective_address, lda]);
    }
}

pub fn ldx(state: &mut State) {
    let data = state.cycle_data.acting_data;
    state.registers.x_index = data;
    state.set_zero_flag(data == 0);
    state.set_negative_flag(data >> 7 == 1);
}

pub fn ldx_absolute_indexed(state: &mut State) {
    ldx(state);
    if state.cycle_data.crossed_page {
        fix_high_effective_address_byte_absolute_indexed(state);
        state
            .cycle_queue
            .push_back(cycle![read_from_effective_address, ldx]);
    }
}

pub fn ldy(state: &mut State) {
    let data = state.cycle_data.acting_data;
    state.registers.y_index = data;
    state.set_zero_flag(data == 0);
    state.set_negative_flag(data >> 7 == 1);
}

pub fn ldy_absolute_indexed(state: &mut State) {
    ldy(state);
    if state.cycle_data.crossed_page {
        fix_high_effective_address_byte_absolute_indexed(state);
        state.cycle_queue.push_back(cycle![ldy]);
    }
}

pub fn sta(state: &mut State) {
    let data = state.registers.accumulator;
    state.cycle_data.acting_data = data;
}

pub fn stx(state: &mut State) {
    let data = state.registers.x_index;
    state.cycle_data.acting_data = data;
}

pub fn sty(state: &mut State) {
    let data = state.registers.y_index;
    state.cycle_data.acting_data = data;
}
