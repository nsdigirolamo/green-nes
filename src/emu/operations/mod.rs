use crate::emu::state::State;

pub mod access;
pub mod arithmetic;
pub mod bitwise;
pub mod compare;
pub mod jump;
pub mod other;
pub mod shift;

pub fn get_pc_address(state: &mut State) {
    state.address_bus = state.registers.program_counter;
}

pub fn get_effective_absolute_address(state: &mut State) {
    state.address_bus = (state.address_high, state.address_low);
}

pub fn get_effective_zero_page_address(state: &mut State) {
    state.address_bus = (0x00, state.address_low);
}

pub fn read_instruction(state: &mut State) {
    let data = state.read_from_memory(state.address_bus);
    state.data_bus = data;
    state.registers.instruction = data;
    state.increment_pc_address();
}

pub fn read_low_address_byte(state: &mut State) {
    let data = state.read_from_memory(state.address_bus);
    state.data_bus = data;
    state.address_low = data;
    state.increment_pc_address();
}

pub fn read_high_address_byte(state: &mut State) {
    let data = state.read_from_memory(state.address_bus);
    state.data_bus = data;
    state.address_high = data;
    state.increment_pc_address();
}

pub fn read_high_address_byte_x_indexed(state: &mut State) {
    let data = state.read_from_memory(state.address_bus);
    state.address_high = data;

    let (data, overflow) = state.address_low.overflowing_add(state.registers.x_index);
    state.address_low = data;
    state.data_bus = data;
    state.crossed_page = overflow;

    state.increment_pc_address();
}

pub fn read_effective_address(state: &mut State) {
    let data = state.read_from_memory(state.address_bus);
    state.data_bus = data;
}

pub fn write_effective_data(state: &mut State) {
    state.write_to_memory(state.address_bus, state.data_bus);
}

pub fn add_x_index_to_address(state: &mut State) {
    let data = state
        .read_from_memory(state.address_bus)
        .wrapping_add(state.registers.x_index);
    state.data_bus = data;
    state.address_low = data;
}

pub fn add_y_index_to_address(state: &mut State) {
    let data = state
        .read_from_memory(state.address_bus)
        .wrapping_add(state.registers.y_index);
    state.data_bus = data;
    state.address_low = data;
}

pub fn fix_high_address_byte(state: &mut State) {
    if state.crossed_page {
        state.address_high = state.address_high.wrapping_add(1);
    }
}
