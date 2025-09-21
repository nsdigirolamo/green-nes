use crate::emu::cpu::state::{STACK_PAGE_HIGH_ADDRESS, State};

pub fn get_pc(state: &mut State) {
    state.buses.addr = state.registers.pc;
    state.increment_pc();
}

pub fn get_pc_without_increment(state: &mut State) {
    state.buses.addr = state.registers.pc;
}

pub fn get_sp(state: &mut State) {
    state.buses.addr = (STACK_PAGE_HIGH_ADDRESS, state.registers.sp);
}

pub fn push_stack(state: &mut State) {
    let new_stack_pointer = state.registers.sp.wrapping_sub(1);

    state.buses.addr = (STACK_PAGE_HIGH_ADDRESS, state.registers.sp);
    state.registers.sp = new_stack_pointer;
}

pub fn pop_stack(state: &mut State) {
    let new_stack_pointer = state.registers.sp.wrapping_add(1);

    state.buses.addr = (STACK_PAGE_HIGH_ADDRESS, state.registers.sp);
    state.registers.sp = new_stack_pointer;
}

pub fn get_effective_address(state: &mut State) {
    state.buses.addr = state.buses.effective_addr;
}

pub fn branch_across_page(state: &mut State) {
    let new_effective_address = (
        state.buses.effective_addr.0.wrapping_add(1),
        state.buses.effective_addr.1,
    );

    state.buses.effective_addr = new_effective_address;
    state.buses.addr = new_effective_address;
    state.registers.pc = new_effective_address;
}

pub fn get_effective_zero_page_address(state: &mut State) {
    state.buses.addr = (0x00, state.buses.effective_addr.1);
}

pub fn get_base_zero_page_address(state: &mut State) {
    state.buses.addr = (0x00, state.buses.base_addr.1);
}

pub fn get_effective_zero_page_x_indexed_address(state: &mut State) {
    let low_base_address = state.buses.base_addr.1;
    let x_index = state.registers.x_index;

    state.buses.addr = (0x00, low_base_address.wrapping_add(x_index));
}

pub fn get_effective_zero_page_y_indexed_address(state: &mut State) {
    let low_base_address = state.buses.base_addr.1;
    let y_index = state.registers.y_index;

    state.buses.addr = (0x00, low_base_address.wrapping_add(y_index));
}

pub fn get_indirect_x_indexed_low_address_byte(state: &mut State) {
    let low_base_address = state.buses.base_addr.1;
    let x_index = state.registers.x_index;
    let x_indirect_low_byte_address = low_base_address.wrapping_add(x_index);

    state.buses.addr = (0x00, x_indirect_low_byte_address);
}

pub fn get_indirect_x_indexed_high_address_byte(state: &mut State) {
    let low_base_address = state.buses.base_addr.1;
    let x_index = state.registers.x_index;
    let x_indirect_high_byte_address = low_base_address.wrapping_add(x_index).wrapping_add(1);

    state.buses.addr = (0x00, x_indirect_high_byte_address);
}

pub fn get_indirect_low_address_byte(state: &mut State) {
    state.buses.addr = (state.buses.indirect_addr.0, state.buses.indirect_addr.1);
}

pub fn get_indirect_high_address_byte(state: &mut State) {
    state.buses.addr = (
        state.buses.indirect_addr.0,
        state.buses.indirect_addr.1.wrapping_add(1),
    );
}

pub fn get_indirect_zero_page_low_address_byte(state: &mut State) {
    state.buses.addr = (0x00, state.buses.indirect_addr.1);
}

pub fn get_indirect_zero_page_high_address_byte(state: &mut State) {
    state.buses.addr = (0x00, state.buses.indirect_addr.1.wrapping_add(1));
}

pub fn get_indirect_y_indexed_address(state: &mut State) {
    let low_effective_address = state
        .buses
        .base_addr
        .1
        .wrapping_add(state.registers.y_index);
    let high_effective_address = state.buses.base_addr.0;

    state.buses.effective_addr = (high_effective_address, low_effective_address);
    state.buses.addr = (state.buses.base_addr.0, low_effective_address)
}

