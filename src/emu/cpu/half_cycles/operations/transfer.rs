use crate::emu::{buses::Buses, cpu::CPU};

/// # Transfer Accumulator to X Register
///
/// Copies the accumulator's value into the X register.
pub fn tax(cpu: &mut CPU, buses: &mut Buses) {
    let result = cpu.registers.a;

    cpu.registers.x_index = result;
    buses.addr = cpu.registers.pc;
    cpu.set_zero_flag(result == 0);
    cpu.set_negative_flag((result & 0b_1000_0000) != 0);
}

/// # Transfer Accumulator to Y Register
///
/// Copies the accumulator's value into the Y register.
pub fn tay(cpu: &mut CPU, buses: &mut Buses) {
    let result = cpu.registers.a;

    cpu.registers.y_index = result;
    buses.addr = cpu.registers.pc;
    cpu.set_zero_flag(result == 0);
    cpu.set_negative_flag((result & 0b_1000_0000) != 0);
}

/// # Transfer Stack Pointer to X Register
///
/// Copies the stack pointer's value into the X register.
pub fn tsx(cpu: &mut CPU, buses: &mut Buses) {
    let result = cpu.registers.sp;

    cpu.registers.x_index = result;
    buses.addr = cpu.registers.pc;
    cpu.set_zero_flag(result == 0);
    cpu.set_negative_flag((result & 0b_1000_0000) != 0);
}

/// # Transfer X Register to Accumulator
///
/// Copies the X register's value into the accumulator.
pub fn txa(cpu: &mut CPU, buses: &mut Buses) {
    let result = cpu.registers.x_index;

    cpu.registers.a = result;
    buses.addr = cpu.registers.pc;
    cpu.set_zero_flag(result == 0);
    cpu.set_negative_flag((result & 0b_1000_0000) != 0);
}

/// # Transfer X Register to Stack Pointer
///
/// Copies the X register's value into the stack pointer.
pub fn txs(cpu: &mut CPU, buses: &mut Buses) {
    cpu.registers.sp = cpu.registers.x_index;
    buses.addr = cpu.registers.pc;
}

/// # Transfer Y Register to Accumulator
///
/// Copies the Y register's value into the accumulator.
pub fn tya(cpu: &mut CPU, buses: &mut Buses) {
    let result = cpu.registers.y_index;

    cpu.registers.a = result;
    buses.addr = cpu.registers.pc;
    cpu.set_zero_flag(result == 0);
    cpu.set_negative_flag((result & 0b_1000_0000) != 0);
}
