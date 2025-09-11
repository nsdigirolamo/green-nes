use crate::cpu::state::State;

pub fn sec(state: &mut State) {
    state.set_carry_flag(true);
}

pub fn clc(state: &mut State) {
    state.set_carry_flag(false);
}

pub fn sed(state: &mut State) {
    state.set_decimal_mode_flag(true);
}

pub fn cld(state: &mut State) {
    state.set_decimal_mode_flag(false);
}

pub fn sei(state: &mut State) {
    state.set_interrupt_disable_flag(true);
}

pub fn cli(state: &mut State) {
    state.set_interrupt_disable_flag(false);
}

pub fn clv(state: &mut State) {
    state.set_overflow_flag(false);
}
