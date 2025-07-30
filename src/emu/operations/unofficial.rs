use crate::emu::{
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

    if state.crossed_page {
        state.cycle_queue.push_back([get_effective_address, lax]);
    }
}

pub fn lax_absolute_indexed(state: &mut State) {
    lax(state);

    if state.crossed_page {
        state.cycle_queue.push_back([get_effective_address, lax]);
    }
}

pub fn sax(state: &mut State) {
    let data = state.accumulator & state.x_index_register;

    state.write_to_memory(state.address_bus, data);
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
