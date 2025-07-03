use crate::{
    cycle,
    emu::{
        addressing::{
            fix_high_effective_address_byte_absolute_indexed, read_from_effective_address,
        },
        state::State,
    },
};

pub fn cmp(state: &mut State) {
    let data = state.cycle_data.acting_data;
    let (difference, overflow) = state.registers.accumulator.overflowing_sub(data);
    state.set_carry_flag(overflow);
    state.set_zero_flag(difference == 0);
    state.set_negative_flag((difference & 0b_1000_0000) != 0);
}

pub fn cmp_absolute_indexed(state: &mut State) {
    cmp(state);
    if state.cycle_data.crossed_page {
        fix_high_effective_address_byte_absolute_indexed(state);
        state
            .cycle_queue
            .push_back(cycle![read_from_effective_address, cmp]);
    }
}
