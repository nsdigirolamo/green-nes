use crate::emu::state::State;

pub fn lda(state: &mut State) {
    let data = state.read_from_memory(state.address_bus);
    state.data_bus = data;

    state.registers.accumulator = data;
    state.set_zero_flag(data == 0);
    state.set_negative_flag(data >> 7 == 1);
}

pub fn ldx(state: &mut State) {
    let data = state.read_from_memory(state.address_bus);
    state.data_bus = data;

    state.registers.x_index = data;
    state.set_zero_flag(data == 0);
    state.set_negative_flag(data >> 7 == 1);
}

pub fn ldy(state: &mut State) {
    let data = state.read_from_memory(state.address_bus);
    state.data_bus = data;

    state.registers.y_index = data;
    state.set_zero_flag(data == 0);
    state.set_negative_flag(data >> 7 == 1);
}

pub fn sta(state: &mut State) {
    let data = state.registers.accumulator;
    state.data_bus = data;

    state.write_to_memory(state.address_bus, data);
}

pub fn stx(state: &mut State) {
    let data = state.registers.x_index;
    state.data_bus = data;

    state.write_to_memory(state.address_bus, data);
}

pub fn sty(state: &mut State) {
    let data = state.registers.y_index;
    state.data_bus = data;

    state.write_to_memory(state.address_bus, data);
}
