use crate::emu::{
    half_cycles::{
        get_effective_absolute_address, get_effective_absolute_address_with_carry, read_opcode,
    },
    state::State,
};

pub fn do_branch(state: &mut State) {
    let pc_low_byte = state.program_counter.1;
    let offset = state.read_from_memory(state.address_bus);
    let (offset_low_byte, overflow) = pc_low_byte.overflowing_add_signed(offset as i8);

    state.effective_address = (state.program_counter.0, offset_low_byte);
    state.crossed_page = overflow;
    state
        .cycle_queue
        .push_back([get_effective_absolute_address, read_opcode]);

    if state.crossed_page {
        state
            .cycle_queue
            .push_back([get_effective_absolute_address_with_carry, read_opcode]);
    }
}

pub fn bcs(state: &mut State) {
    if state.get_carry_flag() {
        do_branch(state);
    }
}

pub fn bcc(state: &mut State) {
    if !state.get_carry_flag() {
        do_branch(state);
    }
}

pub fn beq(state: &mut State) {
    if state.get_zero_flag() {
        do_branch(state);
    }
}

pub fn bne(state: &mut State) {
    if !state.get_zero_flag() {
        do_branch(state);
    }
}

pub fn bmi(state: &mut State) {
    if state.get_negative_flag() {
        do_branch(state);
    }
}

pub fn bpl(state: &mut State) {
    if !state.get_negative_flag() {
        do_branch(state);
    }
}

pub fn bvs(state: &mut State) {
    if state.get_overflow_flag() {
        do_branch(state);
    }
}

pub fn bvc(state: &mut State) {
    if !state.get_overflow_flag() {
        do_branch(state);
    }
}
