use crate::emu::state::State;

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
