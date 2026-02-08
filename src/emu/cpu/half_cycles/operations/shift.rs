use crate::emu::{buses::Buses, cpu::CPU};

/// # Arithmetic Shift Left (Memory Value)
///
/// Shifts all of the bits of a memory value one position to the left. Bit seven
/// is shifted into the carry flag, and 0 is shifted into bit zero.
pub fn asl_m(cpu: &mut CPU, buses: &mut Buses) {
    let data = buses.read();
    let result = data << 1;

    cpu.set_carry_flag((data & 0b_1000_0000) != 0);
    cpu.set_zero_flag(result == 0);
    cpu.set_negative_flag((result & 0b_1000_0000) != 0);
    buses.write(result);
}

/// # Arithmetic Shift Left (Accumulator)
///
/// Shifts all of the bits of the accumulator one position to the left. Bit
/// seven is shifted into the carry flag, and 0 is shifted into bit zero.
pub fn asl_a(cpu: &mut CPU, _: &mut Buses) {
    let data = cpu.registers.a;
    let result = data << 1;

    cpu.set_carry_flag((data & 0b_1000_0000) != 0);
    cpu.set_zero_flag(result == 0);
    cpu.set_negative_flag((result & 0b_1000_0000) != 0);
    cpu.registers.a = result;
}

/// # Logical Shift Right (Memory Value)
///
/// Shifts all of the bits of a memory value one position to
/// the right, moving the value of each bit into the next bit. 0 is shifted into
/// bit seven, and bit zero is shifted into the carry flag.
pub fn lsr_m(cpu: &mut CPU, buses: &mut Buses) {
    let data = buses.read();
    let result = data >> 1;

    cpu.set_carry_flag((data & 0b_0000_0001) != 0);
    cpu.set_zero_flag(result == 0);
    cpu.set_negative_flag(false);
    buses.write(result);
}

/// # Logical Shift Right (Accumulator)
///
/// Shifts all of the bits of the accumulator one position to the right, moving
/// the value of each bit into the next bit. 0 is shifted into bit seven, and
/// bit zero is shifted into the carry flag.
pub fn lsr_a(cpu: &mut CPU, _: &mut Buses) {
    let data = cpu.registers.a;
    let result = data >> 1;

    cpu.set_carry_flag((data & 0b_0000_0001) != 0);
    cpu.set_zero_flag(result == 0);
    cpu.set_negative_flag(false);
    cpu.registers.a = result;
}

/// Rotate Left (Memory Value)
///
/// Shifts a memory value to the left, moving the value of each bit into the
/// neighboring bit and treating the carry flag as though it is both to the left
/// of bit seven and to the right of bit zero.
pub fn rol(cpu: &mut CPU, buses: &mut Buses) {
    let data = buses.read();
    let result = (data << 1) | (cpu.get_carry_flag() as u8);

    cpu.set_carry_flag((data & 0b_1000_0000) != 0);
    cpu.set_zero_flag(result == 0);
    cpu.set_negative_flag((result & 0b_1000_0000) != 0);
    buses.write(result);
}

/// Rotate Left (Accumulator)
///
/// Shifts the accumulator to the left, moving the value of each bit into the
/// neighboring bit and treating the carry flag as though it is both to the left
/// of bit seven and to the right of bit zero.
pub fn rol_a(cpu: &mut CPU, _: &mut Buses) {
    let data = cpu.registers.a;
    let result = (data << 1) | (cpu.get_carry_flag() as u8);

    cpu.set_carry_flag((data & 0b_1000_0000) != 0);
    cpu.set_zero_flag(result == 0);
    cpu.set_negative_flag((result & 0b_1000_0000) != 0);
    cpu.registers.a = result;
}

/// # Rotate Right (Memory Value)
///
/// Shifts a memory value to the left, moving the value of each bit into the
/// neighboring bit and treating the carry flag as though it is both to the
/// right of bit zero and to the left of bit seven.
pub fn ror_m(cpu: &mut CPU, buses: &mut Buses) {
    let data = buses.read();
    let masked_data = data & 0b_1111_1110;
    let result = (masked_data | cpu.get_carry_flag() as u8).rotate_right(1);

    cpu.set_carry_flag((data & 0b_0000_0001) != 0);
    cpu.set_zero_flag(result == 0);
    cpu.set_negative_flag((result & 0b_1000_0000) != 0);
    buses.write(result);
}

/// # Rotate Right (Accumulator)
///
/// Shifts the accumulator to the left, moving the value of each bit into the
/// neighboring bit and treating the carry flag as though it is both to the
/// right of bit zero and to the left of bit seven.
pub fn ror_a(cpu: &mut CPU, _: &mut Buses) {
    let data = cpu.registers.a;
    let masked_data = data & 0b_1111_1110;
    let result = (masked_data | cpu.get_carry_flag() as u8).rotate_right(1);

    cpu.set_carry_flag((data & 0b_0000_0001) != 0);
    cpu.set_zero_flag(result == 0);
    cpu.set_negative_flag((result & 0b_1000_0000) != 0);
    cpu.registers.a = result;
}
