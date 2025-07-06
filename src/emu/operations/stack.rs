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
    state.processor_status_register = state.read_from_memory(state.address_bus);
}

pub fn pla(state: &mut State) {
    state.accumulator = state.read_from_memory(state.address_bus);
}
