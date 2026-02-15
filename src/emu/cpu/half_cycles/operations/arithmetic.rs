use crate::{
    did_signed_overflow,
    emu::{
        buses::Buses,
        cpu::{CPU, half_cycles::get_effective_address, registers::flags::Flags},
    },
};

/// # Increment Memory
///
/// Adds 1 to a memory value.
pub fn inc(cpu: &mut CPU, buses: &mut Buses) {
    let data = buses.read();
    let result = data.wrapping_add(1);

    cpu.registers.psr.set_zero(result == 0);
    cpu.registers.psr.set_negative(result & Flags::N != 0);
    buses.write(result);
}

/// # Increment X Register
///
/// Adds 1 to the X register.
pub fn inx(cpu: &mut CPU, _: &mut Buses) {
    let data = cpu.registers.x_index;
    let result = data.wrapping_add(1);

    cpu.registers.psr.set_zero(result == 0);
    cpu.registers.psr.set_negative(result & Flags::N != 0);
    cpu.registers.x_index = result;
}

/// # Increment Y Register
///
/// Adds 1 to the Y register.
pub fn iny(cpu: &mut CPU, _: &mut Buses) {
    let data = cpu.registers.y_index;
    let result = data.wrapping_add(1);

    cpu.registers.psr.set_zero(result == 0);
    cpu.registers.psr.set_negative(result & Flags::N != 0);
    cpu.registers.y_index = result;
}

/// # Decrement Memory
///
/// Subtracts 1 from a memory value.
pub fn dec(cpu: &mut CPU, buses: &mut Buses) {
    let data = buses.read();
    let result = data.wrapping_sub(1);

    cpu.registers.psr.set_zero(result == 0);
    cpu.registers.psr.set_negative(result & Flags::N != 0);
    buses.write(result);
}

/// # Decrement X Register
///
/// Subtracts 1 from the X register.
pub fn dex(cpu: &mut CPU, _: &mut Buses) {
    let data = cpu.registers.x_index;
    let result = data.wrapping_sub(1);

    cpu.registers.psr.set_zero(result == 0);
    cpu.registers.psr.set_negative(result & Flags::N != 0);
    cpu.registers.x_index = result;
}

/// # Decrement Y Register
///
/// Subtracts 1 from the Y register.
pub fn dey(cpu: &mut CPU, _: &mut Buses) {
    let data = cpu.registers.y_index;
    let result = data.wrapping_sub(1);

    cpu.registers.psr.set_zero(result == 0);
    cpu.registers.psr.set_negative(result & Flags::N != 0);
    cpu.registers.y_index = result;
}

/// # Add With Carry
///
/// Adds a memory value and the carry flag to the accumulator.
pub fn adc(cpu: &mut CPU, buses: &mut Buses) {
    let data = buses.read();
    let accumulator = cpu.registers.a;
    let (sum, overflow1) = accumulator.overflowing_add(data);
    let (result, overflow2) = sum.overflowing_add(cpu.registers.psr.get_carry() as u8);
    let did_unsigned_overflow = overflow1 | overflow2;
    let did_signed_overflow = did_signed_overflow!(accumulator, data, result);

    cpu.registers.a = result;
    cpu.registers.psr.set_carry(did_unsigned_overflow);
    cpu.registers.psr.set_zero(result == 0);
    cpu.registers.psr.set_overflow(did_signed_overflow);
    cpu.registers.psr.set_negative(result & Flags::N != 0);
}

/// # Add With Carry
///
/// Adds a memory value and the carry flag to the accumulator. Uses an
/// additional cycle if a page is crossed.
pub fn adc_indirect_y(cpu: &mut CPU, buses: &mut Buses) {
    adc(cpu, buses);

    if cpu.crossed_page {
        cpu.cycle_queue.push_back([get_effective_address, adc]);
    }
}

/// # Add With Carry
///
/// Adds a memory value and the carry flag to the accumulator. Uses an
/// additional cycle if a page is crossed.
pub fn adc_abs_index(cpu: &mut CPU, buses: &mut Buses) {
    adc(cpu, buses);

    if cpu.crossed_page {
        cpu.cycle_queue.push_back([get_effective_address, adc]);
    }
}

/// # Subtract With Carry
///
/// Subtracts a memory value and the bitwise NOT of the carry flag from the
/// accumulator.
pub fn sbc(cpu: &mut CPU, buses: &mut Buses) {
    let data = buses.read();
    let accumulator = cpu.registers.a;
    let (sum, overflow1) = accumulator.overflowing_add(!data);
    let (result, overflow2) = sum.overflowing_add(cpu.registers.psr.get_carry() as u8);
    let did_unsigned_overflow = overflow1 | overflow2;
    let did_signed_overflow = did_signed_overflow!(accumulator, !data, result);

    cpu.registers.a = result;
    cpu.registers.psr.set_carry(did_unsigned_overflow);
    cpu.registers.psr.set_zero(result == 0);
    cpu.registers.psr.set_overflow(did_signed_overflow);
    cpu.registers.psr.set_negative(result & Flags::N != 0);
}

/// # Subtract With Carry
///
/// Subtracts a memory value and the bitwise NOT of the carry flag from the
/// accumulator. Uses an additional cycle if a page is crossed.
pub fn sbc_indirect_y(cpu: &mut CPU, buses: &mut Buses) {
    sbc(cpu, buses);

    if cpu.crossed_page {
        cpu.cycle_queue.push_back([get_effective_address, sbc]);
    }
}

/// # Subtract With Carry
///
/// Subtracts a memory value and the bitwise NOT of the carry flag from the
/// accumulator. Uses an additional cycle if a page is crossed.
pub fn sbc_abs_index(cpu: &mut CPU, buses: &mut Buses) {
    sbc(cpu, buses);

    if cpu.crossed_page {
        cpu.cycle_queue.push_back([get_effective_address, sbc]);
    }
}
