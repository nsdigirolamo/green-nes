use crate::emu::state::State;

pub fn cmp(state: &mut State) {
    let data = state.read_from_memory(state.address_bus);
    state.data_bus = data;
    let (difference, overflow) = state.registers.accumulator.overflowing_sub(data);
    state.set_carry_flag(overflow);
    state.set_zero_flag(difference == 0);
    state.set_negative_flag((difference & 0b_1000_0000) != 0);
}
