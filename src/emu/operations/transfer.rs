use crate::emu::state::State;

pub fn tax(state: &mut State) {
    let result = state.accumulator;

    state.x_index_register = result;
    state.address_bus = state.program_counter;
    state.set_zero_flag(result == 0);
    state.set_negative_flag((result & 0b_1000_0000) != 0);
}

pub fn tay(state: &mut State) {
    let result = state.accumulator;

    state.y_index_register = result;
    state.address_bus = state.program_counter;
    state.set_zero_flag(result == 0);
    state.set_negative_flag((result & 0b_1000_0000) != 0);
}

pub fn tsx(state: &mut State) {
    let result = state.stack_pointer;

    state.x_index_register = result;
    state.address_bus = state.program_counter;
    state.set_zero_flag(result == 0);
    state.set_negative_flag((result & 0b_1000_0000) != 0);
}

pub fn txa(state: &mut State) {
    let result = state.x_index_register;

    state.accumulator = result;
    state.address_bus = state.program_counter;
    state.set_zero_flag(result == 0);
    state.set_negative_flag((result & 0b_1000_0000) != 0);
}

pub fn txs(state: &mut State) {
    state.stack_pointer = state.x_index_register;
    state.address_bus = state.program_counter;
}

pub fn tya(state: &mut State) {
    let result = state.y_index_register;

    state.accumulator = result;
    state.address_bus = state.program_counter;
    state.set_zero_flag(result == 0);
    state.set_negative_flag((result & 0b_1000_0000) != 0);
}
