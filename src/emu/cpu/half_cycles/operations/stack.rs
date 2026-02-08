use crate::emu::{buses::Buses, cpu::CPU};

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

    cpu.set_zero_flag(result == 0);
    cpu.set_negative_flag((result & 0b_1000_0000) != 0);
    cpu.registers.a = result
}

/// # Pull Processor Status
///
/// Pops the value from the top of the stack and loads it into the processor
/// status flags.
pub fn plp(cpu: &mut CPU, buses: &mut Buses) {
    // B and extra bit are ignored
    let masked_stack_status = buses.read() & 0b_1100_1111;
    let masked_processor_status = cpu.registers.psr & 0b_0011_0000;
    let new_processor_status = masked_stack_status | masked_processor_status;

    let old_interrupt_disable = cpu.get_interrupt_disable_flag();
    let new_interrupt_disable = new_processor_status >> 2 & 1 == 1;

    cpu.registers.psr = new_processor_status;

    // Set back to old interrupt disable flag, then re-apply with delay.
    cpu.set_interrupt_disable_flag(old_interrupt_disable);
    cpu.set_interrupt_disable_flag_with_delay(new_interrupt_disable);
}
