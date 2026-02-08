use crate::emu::{
    buses::Buses,
    cpu::{CPU, half_cycles::get_effective_address},
};

// See: https://www.nesdev.org/wiki/Programming_with_unofficial_opcodes

/// # Bitwise AND and Logical Shift Right (Accumulator)
///
/// Performs a bitwise AND and then a logical shift right on the accumulator.
pub fn alr(_: &mut CPU, _: &mut Buses) {
    todo!("operation not yet implemented: ALR")
}

/// # Bitwise AND With Carry
///
/// Performs a bitwise AND immediate and then copies the resulting negative flag
/// into the carry flag.
pub fn anc(_: &mut CPU, _: &mut Buses) {
    todo!("operation not yet implemented: ANC")
}

/// # Bitwise AND With Rotate Right (Accumulator)
///
/// Performs a bitwise AND and then a rotate right (accumulator), and sets the
/// flags in the following fashion:
/// * The negative flag is set to bit seven of the result.
/// * The zero flag is set if the result is zero.
/// * The carry bit is set to bit six of the result.
/// * The overflow bit is set to bit six XOR bit five of the result.
pub fn arr(_: &mut CPU, _: &mut Buses) {
    todo!("operation not yet implemented: ARR")
}

/// # Bitwise AND of X Register
///
/// Sets the X register to a bitwise AND of the accumulator and the X register's
/// current value.
pub fn axs(_: &mut CPU, _: &mut Buses) {}

/// # Load Accumulator and X Register
///
/// Loads a memory value into the accumulator and X register.
pub fn lax(cpu: &mut CPU, buses: &mut Buses) {
    super::access::lda(cpu, buses);
    super::access::ldx(cpu, buses);
}

/// # Load Accumulator and X Register
///
/// Loads a memory value into the accumulator and X register. Uses an additional
/// cycle if a page is crossed.
pub fn lax_indirect_y(cpu: &mut CPU, buses: &mut Buses) {
    lax(cpu, buses);

    if cpu.crossed_page {
        cpu.cycle_queue.push_back([get_effective_address, lax]);
    }
}

/// # Load Accumulator and X Register
///
/// Loads a memory value into the accumulator and X register. Uses an additional
/// cycle if a page is crossed.
pub fn lax_abs_index(cpu: &mut CPU, buses: &mut Buses) {
    lax(cpu, buses);

    if cpu.crossed_page {
        cpu.cycle_queue.push_back([get_effective_address, lax]);
    }
}

/// # Store Bitwise AND
///
/// Stores the bitwise AND of the accumulator and X register into memory.
pub fn sax(cpu: &mut CPU, buses: &mut Buses) {
    let data = cpu.registers.a & cpu.registers.x_index;

    buses.write(data);
}

/// # Decrement and Compare Accumulator
///
/// Subtracts 1 from a memory value, and then compares A to that memory value,
/// setting flags as appropriate.
pub fn dcp(cpu: &mut CPU, buses: &mut Buses) {
    super::arithmetic::dec(cpu, buses);
    super::compare::cmp(cpu, buses);
}

/// # Increment and Subtract With Carry
///
/// Adds 1 to a memory value, and then subtracts that memory value and the
/// bitwise NOT of the carry flag from the accumulator.
pub fn isc(cpu: &mut CPU, buses: &mut Buses) {
    super::arithmetic::inc(cpu, buses);
    super::arithmetic::sbc(cpu, buses);
}

/// # Rotate Left and Bitwise AND
///
/// Performs a rotate left on a memory value, and then performs a bitwise AND
/// of that value with the accumulator.
pub fn rla(cpu: &mut CPU, buses: &mut Buses) {
    super::shift::rol(cpu, buses);
    super::bitwise::and(cpu, buses);
}

/// # Rotate Right and Add With Carry
///
/// Performs a rotate right on a memory value, and then adds that memory value
/// and the carry flag to the accumulator.
pub fn rra(cpu: &mut CPU, buses: &mut Buses) {
    super::shift::ror_m(cpu, buses);
    super::arithmetic::adc(cpu, buses);
}

/// # Arithmetic Shift Left and Bitwise OR
///
/// Performs an arithmetic shift left on a memory value, and then performs a
/// bitwise OR of that memory value and the accumulator.
pub fn slo(cpu: &mut CPU, buses: &mut Buses) {
    super::shift::asl_m(cpu, buses);
    super::bitwise::ora(cpu, buses);
}

/// # Logical Shift Right and Bitwise XOR
///
/// Performs a logical shift right on a memory value, and then performs a
/// bitwise XOR of that memory value and the accumulator.
pub fn sre(cpu: &mut CPU, buses: &mut Buses) {
    super::shift::lsr_m(cpu, buses);
    super::bitwise::eor(cpu, buses);
}
