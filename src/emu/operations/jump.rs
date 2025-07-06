use crate::emu::state::State;

pub fn jsr(state: &mut State) {
    let data = state.read_from_memory(state.address_bus);

    state.effective_address.0 = data;
    state.data_bus = data;
    state.program_counter = state.effective_address;
}

pub fn jmp_absolute(state: &mut State) {
    let data = state.read_from_memory(state.address_bus);

    state.effective_address.0 = data;
    state.data_bus = data;
    state.program_counter = state.effective_address;
}
