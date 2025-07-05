use crate::emu::state::State;

pub fn jmp(state: &mut State) {
    let data = state.read_from_memory(state.address_bus);
    state.data_bus = data;

    state.address_high = data;
    state.registers.program_counter = (state.address_high, state.address_low);
}
