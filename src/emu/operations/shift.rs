use crate::emu::state::State;

pub fn asl(state: &mut State) {
    let data = state.read_from_memory(state.address_bus);
    let result = data << 1;

    state.data_bus = result;
    state.set_carry_flag((data & 0b_1000_0000) != 0);
    state.set_zero_flag(result == 0);
    state.set_negative_flag((result & 0b_1000_0000) != 0);
}

pub fn lsr(state: &mut State) {
    let data = state.read_from_memory(state.address_bus);
    let result = data >> 1;

    state.data_bus = result;
    state.set_carry_flag((data & 0b_0000_0001) != 0);
    state.set_zero_flag(result == 0);
    state.set_negative_flag(false);
}

pub fn rol(state: &mut State) {
    let data = state.read_from_memory(state.address_bus);
    let result = (data << 1) & (state.get_carry_flag() as u8);

    state.data_bus = result;
    state.set_carry_flag((data & 0b_1000_0000) != 0);
    state.set_zero_flag(result == 0);
    state.set_negative_flag((result & 0b_1000_0000) != 0);
}

pub fn ror(state: &mut State) {
    let data = state.read_from_memory(state.address_bus);

    let result = (data >> 1) & ((state.get_carry_flag() as u8) << 7);

    state.data_bus = result;
    state.set_carry_flag((data & 0b_0000_0001) != 0);
    state.set_zero_flag(result == 0);
    state.set_negative_flag((result & 0b_1000_0000) != 0);
}
