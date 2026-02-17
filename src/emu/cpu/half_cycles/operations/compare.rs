use crate::emu::{
    buses::Buses,
    cpu::{CPU, half_cycles::get_effective_addr, registers::flags::Flags},
};

/// # Compare Accumulator
///
/// Compares A to a memory value, setting flags as appropriate.
pub fn cmp(cpu: &mut CPU, buses: &mut Buses) {
    let data = buses.read();
    let result = cpu.registers.a.wrapping_sub(data);

    cpu.registers.psr.set_carry(cpu.registers.a >= data);
    cpu.registers.psr.set_zero(cpu.registers.a == data);
    cpu.registers.psr.set_negative(result & Flags::N != 0);
}

/// # Compare Accumulator
///
/// Compares A to a memory value, setting flags as appropriate. Uses an
/// additional cycle if a page is crossed.
pub fn cmp_indirect_y(cpu: &mut CPU, buses: &mut Buses) {
    cmp(cpu, buses);

    if cpu.crossed_page {
        cpu.cycle_queue.push_back([get_effective_addr, cmp]);
    }
}

/// # Compare Accumulator
///
/// Compares A to a memory value, setting flags as appropriate. Uses an
/// additional cycle if a page is crossed.
pub fn cmp_abs_index(cpu: &mut CPU, buses: &mut Buses) {
    cmp(cpu, buses);

    if cpu.crossed_page {
        cpu.cycle_queue.push_back([get_effective_addr, cmp]);
    }
}

/// # Compare X Register
///
/// Compares X to a memory value, setting flags as appropriate.
pub fn cpx(cpu: &mut CPU, buses: &mut Buses) {
    let data = buses.read();
    let result = cpu.registers.x_index.wrapping_sub(data);

    cpu.registers.psr.set_carry(cpu.registers.x_index >= data);
    cpu.registers.psr.set_zero(cpu.registers.x_index == data);
    cpu.registers.psr.set_negative(result & Flags::N != 0);
}

/// # Compare Y Register
///
/// Compares Y to a memory value, setting flags as appropriate.
pub fn cpy(cpu: &mut CPU, buses: &mut Buses) {
    let data = buses.read();
    let result = cpu.registers.y_index.wrapping_sub(data);

    cpu.registers.psr.set_carry(cpu.registers.y_index >= data);
    cpu.registers.psr.set_zero(cpu.registers.y_index == data);
    cpu.registers.psr.set_negative(result & Flags::N != 0);
}
