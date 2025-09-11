use crate::cpu::state::State;

pub fn tax(state: &mut State) {
    let result = state.registers.a;

    state.registers.x_index = result;
    state.buses.addr = state.registers.pc;
    state.set_zero_flag(result == 0);
    state.set_negative_flag((result & 0b_1000_0000) != 0);
}

pub fn tay(state: &mut State) {
    let result = state.registers.a;

    state.registers.y_index = result;
    state.buses.addr = state.registers.pc;
    state.set_zero_flag(result == 0);
    state.set_negative_flag((result & 0b_1000_0000) != 0);
}

pub fn tsx(state: &mut State) {
    let result = state.registers.sp;

    state.registers.x_index = result;
    state.buses.addr = state.registers.pc;
    state.set_zero_flag(result == 0);
    state.set_negative_flag((result & 0b_1000_0000) != 0);
}

pub fn txa(state: &mut State) {
    let result = state.registers.x_index;

    state.registers.a = result;
    state.buses.addr = state.registers.pc;
    state.set_zero_flag(result == 0);
    state.set_negative_flag((result & 0b_1000_0000) != 0);
}

pub fn txs(state: &mut State) {
    state.registers.sp = state.registers.x_index;
    state.buses.addr = state.registers.pc;
}

pub fn tya(state: &mut State) {
    let result = state.registers.y_index;

    state.registers.a = result;
    state.buses.addr = state.registers.pc;
    state.set_zero_flag(result == 0);
    state.set_negative_flag((result & 0b_1000_0000) != 0);
}
