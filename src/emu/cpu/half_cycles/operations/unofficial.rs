// Reference: https://www.masswerk.at/nowgobang/2021/6502-illegal-opcodes

use crate::emu::{
    buses::Buses,
    cpu::{CPU, half_cycles::get_effective_address},
};

pub fn lax(cpu: &mut CPU, buses: &mut Buses) {
    super::access::lda(cpu, buses);
    super::access::ldx(cpu, buses);
}

pub fn lax_indirect_y(cpu: &mut CPU, buses: &mut Buses) {
    lax(cpu, buses);

    if cpu.crossed_page {
        cpu.cycle_queue.push_back([get_effective_address, lax]);
    }
}

pub fn lax_absolute_indexed(cpu: &mut CPU, buses: &mut Buses) {
    lax(cpu, buses);

    if cpu.crossed_page {
        cpu.cycle_queue.push_back([get_effective_address, lax]);
    }
}

pub fn sax(cpu: &mut CPU, buses: &mut Buses) {
    let data = cpu.registers.a & cpu.registers.x_index;

    buses.write(data);
}

pub fn usbc(cpu: &mut CPU, buses: &mut Buses) {
    super::arithmetic::sbc(cpu, buses);
}

pub fn dcp(cpu: &mut CPU, buses: &mut Buses) {
    super::arithmetic::dec(cpu, buses);
    super::compare::cmp(cpu, buses);
}

pub fn isc(cpu: &mut CPU, buses: &mut Buses) {
    super::arithmetic::inc(cpu, buses);
    super::arithmetic::sbc(cpu, buses);
}

pub fn slo(cpu: &mut CPU, buses: &mut Buses) {
    super::shift::asl(cpu, buses);
    super::bitwise::ora(cpu, buses);
}

pub fn rla(cpu: &mut CPU, buses: &mut Buses) {
    super::shift::rol(cpu, buses);
    super::bitwise::and(cpu, buses);
}

pub fn sre(cpu: &mut CPU, buses: &mut Buses) {
    super::shift::lsr(cpu, buses);
    super::bitwise::eor(cpu, buses);
}

pub fn rra(cpu: &mut CPU, buses: &mut Buses) {
    super::shift::ror(cpu, buses);
    super::arithmetic::adc(cpu, buses);
}

pub fn jam(cpu: &mut CPU, _: &mut Buses) {
    cpu.is_halted = true;
}