/// The address bus high byte is potentially invalid after this half-cycle.
pub fn get_x_indexed_base_address_with_carry(state: &mut State) {
    let (base_address_high, base_address_low) = state.buses.base_addr;
    let (effective_address_low, overflow) =
        base_address_low.overflowing_add(state.registers.x_index);
    let effective_address_high = base_address_high.wrapping_add(overflow as u8);

    /*
     * atarihq.com/danb/files/64doc.txt says the address bus could potentially
     * be invalid here. The MOS MICROCOMPUTERS hardware manual implies the
     * effective address lines should be valid.
     */
    state.buses.addr = (base_address_high, effective_address_low);
    state.buses.effective_addr = (effective_address_high, effective_address_low);
    state.abstracts.crossed_page = overflow;
}

/// The address bus high byte is potentially invalid after this half-cycle.
pub fn get_y_indexed_base_address_with_carry(state: &mut State) {
    let (base_address_high, base_address_low) = state.buses.base_addr;
    let (effective_address_low, overflow) =
        base_address_low.overflowing_add(state.registers.y_index);
    let effective_address_high = base_address_high.wrapping_add(overflow as u8);

    /*
     * atarihq.com/danb/files/64doc.txt says the address bus could potentially
     * be invalid here. The MOS MICROCOMPUTERS hardware manual implies the
     * effective address lines should be valid.
     */
    state.buses.addr = (base_address_high, effective_address_low);
    state.buses.effective_addr = (effective_address_high, effective_address_low);
    state.abstracts.crossed_page = overflow;
}

pub fn get_low_interrupt_vector(state: &mut State) {
    state.buses.addr = (0xFF, 0xFE);
}

pub fn get_high_interrupt_vector(state: &mut State) {
    state.buses.addr = (0xFF, 0xFF);
}

pub fn read_opcode(state: &mut State) {
    let data = state.mem_read(state.buses.addr);

    state.buses.data = data;
    state.registers.ir = data;
}

pub fn read_high_pc_address_byte(state: &mut State) {
    let data = state.mem_read(state.buses.addr);

    state.buses.data = data;
    state.registers.pc.0 = data;
}

pub fn read_low_pc_address_byte(state: &mut State) {
    let data = state.mem_read(state.buses.addr);

    state.buses.data = data;
    state.registers.pc.1 = data;
}

pub fn read_high_effective_address_byte(state: &mut State) {
    let data = state.mem_read(state.buses.addr);

    state.buses.data = data;
    state.buses.effective_addr.0 = data;
}

pub fn read_low_effective_address_byte(state: &mut State) {
    let data = state.mem_read(state.buses.addr);

    state.buses.data = data;
    state.buses.effective_addr.1 = data;
}

pub fn read_high_base_address_byte(state: &mut State) {
    let data = state.mem_read(state.buses.addr);

    state.buses.data = data;
    state.buses.base_addr.0 = data;
}

pub fn read_low_base_address_byte(state: &mut State) {
    let data = state.mem_read(state.buses.addr);

    state.buses.data = data;
    state.buses.base_addr.1 = data;
}

pub fn read_high_indirect_address_byte(state: &mut State) {
    let data = state.mem_read(state.buses.addr);

    state.buses.data = data;
    state.buses.indirect_addr.0 = data;
}

pub fn read_low_indirect_address_byte(state: &mut State) {
    let data = state.mem_read(state.buses.addr);

    state.buses.data = data;
    state.buses.indirect_addr.1 = data;
}

pub fn read_data(state: &mut State) {
    state.mem_read(state.buses.addr);
}

pub fn write_data(state: &mut State) {
    state.mem_write(state.buses.addr, state.buses.data);
}

pub fn write_pc_high(state: &mut State) {
    state.mem_write(state.buses.addr, state.registers.pc.0);
}

pub fn write_pc_low(state: &mut State) {
    state.mem_write(state.buses.addr, state.registers.pc.1);
}

pub fn write_status(state: &mut State) {
    state.mem_write(state.buses.addr, state.registers.psr);
}
