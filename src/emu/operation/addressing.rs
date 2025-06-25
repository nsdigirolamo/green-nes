use crate::{concat_u8, emu::state::State};

pub fn get_absolute_address(state: &State) -> u16 {
    let low_address_byte = state.cycle_data.low_operand;
    let high_address_byte = state.cycle_data.high_operand;

    concat_u8!(high_address_byte, low_address_byte)
}
