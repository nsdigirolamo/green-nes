use crate::emu::{buses::Buses, cpu::CPU};

pub mod operations;

const STACK_PAGE_HIGH_ADDRESS: u8 = 0x01;

pub type HalfCycle = fn(&mut CPU, &mut Buses);

pub fn get_pc(cpu: &mut CPU, buses: &mut Buses) {
    buses.addr = cpu.registers.pc;
    cpu.increment_pc();
}

pub fn get_pc_without_increment(cpu: &mut CPU, buses: &mut Buses) {
    buses.addr = cpu.registers.pc;
}

pub fn get_sp(cpu: &mut CPU, buses: &mut Buses) {
    buses.addr = (STACK_PAGE_HIGH_ADDRESS, cpu.registers.sp);
}

pub fn push_stack(cpu: &mut CPU, buses: &mut Buses) {
    let new_stack_pointer = cpu.registers.sp.wrapping_sub(1);

    buses.addr = (STACK_PAGE_HIGH_ADDRESS, cpu.registers.sp);
    cpu.registers.sp = new_stack_pointer;
}

pub fn pop_stack(cpu: &mut CPU, buses: &mut Buses) {
    let new_stack_pointer = cpu.registers.sp.wrapping_add(1);

    buses.addr = (STACK_PAGE_HIGH_ADDRESS, cpu.registers.sp);
    cpu.registers.sp = new_stack_pointer;
}

pub fn get_effective_address(cpu: &mut CPU, buses: &mut Buses) {
    buses.addr = cpu.buses.effective_addr;
}

pub fn branch_across_page(cpu: &mut CPU, buses: &mut Buses) {
    let new_effective_address = (
        cpu.buses.effective_addr.0.wrapping_add(1),
        cpu.buses.effective_addr.1,
    );

    cpu.buses.effective_addr = new_effective_address;
    buses.addr = new_effective_address;
    cpu.registers.pc = new_effective_address;
}

pub fn get_effective_zero_page_address(cpu: &mut CPU, buses: &mut Buses) {
    buses.addr = (0x00, cpu.buses.effective_addr.1);
}

pub fn get_base_zero_page_address(cpu: &mut CPU, buses: &mut Buses) {
    buses.addr = (0x00, cpu.buses.base_addr.1);
}

pub fn get_effective_zero_page_x_indexed_address(cpu: &mut CPU, buses: &mut Buses) {
    let low_base_address = cpu.buses.base_addr.1;
    let x_index = cpu.registers.x_index;

    buses.addr = (0x00, low_base_address.wrapping_add(x_index));
}

pub fn get_effective_zero_page_y_indexed_address(cpu: &mut CPU, buses: &mut Buses) {
    let low_base_address = cpu.buses.base_addr.1;
    let y_index = cpu.registers.y_index;

    buses.addr = (0x00, low_base_address.wrapping_add(y_index));
}

pub fn get_indirect_x_indexed_low_address_byte(cpu: &mut CPU, buses: &mut Buses) {
    let low_base_address = cpu.buses.base_addr.1;
    let x_index = cpu.registers.x_index;
    let x_indirect_low_byte_address = low_base_address.wrapping_add(x_index);

    buses.addr = (0x00, x_indirect_low_byte_address);
}

pub fn get_indirect_x_indexed_high_address_byte(cpu: &mut CPU, buses: &mut Buses) {
    let low_base_address = cpu.buses.base_addr.1;
    let x_index = cpu.registers.x_index;
    let x_indirect_high_byte_address = low_base_address.wrapping_add(x_index).wrapping_add(1);

    buses.addr = (0x00, x_indirect_high_byte_address);
}

pub fn get_indirect_low_address_byte(cpu: &mut CPU, buses: &mut Buses) {
    buses.addr = (cpu.buses.indirect_addr.0, cpu.buses.indirect_addr.1);
}

pub fn get_indirect_high_address_byte(cpu: &mut CPU, buses: &mut Buses) {
    buses.addr = (
        cpu.buses.indirect_addr.0,
        cpu.buses.indirect_addr.1.wrapping_add(1),
    );
}

pub fn get_indirect_zero_page_low_address_byte(cpu: &mut CPU, buses: &mut Buses) {
    buses.addr = (0x00, cpu.buses.indirect_addr.1);
}

pub fn get_indirect_zero_page_high_address_byte(cpu: &mut CPU, buses: &mut Buses) {
    buses.addr = (0x00, cpu.buses.indirect_addr.1.wrapping_add(1));
}

pub fn get_indirect_y_indexed_address(cpu: &mut CPU, buses: &mut Buses) {
    let low_effective_address = cpu.buses.base_addr.1.wrapping_add(cpu.registers.y_index);
    let high_effective_address = cpu.buses.base_addr.0;

    cpu.buses.effective_addr = (high_effective_address, low_effective_address);
    buses.addr = (cpu.buses.base_addr.0, low_effective_address)
}

