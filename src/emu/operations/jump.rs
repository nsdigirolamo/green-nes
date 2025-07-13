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

pub fn rti(state: &mut State) {
    // B and extra bit are ignored
    let masked_stack_status = state.read_from_memory(state.address_bus) & 0b_1100_1111;
    let masked_processor_status = state.processor_status_register & 0b_0011_0000;
    let new_processor_status = masked_stack_status | masked_processor_status;

    state.processor_status_register = new_processor_status;
}
