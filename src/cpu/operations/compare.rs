use crate::cpu::{half_cycles::get_effective_address, state::State};

pub fn cmp(state: &mut State) {
    let data = state.read_from_memory(state.buses.addr);
    let result = state.registers.a.wrapping_sub(data);

    state.set_carry_flag(state.registers.a >= data);
    state.set_zero_flag(state.registers.a == data);
    state.set_negative_flag((result & 0b_1000_0000) != 0);
}

pub fn cmp_indirect_y(state: &mut State) {
    cmp(state);

    if state.abstracts.crossed_page {
        state
            .abstracts
            .cycle_queue
            .push_back([get_effective_address, cmp]);
    }
}

pub fn cmp_absolute_indexed(state: &mut State) {
    cmp(state);

    if state.abstracts.crossed_page {
        state
            .abstracts
            .cycle_queue
            .push_back([get_effective_address, cmp]);
    }
}

pub fn cpx(state: &mut State) {
    let data = state.read_from_memory(state.buses.addr);
    let result = state.registers.x_index.wrapping_sub(data);

    state.set_carry_flag(state.registers.x_index >= data);
    state.set_zero_flag(state.registers.x_index == data);
    state.set_negative_flag((result & 0b_1000_0000) != 0);
}

pub fn cpy(state: &mut State) {
    let data = state.read_from_memory(state.buses.addr);
    let result = state.registers.y_index.wrapping_sub(data);

    state.set_carry_flag(state.registers.y_index >= data);
    state.set_zero_flag(state.registers.y_index == data);
    state.set_negative_flag((result & 0b_1000_0000) != 0);
}