/// The address bus high byte is potentially invalid after this half-cycle.
pub fn get_x_indexed_base_address_with_carry(cpu: &mut CPU, buses: &mut Buses) {
    let (base_address_high, base_address_low) = cpu.buses.base_addr;
    let (effective_address_low, overflow) = base_address_low.overflowing_add(cpu.registers.x_index);
    let effective_address_high = base_address_high.wrapping_add(overflow as u8);

    /*
     * atarihq.com/danb/files/64doc.txt says the address bus could potentially
     * be invalid here. The MOS MICROCOMPUTERS hardware manual implies the
     * effective address lines should be valid.
     */
    buses.addr = (base_address_high, effective_address_low);
    cpu.buses.effective_addr = (effective_address_high, effective_address_low);
    cpu.crossed_page = overflow;
}

/// The address bus high byte is potentially invalid after this half-cycle.
pub fn get_y_indexed_base_address_with_carry(cpu: &mut CPU, buses: &mut Buses) {
    let (base_address_high, base_address_low) = cpu.buses.base_addr;
    let (effective_address_low, overflow) = base_address_low.overflowing_add(cpu.registers.y_index);
    let effective_address_high = base_address_high.wrapping_add(overflow as u8);

    /*
     * atarihq.com/danb/files/64doc.txt says the address bus could potentially
     * be invalid here. The MOS MICROCOMPUTERS hardware manual implies the
     * effective address lines should be valid.
     */
    buses.addr = (base_address_high, effective_address_low);
    cpu.buses.effective_addr = (effective_address_high, effective_address_low);
    cpu.crossed_page = overflow;
}

pub fn get_low_irq_vector(_: &mut CPU, buses: &mut Buses) {
    buses.addr = (0xFF, 0xFE);
}

pub fn get_high_irq_vector(_: &mut CPU, buses: &mut Buses) {
    buses.addr = (0xFF, 0xFF);
}

pub fn get_low_nmi_vector(_: &mut CPU, buses: &mut Buses) {
    buses.addr = (0xFF, 0xFA);
}

pub fn get_high_nmi_vector(_: &mut CPU, buses: &mut Buses) {
    buses.addr = (0xFF, 0xFB)
}

pub fn read_opcode(cpu: &mut CPU, buses: &mut Buses) {
    let data = buses.read();

    buses.data = data;
    cpu.registers.ir = data;
}

pub fn read_high_pc_address_byte(cpu: &mut CPU, buses: &mut Buses) {
    let data = buses.read();

    buses.data = data;
    cpu.registers.pc.0 = data;
}

pub fn read_low_pc_address_byte(cpu: &mut CPU, buses: &mut Buses) {
    let data = buses.read();

    buses.data = data;
    cpu.registers.pc.1 = data;
}

pub fn read_high_effective_address_byte(cpu: &mut CPU, buses: &mut Buses) {
    let data = buses.read();

    buses.data = data;
    cpu.buses.effective_addr.0 = data;
}

pub fn read_low_effective_address_byte(cpu: &mut CPU, buses: &mut Buses) {
    let data = buses.read();

    buses.data = data;
    cpu.buses.effective_addr.1 = data;
}

pub fn read_high_base_address_byte(cpu: &mut CPU, buses: &mut Buses) {
    let data = buses.read();

    buses.data = data;
    cpu.buses.base_addr.0 = data;
}

pub fn read_low_base_address_byte(cpu: &mut CPU, buses: &mut Buses) {
    let data = buses.read();

    buses.data = data;
    cpu.buses.base_addr.1 = data;
}

pub fn read_high_indirect_address_byte(cpu: &mut CPU, buses: &mut Buses) {
    let data = buses.read();

    buses.data = data;
    cpu.buses.indirect_addr.0 = data;
}

pub fn read_low_indirect_address_byte(cpu: &mut CPU, buses: &mut Buses) {
    let data = buses.read();

    buses.data = data;
    cpu.buses.indirect_addr.1 = data;
}

pub fn read_data(_: &mut CPU, buses: &mut Buses) {
    buses.read();
}

pub fn write_data(_: &mut CPU, buses: &mut Buses) {
    buses.write(buses.data);
}

pub fn write_pc_high(cpu: &mut CPU, buses: &mut Buses) {
    buses.write(cpu.registers.pc.0);
}

pub fn write_pc_low(cpu: &mut CPU, buses: &mut Buses) {
    buses.write(cpu.registers.pc.1);
}

pub fn write_status(cpu: &mut CPU, buses: &mut Buses) {
    buses.write(cpu.registers.psr);
}
