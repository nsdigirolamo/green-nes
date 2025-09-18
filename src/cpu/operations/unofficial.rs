use crate::cpu::{
    half_cycles::get_effective_address,
    operations::{
        access::{lda, ldx},
        arithmetic::{adc, dec, inc, sbc},
        bitwise::{and, eor, ora},
        compare::cmp,
        shift::{asl, lsr, rol, ror},
    },
    state::State,
};

// Reference: https://www.masswerk.at/nowgobang/2021/6502-illegal-opcodes

pub fn lax(state: &mut State) {
    lda(state);
    ldx(state);
}

pub fn lax_indirect_y(state: &mut State) {
    lax(state);

    if state.abstracts.crossed_page {
        state
            .abstracts
            .cycle_queue
            .push_back([get_effective_address, lax]);
    }
}

pub fn lax_absolute_indexed(state: &mut State) {
    lax(state);

    if state.abstracts.crossed_page {
        state
            .abstracts
            .cycle_queue
            .push_back([get_effective_address, lax]);
    }
}

pub fn sax(state: &mut State) {
    let data = state.registers.a & state.registers.x_index;

    state.mem_write(state.buses.addr, data);
}

pub fn usbc(state: &mut State) {
    sbc(state);
}

pub fn dcp(state: &mut State) {
    dec(state);
    cmp(state);
}

pub fn isc(state: &mut State) {
    inc(state);
    sbc(state);
}

pub fn slo(state: &mut State) {
    asl(state);
    ora(state);
}

pub fn rla(state: &mut State) {
    rol(state);
    and(state);
}

pub fn sre(state: &mut State) {
    lsr(state);
    eor(state);
}

pub fn rra(state: &mut State) {
    ror(state);
    adc(state);
}

pub fn jam(state: &mut State) {
    state.abstracts.is_halted = true;
}
