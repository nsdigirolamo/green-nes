use crate::cpu::{
    half_cycles::{branch_across_page, get_effective_address, read_opcode},
    state::State,
};

pub fn do_branch(state: &mut State, condition: bool) {
    let offset = state.buses.read(state.buses.addr);

    if condition {
        let (pc_high, pc_low) = state.registers.pc;
        let (pc_low_offset, overflow) = pc_low.overflowing_add_signed(offset as i8);

        state.buses.effective_addr = (pc_high, pc_low_offset);
        state.abstracts.crossed_page = overflow;

        state
            .abstracts
            .cycle_queue
            .push_back([get_effective_address, read_opcode]);

        if state.abstracts.crossed_page {
            state
                .abstracts
                .cycle_queue
                .push_back([branch_across_page, read_opcode]);
        } else {
            state.registers.pc = (pc_high, pc_low_offset);
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
