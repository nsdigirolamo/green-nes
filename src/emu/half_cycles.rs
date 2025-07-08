use crate::emu::state::State;

pub fn get_pc_address(state: &mut State) {
    state.address_bus = state.program_counter;
    state.increment_pc();
}

pub fn get_pc_without_increment(state: &mut State) {
    state.address_bus = state.program_counter;
}

pub fn get_sp_address(state: &mut State) {
    state.address_bus = (0x10, state.stack_pointer);
}

pub fn push_stack(state: &mut State) {
    state.address_bus = (0x10, state.stack_pointer);
    state.stack_pointer = state.stack_pointer.wrapping_sub(1);
}

pub fn pop_stack(state: &mut State) {
    state.address_bus = (0x10, state.stack_pointer);
    state.stack_pointer = state.stack_pointer.wrapping_add(1);
}

pub fn get_effective_absolute_address(state: &mut State) {
    state.address_bus = state.effective_address;
}

pub fn get_effective_absolute_address_with_carry(state: &mut State) {
    state.effective_address.1 = state.effective_address.1.wrapping_add(1);
    state.address_bus = state.effective_address;
}

pub fn get_effective_zero_page_address(state: &mut State) {
    state.address_bus = (0x00, state.effective_address.1);
}

pub fn get_base_zero_page_address(state: &mut State) {
    state.address_bus = (0x00, state.base_address.1);
}

pub fn get_effective_zero_page_x_indexed_address(state: &mut State) {
    let low_base_address = state.base_address.1;
    let x_index = state.x_index_register;

    state.address_bus = (0x00, low_base_address.wrapping_add(x_index));
}

pub fn get_effective_zero_page_y_indexed_address(state: &mut State) {
    let low_base_address = state.base_address.1;
    let y_index = state.y_index_register;

    state.address_bus = (0x00, low_base_address.wrapping_add(y_index));
}

pub fn get_absolute_x_indexed_address(state: &mut State) {
    let base_address = state.base_address;
    let x_index = state.x_index_register;
    let (low_effective_address_byte, overflow) = base_address.1.overflowing_add(x_index);
    let high_effective_address_byte = base_address.0.wrapping_add(overflow as u8);

    /*
     * atarihq.com/danb/files/64doc.txt has different cycle descriptions from
     * the MCS 6500 Family Hardware Manual. 64doc says the address bus could be
     * potentially invalid, while the hardware manual implies the effective
     * address lines should be valid.
     */
    state.effective_address = (high_effective_address_byte, low_effective_address_byte);
    state.address_bus = (base_address.0, low_effective_address_byte)
}

pub fn get_absolute_y_indexed_address(state: &mut State) {
    let base_address = state.base_address;
    let y_index = state.y_index_register;
    let (low_effective_address_byte, overflow) = base_address.1.overflowing_add(y_index);
    let high_effective_address_byte = base_address.0.wrapping_add(overflow as u8);

    /*
     * atarihq.com/danb/files/64doc.txt has different cycle descriptions from
     * the MCS 6500 Family Hardware Manual. 64doc says the address bus could be
     * potentially invalid, while the hardware manual implies the effective
     * address lines should be valid.
     */
    state.effective_address = (high_effective_address_byte, low_effective_address_byte);
    state.address_bus = (base_address.0, low_effective_address_byte)
}

pub fn get_indirect_x_indexed_low_address_byte(state: &mut State) {
    let low_base_address = state.base_address.1;
    let x_index = state.x_index_register;
    let x_indirect_low_byte_address = low_base_address.wrapping_add(x_index);

    state.address_bus = (0x00, x_indirect_low_byte_address);
}

pub fn get_indirect_x_indexed_high_address_byte(state: &mut State) {
    let low_base_address = state.base_address.1;
    let x_index = state.x_index_register;
    let x_indirect_high_byte_address = low_base_address.wrapping_add(x_index).wrapping_add(1);

    state.address_bus = (0x00, x_indirect_high_byte_address);
}

pub fn get_indirect_zero_page_low_address_byte(state: &mut State) {
    state.address_bus = (0x00, state.indirect_address.1);
}

pub fn get_indirect_zero_page_high_address_byte(state: &mut State) {
    state.address_bus = (0x00, state.indirect_address.1.wrapping_add(1));
}

pub fn get_indirect_y_indexed_address(state: &mut State) {
    let low_effective_address = state.base_address.1.wrapping_add(state.y_index_register);
    let high_effective_address = state.base_address.0;

    state.effective_address = (high_effective_address, low_effective_address);
    state.address_bus = (state.base_address.0, low_effective_address)
}

pub fn get_indirect_y_indexed_address_with_carry(state: &mut State) {
    let (low_effective_address, overflow) =
        state.base_address.1.overflowing_add(state.y_index_register);
    let high_effective_address = state.base_address.0.wrapping_add(overflow as u8);

    state.effective_address = (high_effective_address, low_effective_address);
    state.crossed_page = overflow;
}

pub fn read_opcode(state: &mut State) {
    let data = state.read_from_memory(state.address_bus);

    state.data_bus = data;
    state.instruction_register = data;
}

pub fn read_high_effective_address_byte(state: &mut State) {
    let data = state.read_from_memory(state.address_bus);

    state.data_bus = data;
    state.effective_address.0 = data;
}

pub fn read_low_effective_address_byte(state: &mut State) {
    let data = state.read_from_memory(state.address_bus);

    state.data_bus = data;
    state.effective_address.1 = data;
}

pub fn read_high_base_address_byte(state: &mut State) {
    let data = state.read_from_memory(state.address_bus);

    state.data_bus = data;
    state.base_address.0 = data;
}

pub fn read_low_base_address_byte(state: &mut State) {
    let data = state.read_from_memory(state.address_bus);

    state.data_bus = data;
    state.base_address.1 = data;
}

pub fn read_high_indirect_address_byte(state: &mut State) {
    let data = state.read_from_memory(state.address_bus);

    state.data_bus = data;
    state.indirect_address.0 = data;
}

pub fn read_low_indirect_address_byte(state: &mut State) {
    let data = state.read_from_memory(state.address_bus);

    state.data_bus = data;
    state.indirect_address.1 = data;
}

pub fn read_data(state: &mut State) {
    state.read_from_memory(state.address_bus);
}

pub fn write_data(state: &mut State) {
    state.write_to_memory(state.address_bus, state.data_bus);
}

pub fn write_pc_high(state: &mut State) {
    state.write_to_memory(state.address_bus, state.program_counter.0);
}

pub fn write_pc_low(state: &mut State) {
    state.write_to_memory(state.address_bus, state.program_counter.1);
}
