use crate::{concat_u8, emu::state::State};

fn get_effective_absolute_address(state: &mut State) {
    let low_address_byte = state.cycle_data.low_operand;
    let high_address_byte = state.cycle_data.high_operand;

    state.cycle_data.effective_address = concat_u8!(high_address_byte, low_address_byte);
}

pub fn read_at_effective_absolute_address(state: &mut State) {
    get_effective_absolute_address(state);
    let address = state.cycle_data.effective_address;
    let data = state.read_from_memory(address);

    state.cycle_data.acting_data = data;
}

pub fn write_to_effective_absolute_address(state: &mut State) {
    get_effective_absolute_address(state);
    let address = state.cycle_data.effective_address;
    let data = state.cycle_data.acting_data;

    state.write_to_memory(address, data);
}
