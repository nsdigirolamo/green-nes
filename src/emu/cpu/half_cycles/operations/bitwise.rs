use crate::emu::{
    buses::Buses,
    cpu::{CPU, half_cycles::get_effective_address},
};

/// # Bitwise AND
///
/// Bitwise AND of a memory value and the accumulator.
pub fn and(cpu: &mut CPU, buses: &mut Buses) {
    let data = buses.read();
    let result = cpu.registers.a & data;

    cpu.registers.a = result;
    cpu.set_zero_flag(result == 0);
    cpu.set_negative_flag((result & 0b_1000_0000) != 0);
}

/// # Bitwise AND
///
/// Bitwise AND of a memory value and the accumulator. Uses an additional cycle
/// if a page is crossed.
pub fn and_indirect_y(cpu: &mut CPU, buses: &mut Buses) {
    and(cpu, buses);

    if cpu.crossed_page {
        cpu.cycle_queue.push_back([get_effective_address, and]);
    }
}

/// # Bitwise AND
///
/// Bitwise AND of a memory value and the accumulator. Uses an additional cycle
/// if a page is crossed.
pub fn and_abs_index(cpu: &mut CPU, buses: &mut Buses) {
    and(cpu, buses);

    if cpu.crossed_page {
        cpu.cycle_queue.push_back([get_effective_address, and]);
    }
}

/// # Bit Test
///
/// Modifies flags in the following fashion:
/// * The zero flag is set depending on the result of a bitwise AND of the
///   accumulator and a memory value.
/// * Loads bit six from the memory value into the overflow flag.
/// * Loads bit seven from the memory value into the negative flag.
pub fn bit(cpu: &mut CPU, buses: &mut Buses) {
    let data = buses.read();
    let result = cpu.registers.a & data;

    cpu.set_zero_flag(result == 0);
    cpu.set_overflow_flag((data & 0b_0100_0000) != 0);
    cpu.set_negative_flag((data & 0b_1000_0000) != 0);
}

/// # Bitwise Exclusive OR
///
/// Bitwise XOR of a memory value and the accumulator.
pub fn eor(cpu: &mut CPU, buses: &mut Buses) {
    let data = buses.read();
    let result = cpu.registers.a ^ data;

    cpu.registers.a = result;
    cpu.set_zero_flag(result == 0);
    cpu.set_negative_flag((result & 0b_1000_0000) != 0);
}

/// # Bitwise Exclusive OR
///
/// Bitwise XOR of a memory value and the accumulator. Uses an additional cycle
/// if a page is crossed.
pub fn eor_indirect_y(cpu: &mut CPU, buses: &mut Buses) {
    eor(cpu, buses);

    if cpu.crossed_page {
        cpu.cycle_queue.push_back([get_effective_address, eor]);
    }
}

/// # Bitwise Exclusive OR
///
/// Bitwise XOR of a memory value and the accumulator. Uses an additional cycle
/// if a page is crossed.
pub fn eor_abs_index(cpu: &mut CPU, buses: &mut Buses) {
    eor(cpu, buses);

    if cpu.crossed_page {
        cpu.cycle_queue.push_back([get_effective_address, eor]);
    }
}

/// # Bitwise OR
///
/// Bitwise OR of a memory value and the accumulator.
pub fn ora(cpu: &mut CPU, buses: &mut Buses) {
    let data = buses.read();
    let result = cpu.registers.a | data;

    cpu.registers.a = result;
    cpu.set_zero_flag(result == 0);
    cpu.set_negative_flag((result & 0b_1000_0000) != 0);
}

/// # Bitwise OR
///
/// Bitwise OR of a memory value and the accumulator. Uses an additional cycle
/// if a page crossed.
pub fn ora_indirect_y(cpu: &mut CPU, buses: &mut Buses) {
    ora(cpu, buses);

    if cpu.crossed_page {
        cpu.cycle_queue.push_back([get_effective_address, ora]);
    }
}

/// # Bitwise OR
///
/// Bitwise OR of a memory value and the accumulator. Uses an additional cycle
/// if a page crossed.
pub fn ora_abs_index(cpu: &mut CPU, buses: &mut Buses) {
    ora(cpu, buses);

    if cpu.crossed_page {
        cpu.cycle_queue.push_back([get_effective_address, ora]);
    }
}
