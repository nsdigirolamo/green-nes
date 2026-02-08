use crate::emu::{
    buses::Buses,
    cpu::{CPU, half_cycles::get_effective_address},
};

/// # Load Accumulator
///
/// Loads a memory value into the accumulator.
pub fn lda(cpu: &mut CPU, buses: &mut Buses) {
    let data = buses.read();

    cpu.registers.a = data;
    cpu.set_zero_flag(data == 0);
    cpu.set_negative_flag(data >> 7 == 1);
}

pub fn lda_indirect_y(cpu: &mut CPU, buses: &mut Buses) {
    lda(cpu, buses);

    if cpu.crossed_page {
        cpu.cycle_queue.push_back([get_effective_address, lda]);
    }
}

pub fn lda_absolute_indexed(cpu: &mut CPU, buses: &mut Buses) {
    lda(cpu, buses);

    if cpu.crossed_page {
        cpu.cycle_queue.push_back([get_effective_address, lda]);
    }
}

/// # Load X Register
///
/// Loads a memory value into the X register.
pub fn ldx(cpu: &mut CPU, buses: &mut Buses) {
    let data = buses.read();

    cpu.registers.x_index = data;
    cpu.set_zero_flag(data == 0);
    cpu.set_negative_flag(data >> 7 == 1);
}

pub fn ldx_absolute_indexed(cpu: &mut CPU, buses: &mut Buses) {
    ldx(cpu, buses);

    if cpu.crossed_page {
        cpu.cycle_queue.push_back([get_effective_address, ldx]);
    }
}

/// # Load Y Register
///
/// Loads a memory value into the Y register.
pub fn ldy(cpu: &mut CPU, buses: &mut Buses) {
    let data = buses.read();

    cpu.registers.y_index = data;
    cpu.set_zero_flag(data == 0);
    cpu.set_negative_flag(data >> 7 == 1);
}

pub fn ldy_absolute_indexed(cpu: &mut CPU, buses: &mut Buses) {
    ldy(cpu, buses);

    if cpu.crossed_page {
        cpu.cycle_queue.push_back([get_effective_address, ldy]);
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
