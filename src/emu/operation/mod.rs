use crate::emu::{Operation, operation::absolute::AbsoluteAddressingMode, state::State};

pub mod absolute;

pub enum AddressingMode {
    Implied,
    Relative,
    Immediate,
    Absolute(AbsoluteAddressingMode),
    ZeroPage,
    IndirectAbsolute,
    AbsoluteIndexed,
    ZeroPageIndexed,
    IndirectIndexed,
    IndexedIndirect,
}

pub fn fetch_opcode(state: &mut State) {
    let data = state.read_from_pc_address();
    state.cycle_data.opcode = data;

    let pc = state.registers.program_counter;
    state.registers.program_counter = pc.wrapping_add(1);
}

fn fetch_low_operand(state: &mut State) {
    let data = state.read_from_pc_address();
    state.cycle_data.low_operand = data;

    let pc = state.registers.program_counter;
    state.registers.program_counter = pc.wrapping_add(1);
}

fn fetch_high_operand(state: &mut State) {
    let data = state.read_from_pc_address();
    state.cycle_data.high_operand = data;

    let pc = state.registers.program_counter;
    state.registers.program_counter = pc.wrapping_add(1);
}

pub fn get_operation(_opcode: u8) -> impl Operation {
    AbsoluteAddressingMode::JMP // @TODO: Replace with real values
}
