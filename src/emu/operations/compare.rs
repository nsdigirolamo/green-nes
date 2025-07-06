use crate::emu::{operations::get_effective_absolute_address, state::State};

pub fn cmp(state: &mut State) {
    let data = state.read_from_memory(state.address_bus);
    state.data_bus = data;

    let (difference, overflow) = state.registers.accumulator.overflowing_sub(data);

    state.set_carry_flag(overflow);
    state.set_zero_flag(difference == 0);
    state.set_negative_flag((difference & 0b_1000_0000) != 0);
}

pub fn cmp_absolute_indexed(state: &mut State) {
    cmp(state);
    if state.crossed_page {
        state.address_high = state.address_high.wrapping_add(1);
        state
            .cycle_queue
            .push_back([get_effective_absolute_address, cmp]);
    }
}
