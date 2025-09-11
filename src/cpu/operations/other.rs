use crate::cpu::{half_cycles::get_effective_address, state::State};

pub fn nop(_: &mut State) {}

pub fn nop_absolute_indexed(state: &mut State) {
    if state.abstracts.crossed_page {
        state
            .abstracts
            .cycle_queue
            .push_back([get_effective_address, nop]);
    }
}
