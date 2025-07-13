use crate::emu::{half_cycles::get_effective_address, state::State};

pub fn cmp(state: &mut State) {
    let data = state.read_from_memory(state.address_bus);
    let result = state.accumulator.wrapping_sub(data);

    state.set_carry_flag(state.accumulator >= data);
    state.set_zero_flag(state.accumulator == data);
    state.set_negative_flag((result & 0b_1000_0000) != 0);
}

pub fn cmp_indirect_y(state: &mut State) {
    cmp(state);

    if state.crossed_page {
        state.cycle_queue.push_back([get_effective_address, cmp]);
    }
}

pub fn cmp_absolute_indexed(state: &mut State) {
    cmp(state);

    if state.crossed_page {
        state.cycle_queue.push_back([get_effective_address, cmp]);
    }
}

pub fn cpx(state: &mut State) {
    let data = state.read_from_memory(state.address_bus);
    let result = state.x_index_register.wrapping_sub(data);

    state.set_carry_flag(state.x_index_register >= data);
    state.set_zero_flag(state.x_index_register == data);
    state.set_negative_flag((result & 0b_1000_0000) != 0);
}

pub fn cpy(state: &mut State) {
    let data = state.read_from_memory(state.address_bus);
    let result = state.y_index_register.wrapping_sub(data);

    state.set_carry_flag(state.y_index_register >= data);
    state.set_zero_flag(state.y_index_register == data);
    state.set_negative_flag((result & 0b_1000_0000) != 0);
}
