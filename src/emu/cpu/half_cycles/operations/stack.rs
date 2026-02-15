use crate::emu::{
    buses::Buses,
    cpu::{CPU, registers::flags::Flags},
};

/// # Push Accumulator
///
/// Pushes the accumulator's value onto the top of the stack.
pub fn pha(cpu: &mut CPU, buses: &mut Buses) {
    buses.write(cpu.registers.a);
}

/// # Push Processor Status
///
/// Pushes the processor status flags onto the top of the stack.
pub fn php(cpu: &mut CPU, buses: &mut Buses) {
    buses.write(cpu.registers.psr | 0b_0011_0000);
}

/// # Pull Acumulator
///
/// Pops the value from the top of the stack and loads it into the accumulator.
pub fn pla(cpu: &mut CPU, buses: &mut Buses) {
    let result = buses.read();

    cpu.registers.psr.set_zero(result == 0);
    cpu.registers.psr.set_negative(result & Flags::N != 0);
    cpu.registers.a = result
}

/// # Pull Processor Status
///
/// Pops the value from the top of the stack and loads it into the processor
/// status flags.
pub fn plp(cpu: &mut CPU, buses: &mut Buses) {
    // B and extra bit are ignored
    let masked_stack_status = buses.read() & 0b_1100_1111;
    let masked_psr = cpu.registers.psr & 0b_0011_0000;
    let new_psr = masked_stack_status | masked_psr;

    let old_i = cpu.registers.psr.get_interrupt_disable();
    let new_i = new_psr >> 2 & 1 == 1;

    cpu.registers.psr = new_psr.into();

    // Set back to old interrupt disable flag, then re-apply with delay.
    cpu.registers.psr.set_interrupt_disable(old_i);
    cpu.interrupt_disabled = Some(new_i);
}
