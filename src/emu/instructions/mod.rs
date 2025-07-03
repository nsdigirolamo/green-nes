use crate::emu::{
    addressing::AbsoluteAddressing,
    instructions::access::lda,
    state::{Cycle, Instruction, State},
};

pub mod access;
pub mod arithmetic;
pub mod bitwise;
pub mod compare;
pub mod jump;
pub mod other;
pub mod shift;

pub fn fetch_opcode(state: &mut State) {
    let data = state.read_from_pc_address();
    state.cycle_data.opcode = data;

    let pc = state.registers.program_counter;
    state.registers.program_counter = pc.wrapping_add(1);
}

pub fn get_operation(_opcode: u8) -> Vec<Cycle> {
    AbsoluteAddressing::Read.get_cycles(lda) // TODO: Replace dummy return
}
