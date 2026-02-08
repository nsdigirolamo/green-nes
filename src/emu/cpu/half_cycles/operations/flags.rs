use crate::emu::{buses::Buses, cpu::CPU};

/// # Set Carry
///
/// Sets the carry flag.
pub fn sec(cpu: &mut CPU, _: &mut Buses) {
    cpu.set_carry_flag(true);
}

/// # Clear Carry
///
/// Clears the carry flag.
pub fn clc(cpu: &mut CPU, _: &mut Buses) {
    cpu.set_carry_flag(false);
}

/// # Set Decimal
///
/// SED sets the decimal flag.
pub fn sed(cpu: &mut CPU, _: &mut Buses) {
    cpu.set_decimal_mode_flag(true);
}

/// # Clear Decimal
///
/// Clears the decimal flag.
pub fn cld(cpu: &mut CPU, _: &mut Buses) {
    cpu.set_decimal_mode_flag(false);
}

/// # Set Interrupt Disable
///
/// Sets the interrupt disable flag.
pub fn sei(cpu: &mut CPU, _: &mut Buses) {
    cpu.set_interrupt_disable_flag_with_delay(true);
}

/// # Clear Interrupt Disable
///
/// Clears the interrupt disable flag.
pub fn cli(cpu: &mut CPU, _: &mut Buses) {
    cpu.set_interrupt_disable_flag_with_delay(false);
}

/// # Clear Overflow
///
/// Clears the overflow flag.
pub fn clv(cpu: &mut CPU, _: &mut Buses) {
    cpu.set_overflow_flag(false);
}
