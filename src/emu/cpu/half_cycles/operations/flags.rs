use crate::emu::{buses::Buses, cpu::CPU};

/// # Set Carry
///
/// Sets the carry flag.
pub fn sec(cpu: &mut CPU, _: &mut Buses) {
    cpu.registers.psr.set_carry(true);
}

/// # Clear Carry
///
/// Clears the carry flag.
pub fn clc(cpu: &mut CPU, _: &mut Buses) {
    cpu.registers.psr.set_carry(false);
}

/// # Set Decimal
///
/// SED sets the decimal flag.
pub fn sed(cpu: &mut CPU, _: &mut Buses) {
    cpu.registers.psr.set_decimal(true);
}

/// # Clear Decimal
///
/// Clears the decimal flag.
pub fn cld(cpu: &mut CPU, _: &mut Buses) {
    cpu.registers.psr.set_decimal(false);
}

/// # Set Interrupt Disable
///
/// Sets the interrupt disable flag.
pub fn sei(cpu: &mut CPU, _: &mut Buses) {
    cpu.interrupt_disabled = Some(true)
}

/// # Clear Interrupt Disable
///
/// Clears the interrupt disable flag.
pub fn cli(cpu: &mut CPU, _: &mut Buses) {
    cpu.interrupt_disabled = Some(false)
}

/// # Clear Overflow
///
/// Clears the overflow flag.
pub fn clv(cpu: &mut CPU, _: &mut Buses) {
    cpu.registers.psr.set_overflow(false);
}
