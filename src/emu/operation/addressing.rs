use crate::{concat_u8, emu::state::State};

// Absolute Addressing

/**
Gets the effective absolute address, with the expectation that the low and high
operands have already been appropriately set.
*/
fn get_effective_absolute_address(state: &mut State) {
    let low_address_byte = state.cycle_data.low_operand;
    let high_address_byte = state.cycle_data.high_operand;
    state.cycle_data.effective_address = concat_u8!(high_address_byte, low_address_byte);
}

/**
Reads from the effective absolute address, with the expectation that the low and
high operands have already been appropriately set.
*/
pub fn read_at_effective_absolute_address(state: &mut State) {
    get_effective_absolute_address(state);
    let address = state.cycle_data.effective_address;
    let data = state.read_from_memory(address);
    state.cycle_data.acting_data = data;
}

/**
Writes to the effective absolute address, with the expectation that the low and
high operands have already been appropriately set.
*/
pub fn write_to_effective_absolute_address(state: &mut State) {
    get_effective_absolute_address(state);
    let address = state.cycle_data.effective_address;
    let data = state.cycle_data.acting_data;
    state.write_to_memory(address, data);
}

// Zero Page Addressing

/**
Gets the effective zero page address, with the expectation that the low operand
has already been appropriately set.
*/
fn get_effective_zero_page_address(state: &mut State) {
    let low_address_byte = state.cycle_data.low_operand;
    state.cycle_data.effective_address = concat_u8!(0x00, low_address_byte);
}

/**
Reads from the effective zero page address, with the expectation that the low
operand has already been appropriately set.
*/
pub fn read_at_effective_zero_page_address(state: &mut State) {
    get_effective_zero_page_address(state);
    let address = state.cycle_data.effective_address;
    let data = state.read_from_memory(address);
    state.cycle_data.acting_data = data;
}

/**
Writes to the effective zero page address, with the expectation that the low
operand has already been appropriately set.
*/
pub fn write_to_effective_zero_page_address(state: &mut State) {
    get_effective_zero_page_address(state);
    let address = state.cycle_data.effective_address;
    let data = state.cycle_data.acting_data;
    state.write_to_memory(address, data);
}

// Zero Page Indexed Addressing

/**
Gets the effective zero page x-indexed address, with the expectation that the
low operand and x index register have already been appropriately set.
*/
pub fn get_effective_zero_page_x_indexed_address(state: &mut State) {
    let low_address_byte = state.cycle_data.low_operand;
    let zero_page_address = concat_u8!(0x00, low_address_byte);
    state.read_from_memory(zero_page_address); // @TODO: Is this redundant read required?

    let offset = state.registers.x_index;
    let offset_low_address_byte = low_address_byte.wrapping_add(offset);
    state.cycle_data.effective_address = concat_u8!(0x00, offset_low_address_byte);
}

/**
Reads from the effective zero page x-indexed address, with the expectation that
the effective address has already been appropriately set by
`get_effective_zero_page_x_indexed_address`.
*/
pub fn read_at_effective_zero_page_x_indexed_address(state: &mut State) {
    let address = state.cycle_data.effective_address;
    let data = state.read_from_memory(address);
    state.cycle_data.acting_data = data;
}

/**
Writes to the effective zero page x-indexed address, with the expectation that
the effective address has already been appropriately set by
`get_effective_zero_page_x_indexed_address`.
*/
pub fn write_to_effective_zero_page_x_indexed_address(state: &mut State) {
    let address = state.cycle_data.effective_address;
    let data = state.cycle_data.acting_data;
    state.write_to_memory(address, data);
}

/**
Gets the effective zero page y-indexed address, with the expectation that the
low operand and y index register have already been appropriately set.
*/
pub fn get_effective_zero_page_y_indexed_address(state: &mut State) {
    let low_address_byte = state.cycle_data.low_operand;
    let zero_page_address = concat_u8!(0x00, low_address_byte);
    state.read_from_memory(zero_page_address); // @TODO: Is this redundant read required?

    let offset = state.registers.y_index;
    let offset_low_address_byte = low_address_byte.wrapping_add(offset);
    state.cycle_data.effective_address = concat_u8!(0x00, offset_low_address_byte);
}

/**
Reads from the effective zero page y-indexed address, with the expectation that
the effective address has already been appropriately set by
`get_effective_zero_page_y_indexed_address`.
*/
pub fn read_at_effective_zero_page_y_indexed_address(state: &mut State) {
    let address = state.cycle_data.effective_address;
    let data = state.read_from_memory(address);
    state.cycle_data.acting_data = data;
}

/**
Writes to the effective zero page y-indexed address, with the expectation that
the effective address has already been appropriately set by
`get_effective_zero_page_y_indexed_address`.
*/
pub fn write_to_effective_zero_page_y_indexed_address(state: &mut State) {
    let address = state.cycle_data.effective_address;
    let data = state.cycle_data.acting_data;
    state.write_to_memory(address, data);
}
