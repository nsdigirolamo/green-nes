use crate::emu::{buses::Buses, cpu::CPU};

pub fn jsr(cpu: &mut CPU, buses: &mut Buses) {
    let data = buses.read();

    cpu.buses.effective_addr.0 = data;
    buses.data = data;
    cpu.registers.pc = cpu.buses.effective_addr;
}

pub fn jmp_absolute(cpu: &mut CPU, buses: &mut Buses) {
    let data = buses.read();

    cpu.buses.effective_addr.0 = data;
    buses.data = data;
    cpu.registers.pc = cpu.buses.effective_addr;
}

pub fn rti(cpu: &mut CPU, buses: &mut Buses) {
    // B and extra bit are ignored
    let masked_stack_status = buses.read() & 0b_1100_1111;
    let masked_processor_status = cpu.registers.psr & 0b_0011_0000;
    let new_processor_status = masked_stack_status | masked_processor_status;

    cpu.registers.psr = new_processor_status;
}
