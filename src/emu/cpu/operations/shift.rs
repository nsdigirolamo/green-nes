use crate::emu::cpu::state::State;

pub fn asl(state: &mut State) {
    let data = state.mem_read(state.buses.addr);
    let result = data << 1;

    state.set_carry_flag((data & 0b_1000_0000) != 0);
    state.set_zero_flag(result == 0);
    state.set_negative_flag((result & 0b_1000_0000) != 0);
    state.mem_write(state.buses.addr, result);
}

pub fn asl_accumulator(state: &mut State) {
    let data = state.registers.a;
    let result = data << 1;

    state.set_carry_flag((data & 0b_1000_0000) != 0);
    state.set_zero_flag(result == 0);
    state.set_negative_flag((result & 0b_1000_0000) != 0);
    state.registers.a = result;
}

pub fn lsr(state: &mut State) {
    let data = state.mem_read(state.buses.addr);
    let result = data >> 1;

    state.set_carry_flag((data & 0b_0000_0001) != 0);
    state.set_zero_flag(result == 0);
    state.set_negative_flag(false);
    state.mem_write(state.buses.addr, result);
}

pub fn lsr_accumulator(state: &mut State) {
    let data = state.registers.a;
    let result = data >> 1;

    state.set_carry_flag((data & 0b_0000_0001) != 0);
    state.set_zero_flag(result == 0);
    state.set_negative_flag(false);
    state.registers.a = result;
}

pub fn rol(state: &mut State) {
    let data = state.mem_read(state.buses.addr);
    let result = (data << 1) | (state.get_carry_flag() as u8);

    state.set_carry_flag((data & 0b_1000_0000) != 0);
    state.set_zero_flag(result == 0);
    state.set_negative_flag((result & 0b_1000_0000) != 0);
    state.mem_write(state.buses.addr, result);
}

pub fn rol_accumulator(state: &mut State) {
    let data = state.registers.a;
    let result = (data << 1) | (state.get_carry_flag() as u8);

    state.set_carry_flag((data & 0b_1000_0000) != 0);
    state.set_zero_flag(result == 0);
    state.set_negative_flag((result & 0b_1000_0000) != 0);
    state.registers.a = result;
}

pub fn ror(state: &mut State) {
    let data = state.mem_read(state.buses.addr);
    let masked_data = data & 0b_1111_1110;
    let result = (masked_data | state.get_carry_flag() as u8).rotate_right(1);

    state.set_carry_flag((data & 0b_0000_0001) != 0);
    state.set_zero_flag(result == 0);
    state.set_negative_flag((result & 0b_1000_0000) != 0);
    state.mem_write(state.buses.addr, result);
}

pub fn ror_accumulator(state: &mut State) {
    let data = state.registers.a;
    let masked_data = data & 0b_1111_1110;
    let result = (masked_data | state.get_carry_flag() as u8).rotate_right(1);

    state.set_carry_flag((data & 0b_0000_0001) != 0);
    state.set_zero_flag(result == 0);
    state.set_negative_flag((result & 0b_1000_0000) != 0);
    state.registers.a = result;
}
