use crate::emu::{buses::Buses, cpu::CPU};

pub fn pha(cpu: &mut CPU, buses: &mut Buses) {
    buses.write(cpu.registers.a);
}

pub fn php(cpu: &mut CPU, buses: &mut Buses) {
    buses.write(cpu.registers.psr | 0b_0011_0000);
}

pub fn plp(cpu: &mut CPU, buses: &mut Buses) {
    // B and extra bit are ignored
    let masked_stack_status = buses.read() & 0b_1100_1111;
    let masked_processor_status = cpu.registers.psr & 0b_0011_0000;
    let new_processor_status = masked_stack_status | masked_processor_status;
    // @TODO: The effect of changing the I flag should be delayed by one instruction.
    cpu.registers.psr = new_processor_status
}

pub fn pla(cpu: &mut CPU, buses: &mut Buses) {
    let result = buses.read();

    cpu.set_zero_flag(result == 0);
    cpu.set_negative_flag((result & 0b_1000_0000) != 0);
    cpu.registers.a = result
}
