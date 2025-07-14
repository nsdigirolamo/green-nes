use crate::emu::state::State;

pub fn asl(state: &mut State) {
    let data = state.read_from_memory(state.address_bus);
    let result = data << 1;

    state.set_carry_flag((data & 0b_1000_0000) != 0);
    state.set_zero_flag(result == 0);
    state.set_negative_flag((result & 0b_1000_0000) != 0);
    state.write_to_memory(state.address_bus, result);
}

pub fn asl_accumulator(state: &mut State) {
    let data = state.accumulator;
    let result = data << 1;

    state.set_carry_flag((data & 0b_1000_0000) != 0);
    state.set_zero_flag(result == 0);
    state.set_negative_flag((result & 0b_1000_0000) != 0);
    state.accumulator = result;
}

pub fn lsr(state: &mut State) {
    let data = state.read_from_memory(state.address_bus);
    let result = data >> 1;

    state.set_carry_flag((data & 0b_0000_0001) != 0);
    state.set_zero_flag(result == 0);
    state.set_negative_flag(false);
    state.write_to_memory(state.address_bus, result);
}

pub fn lsr_accumulator(state: &mut State) {
    let data = state.accumulator;
    let result = data >> 1;

    state.set_carry_flag((data & 0b_0000_0001) != 0);
    state.set_zero_flag(result == 0);
    state.set_negative_flag(false);
    state.accumulator = result;
}

pub fn rol(state: &mut State) {
    let data = state.read_from_memory(state.address_bus);
    let result = (data << 1) | (state.get_carry_flag() as u8);

    state.set_carry_flag((data & 0b_1000_0000) != 0);
    state.set_zero_flag(result == 0);
    state.set_negative_flag((result & 0b_1000_0000) != 0);
    state.write_to_memory(state.address_bus, result);
}

pub fn rol_accumulator(state: &mut State) {
    let data = state.accumulator;
    let result = (data << 1) | (state.get_carry_flag() as u8);

    state.set_carry_flag((data & 0b_1000_0000) != 0);
    state.set_zero_flag(result == 0);
    state.set_negative_flag((result & 0b_1000_0000) != 0);
    state.accumulator = result;
}

pub fn ror(state: &mut State) {
    let data = state.read_from_memory(state.address_bus);
    let masked_data = data & 0b_1111_1110;
    let result = (masked_data | state.get_carry_flag() as u8).rotate_right(1);

    state.set_carry_flag((data & 0b_0000_0001) != 0);
    state.set_zero_flag(result == 0);
    state.set_negative_flag((result & 0b_1000_0000) != 0);
    state.write_to_memory(state.address_bus, result);
}

pub fn ror_accumulator(state: &mut State) {
    let data = state.accumulator;
    let masked_data = data & 0b_1111_1110;
    let result = (masked_data | state.get_carry_flag() as u8).rotate_right(1);

    state.set_carry_flag((data & 0b_0000_0001) != 0);
    state.set_zero_flag(result == 0);
    state.set_negative_flag((result & 0b_1000_0000) != 0);
    state.accumulator = result;
}
