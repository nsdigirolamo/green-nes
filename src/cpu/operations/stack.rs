use crate::cpu::state::State;

pub fn pha(state: &mut State) {
    state.mem_write(state.buses.addr, state.registers.a);
}

pub fn php(state: &mut State) {
    state
        .buses
        .write(state.buses.addr, state.registers.psr | 0b_0011_0000);
}

pub fn plp(state: &mut State) {
    // B and extra bit are ignored
    let masked_stack_status = state.mem_read(state.buses.addr) & 0b_1100_1111;
    let masked_processor_status = state.registers.psr & 0b_0011_0000;
    let new_processor_status = masked_stack_status | masked_processor_status;
    // @TODO: The effect of changing the I flag should be delayed by one instruction.
    state.registers.psr = new_processor_status
}

pub fn pla(state: &mut State) {
    let result = state.mem_read(state.buses.addr);

    state.set_zero_flag(result == 0);
    state.set_negative_flag((result & 0b_1000_0000) != 0);
    state.registers.a = result
}
