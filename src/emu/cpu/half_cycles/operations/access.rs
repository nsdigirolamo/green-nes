use crate::emu::{
    buses::Buses,
    cpu::{CPU, half_cycles::get_effective_addr, registers::flags::Flags},
};

/// # Load Accumulator
///
/// Loads a memory value into the accumulator.
pub fn lda(cpu: &mut CPU, buses: &mut Buses) {
    let data = buses.read();

    cpu.registers.a = data;
    cpu.registers.psr.set_zero(data == 0);
    cpu.registers.psr.set_negative(data & Flags::N != 0);
}

/// # Load Accumulator
///
/// Loads a memory value into the accumulator. Uses an additional cycle if a
/// page is crossed.
pub fn lda_indirect_y(cpu: &mut CPU, buses: &mut Buses) {
    lda(cpu, buses);

    if cpu.crossed_page {
        cpu.cycle_queue.push_back([get_effective_addr, lda]);
    }
}

/// # Load Accumulator
///
/// Loads a memory value into the accumulator. Uses an additional cycle if a
/// page is crossed.
pub fn lda_abs_index(cpu: &mut CPU, buses: &mut Buses) {
    lda(cpu, buses);

    if cpu.crossed_page {
        cpu.cycle_queue.push_back([get_effective_addr, lda]);
    }
}

/// # Load X Register
///
/// Loads a memory value into the X register.
pub fn ldx(cpu: &mut CPU, buses: &mut Buses) {
    let data = buses.read();

    cpu.registers.x_index = data;
    cpu.registers.psr.set_zero(data == 0);
    cpu.registers.psr.set_negative(data & Flags::N != 0);
}

/// # Load X Register
///
/// Loads a memory value into the X register. Uses an additional cycle if a page
/// is crossed.
pub fn ldx_abs_index(cpu: &mut CPU, buses: &mut Buses) {
    ldx(cpu, buses);

    if cpu.crossed_page {
        cpu.cycle_queue.push_back([get_effective_addr, ldx]);
    }
}

/// # Load Y Register
///
/// Loads a memory value into the Y register.
pub fn ldy(cpu: &mut CPU, buses: &mut Buses) {
    let data = buses.read();

    cpu.registers.y_index = data;
    cpu.registers.psr.set_zero(data == 0);
    cpu.registers.psr.set_negative(data & Flags::N != 0);
}

/// # Load Y Register
///
/// Loads a memory value into the Y register. Uses an additional cycle if a page
/// is crossed.
pub fn ldy_abs_index(cpu: &mut CPU, buses: &mut Buses) {
    ldy(cpu, buses);

    if cpu.crossed_page {
        cpu.cycle_queue.push_back([get_effective_addr, ldy]);
    }
}

/// # Store Accumulator
///
/// Stores the accumulator's value into memory.
pub fn sta(cpu: &mut CPU, buses: &mut Buses) {
    let data = cpu.registers.a;

    buses.write(data)
}

/// # Store X Register
///
/// Stores the X register's value into memory.
pub fn stx(cpu: &mut CPU, buses: &mut Buses) {
    let data = cpu.registers.x_index;

    buses.write(data);
}

/// # Store Y Register
///
/// Stores the Y register's value into memory.
pub fn sty(cpu: &mut CPU, buses: &mut Buses) {
    let data = cpu.registers.y_index;

    buses.write(data);
}
