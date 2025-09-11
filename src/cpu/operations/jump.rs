use crate::cpu::state::State;

pub fn jsr(state: &mut State) {
    let data = state.read_from_memory(state.buses.addr);

    state.buses.effective_addr.0 = data;
    state.buses.data = data;
    state.registers.pc = state.buses.effective_addr;
}

pub fn jmp_absolute(state: &mut State) {
    let data = state.read_from_memory(state.buses.addr);

    state.buses.effective_addr.0 = data;
    state.buses.data = data;
    state.registers.pc = state.buses.effective_addr;
}

pub fn rti(state: &mut State) {
    // B and extra bit are ignored
    let masked_stack_status = state.read_from_memory(state.buses.addr) & 0b_1100_1111;
    let masked_processor_status = state.registers.psr & 0b_0011_0000;
    let new_processor_status = masked_stack_status | masked_processor_status;

    state.registers.psr = new_processor_status;
}
