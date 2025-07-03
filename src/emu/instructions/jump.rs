use crate::{concat_u8, emu::state::State};

pub fn jmp_absolute(state: &mut State) {
    let low_address_byte = state.cycle_data.low_operand;
    let high_address_byte = state.read_from_pc_address();

    state.registers.program_counter = concat_u8!(high_address_byte, low_address_byte)
}
