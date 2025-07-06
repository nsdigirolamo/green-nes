use crate::emu::state::State;

pub fn cmp(state: &mut State) {
    let data = state.read_from_memory(state.address_bus);
    let (result, overflow) = state.accumulator.overflowing_sub(data);

    state.set_carry_flag(overflow);
    state.set_zero_flag(result == 0);
    state.set_negative_flag((result >> 7) == 1);
}

pub fn cpx(state: &mut State) {
    let data = state.read_from_memory(state.address_bus);
    let (result, overflow) = state.x_index_register.overflowing_sub(data);

    state.set_carry_flag(overflow);
    state.set_zero_flag(result == 0);
    state.set_negative_flag((result >> 7) == 1);
}

pub fn cpy(state: &mut State) {
    let data = state.read_from_memory(state.address_bus);
    let (result, overflow) = state.y_index_register.overflowing_sub(data);

    state.set_carry_flag(overflow);
    state.set_zero_flag(result == 0);
    state.set_negative_flag((result >> 7) == 1);
}
