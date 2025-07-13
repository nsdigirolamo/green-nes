use crate::emu::state::State;

pub fn pha(state: &mut State) {
    state.write_to_memory(state.address_bus, state.accumulator);
}

pub fn php(state: &mut State) {
    state.write_to_memory(
        state.address_bus,
        state.processor_status_register | 0b_0011_0000,
    );
}

pub fn plp(state: &mut State) {
    // @TODO: The effect of changing the I flag should be delayed by one instruction.
    state.processor_status_register = state.read_from_memory(state.address_bus);
}

pub fn pla(state: &mut State) {
    let result = state.read_from_memory(state.address_bus);

    state.set_zero_flag(result == 0);
    state.set_negative_flag((result & 0b_1000_0000) != 0);
    state.accumulator = result
}
