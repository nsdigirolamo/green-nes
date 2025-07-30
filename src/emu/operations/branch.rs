use crate::emu::{
    half_cycles::{branch_across_page, get_effective_address, read_opcode},
    state::State,
};

pub fn do_branch(state: &mut State, condition: bool) {
    let offset = state.read_from_memory(state.address_bus);

    if condition {
        let (pc_high, pc_low) = state.program_counter;
        let (pc_low_offset, overflow) = pc_low.overflowing_add_signed(offset as i8);

        state.effective_address = (pc_high, pc_low_offset);
        state.crossed_page = overflow;

        state
            .cycle_queue
            .push_back([get_effective_address, read_opcode]);

        if state.crossed_page {
            state
                .cycle_queue
                .push_back([branch_across_page, read_opcode]);
        } else {
            state.program_counter = (pc_high, pc_low_offset);
        }
    }
}

pub fn bcs(state: &mut State) {
    do_branch(state, state.get_carry_flag());
}

pub fn bcc(state: &mut State) {
    do_branch(state, !state.get_carry_flag())
}

pub fn beq(state: &mut State) {
    do_branch(state, state.get_zero_flag())
}

pub fn bne(state: &mut State) {
    do_branch(state, !state.get_zero_flag())
}

pub fn bmi(state: &mut State) {
    do_branch(state, state.get_negative_flag())
}

pub fn bpl(state: &mut State) {
    do_branch(state, !state.get_negative_flag())
}

pub fn bvs(state: &mut State) {
    do_branch(state, state.get_overflow_flag())
}

pub fn bvc(state: &mut State) {
    do_branch(state, !state.get_overflow_flag())
}
