use crate::{
    concat_u8,
    emu::{buses::Buses, cpu::CPU},
    split_u16,
};

pub mod operations;

const STACK_PAGE_HIGH_ADDRESS: u8 = 0x01;

pub type HalfCycle = fn(&mut CPU, &mut Buses);

/// Loads the program counter onto the address bus and then increments the
/// program counter.
pub fn get_pc_with_inc(cpu: &mut CPU, buses: &mut Buses) {
    buses.addr = cpu.registers.pc;
    let new_pc = concat_u8!(cpu.registers.pc.0, cpu.registers.pc.1).wrapping_add(1);
    cpu.registers.pc = split_u16!(new_pc);
}

/// Loads the program counter onto the address bus.
pub fn get_pc(cpu: &mut CPU, buses: &mut Buses) {
    buses.addr = cpu.registers.pc;
}

/// Loads the stack pointer onto the address bus.
pub fn get_sp(cpu: &mut CPU, buses: &mut Buses) {
    buses.addr = (STACK_PAGE_HIGH_ADDRESS, cpu.registers.sp);
}

/// Loads the stack pointer onto the address bus and then decrements the stack
/// pointer.
pub fn push_stack(cpu: &mut CPU, buses: &mut Buses) {
    buses.addr = (STACK_PAGE_HIGH_ADDRESS, cpu.registers.sp);
    cpu.registers.sp = cpu.registers.sp.wrapping_sub(1);
}

/// Loads the stack pointer onto the address bus and then increments the stack
/// pointer.
pub fn pop_stack(cpu: &mut CPU, buses: &mut Buses) {
    buses.addr = (STACK_PAGE_HIGH_ADDRESS, cpu.registers.sp);
    cpu.registers.sp = cpu.registers.sp.wrapping_add(1);
}

/// Loads the effective address onto the address bus.
pub fn get_effective_addr(cpu: &mut CPU, buses: &mut Buses) {
    buses.addr = cpu.buses.effective_addr;
}

/// Loads the effective zero page address onto the address bus.
pub fn get_effective_zero_page_addr(cpu: &mut CPU, buses: &mut Buses) {
    buses.addr = (0x00, cpu.buses.effective_addr.1);
}

/// Loads the base zero page address onto the address bus.
pub fn get_base_zero_page_addr(cpu: &mut CPU, buses: &mut Buses) {
    buses.addr = (0x00, cpu.buses.base_addr.1);
}

/// Loads the base zero page address onto the address bus, offset by the
/// X index register.
pub fn get_base_zero_page_x_indexed_addr(cpu: &mut CPU, buses: &mut Buses) {
    buses.addr = (
        0x00,
        cpu.buses.base_addr.1.wrapping_add(cpu.registers.x_index),
    );
}

/// Loads the base zero page address onto the address bus, offset by the X index
/// register plus one.
///
/// Used in indirect X addressing to retrieve the high order byte of the
/// effective address.
pub fn get_base_zero_page_x_indexed_addr_high_byte(cpu: &mut CPU, buses: &mut Buses) {
    buses.addr = (
        0x00,
        cpu.buses
            .base_addr
            .1
            .wrapping_add(cpu.registers.x_index)
            .wrapping_add(1),
    );
}

/// Loads the base zero page address onto the address bus, offset by the Y index
/// register.
pub fn get_base_zero_page_y_indexed_addr(cpu: &mut CPU, buses: &mut Buses) {
    buses.addr = (
        0x00,
        cpu.buses.base_addr.1.wrapping_add(cpu.registers.y_index),
    );
}

/// Loads the indirect address onto the address bus.
pub fn get_indirect_addr(cpu: &mut CPU, buses: &mut Buses) {
    buses.addr = (cpu.buses.indirect_addr.0, cpu.buses.indirect_addr.1);
}

/// Loads the indirect address plus one onto the address bus.
///
/// Used in the indirect jump instruction to retrieve the high order byte of the
/// indirect address.
pub fn get_indirect_addr_high_byte(cpu: &mut CPU, buses: &mut Buses) {
    buses.addr = (
        cpu.buses.indirect_addr.0,
        cpu.buses.indirect_addr.1.wrapping_add(1),
    );
}

/// Loads the indirect zero page address onto the address bus.
pub fn get_indirect_zero_page_addr(cpu: &mut CPU, buses: &mut Buses) {
    buses.addr = (0x00, cpu.buses.indirect_addr.1);
}

/// Loads the indirect zero page address plus one onto the address bus.
///
/// Used in indirect Y addressing to retrieve the high order byte of the
/// effective address.
pub fn get_indirect_zero_page_addr_high_byte(cpu: &mut CPU, buses: &mut Buses) {
    buses.addr = (0x00, cpu.buses.indirect_addr.1.wrapping_add(1));
}

/// Loads the base address offset by the Y index register onto both the address
/// bus and the effective address.
pub fn get_indirect_y_indexed_addr(cpu: &mut CPU, buses: &mut Buses) {
    let new_effective_addr = (
        cpu.buses.base_addr.0,
        cpu.buses.base_addr.1.wrapping_add(cpu.registers.y_index),
    );

    cpu.buses.effective_addr = new_effective_addr;
    buses.addr = new_effective_addr;
}

/// Adds the X index register to the base address, storing the result in the
/// address bus and effective address.
pub fn get_base_addr_x_indexed_with_carry(cpu: &mut CPU, buses: &mut Buses) {
    let (base_address_high, base_address_low) = cpu.buses.base_addr;
    let (effective_address_low, overflow) = base_address_low.overflowing_add(cpu.registers.x_index);
    let effective_address_high = base_address_high.wrapping_add(overflow as u8);

    buses.addr = (base_address_high, effective_address_low);
    cpu.buses.effective_addr = (effective_address_high, effective_address_low);
    cpu.crossed_page = overflow;
}

/// Adds the Y index register to the base address, storing the result in the
/// address bus and effective address.
pub fn get_base_addr_y_indexed_with_carry(cpu: &mut CPU, buses: &mut Buses) {
    let (base_address_high, base_address_low) = cpu.buses.base_addr;
    let (effective_address_low, overflow) = base_address_low.overflowing_add(cpu.registers.y_index);
    let effective_address_high = base_address_high.wrapping_add(overflow as u8);

    buses.addr = (base_address_high, effective_address_low);
    cpu.buses.effective_addr = (effective_address_high, effective_address_low);
    cpu.crossed_page = overflow;
}

/// Loads the IRQ vector's low byte onto the address bus.
pub fn get_irq_vector_low_byte(_: &mut CPU, buses: &mut Buses) {
    buses.addr = (0xFF, 0xFE);
}

/// Loads the IRQ vector's high byte onto the address bus.
pub fn get_irq_vector_high_byte(_: &mut CPU, buses: &mut Buses) {
    buses.addr = (0xFF, 0xFF);
}

/// Loads the NMI vector's low byte onto the address bus.
pub fn get_nmi_vector_low_byte(_: &mut CPU, buses: &mut Buses) {
    buses.addr = (0xFF, 0xFA);
}

/// Loads the NMI vector's high byte onto the address bus.
pub fn get_nmi_vector_high_byte(_: &mut CPU, buses: &mut Buses) {
    buses.addr = (0xFF, 0xFB)
}

/// Reads the byte addressed by the address bus and places it onto both the data
/// bus and the instruction register.
pub fn read_opcode(cpu: &mut CPU, buses: &mut Buses) {
    cpu.registers.ir = buses.read();
}

/// Reads the byte addressed by the address bus and places it onto both the data
/// bus and the program counter high byte.
pub fn read_pc_high_byte(cpu: &mut CPU, buses: &mut Buses) {
    cpu.registers.pc.0 = buses.read();
}

/// Reads the byte addressed by the address bus and places it onto both the data
/// bus and the program counter low byte.
pub fn read_pc_low_byte(cpu: &mut CPU, buses: &mut Buses) {
    cpu.registers.pc.1 = buses.read();
}

/// Reads the byte addressed by the address bus and places it onto both the data
/// bus and the effective address high byte.
pub fn read_effective_addr_high_byte(cpu: &mut CPU, buses: &mut Buses) {
    cpu.buses.effective_addr.0 = buses.read();
}

/// Reads the byte addressed by the address bus and places it onto both the data
/// bus and the effective address low byte.
pub fn read_effective_addr_low_byte(cpu: &mut CPU, buses: &mut Buses) {
    cpu.buses.effective_addr.1 = buses.read();
}

/// Reads the byte addressed by the address bus and places it onto both the data
/// bus and the base address high byte.
pub fn read_base_addr_high_byte(cpu: &mut CPU, buses: &mut Buses) {
    cpu.buses.base_addr.0 = buses.read();
}

/// Reads the byte addressed by the address bus and places it onto both the data
/// bus and the base address low byte.
pub fn read_base_addr_low_byte(cpu: &mut CPU, buses: &mut Buses) {
    cpu.buses.base_addr.1 = buses.read();
}

/// Reads the byte addressed by the address bus and places it onto both the data
/// bus and the indirect address high byte.
pub fn read_indirect_addr_high_byte(cpu: &mut CPU, buses: &mut Buses) {
    cpu.buses.indirect_addr.0 = buses.read();
}

/// Reads the byte addressed by the address bus and places it onto both the data
/// bus and the indirect address low byte.
pub fn read_indirect_addr_low_byte(cpu: &mut CPU, buses: &mut Buses) {
    cpu.buses.indirect_addr.1 = buses.read();
}

/// Reads the byte addressed by the address bus and places it onto the data bus.
pub fn read_data(_: &mut CPU, buses: &mut Buses) {
    buses.read();
}

/// Writes the byte on the data bus into the memory location addressed by the
/// address bus.
pub fn write_data(_: &mut CPU, buses: &mut Buses) {
    buses.write(buses.data);
}

/// Writes the program counter high byte into the memory location addressed by
/// the address bus.
pub fn write_pc_high_byte(cpu: &mut CPU, buses: &mut Buses) {
    buses.write(cpu.registers.pc.0);
}

/// Writes the program counter low byte into the memory location addressed by
/// the address bus.
pub fn write_pc_low_byte(cpu: &mut CPU, buses: &mut Buses) {
    buses.write(cpu.registers.pc.1);
}

/// Writes the processor status register byte into the memory location addressed
/// by the address bus, then sets the interrupt disable flag.
pub fn write_break_status(cpu: &mut CPU, buses: &mut Buses) {
    buses.write(cpu.registers.psr | 0b_0011_0000);
    cpu.registers.psr.set_interrupt_disable(true);
}
