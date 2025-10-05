use crate::emu::{
    buses::Buses,
    cpu::{CPU, half_cycles::get_effective_address},
};

pub fn and(cpu: &mut CPU, buses: &mut Buses) {
    let data = buses.read();
    let result = cpu.registers.a & data;

    cpu.registers.a = result;
    cpu.set_zero_flag(result == 0);
    cpu.set_negative_flag((result & 0b_1000_0000) != 0);
}

pub fn and_indirect_y(cpu: &mut CPU, buses: &mut Buses) {
    and(cpu, buses);

    if cpu.crossed_page {
        cpu.cycle_queue.push_back([get_effective_address, and]);
    }
}

pub fn and_absolute_indexed(cpu: &mut CPU, buses: &mut Buses) {
    and(cpu, buses);

    if cpu.crossed_page {
        cpu.cycle_queue.push_back([get_effective_address, and]);
    }
}

pub fn bit(cpu: &mut CPU, buses: &mut Buses) {
    let data = buses.read();
    let result = cpu.registers.a & data;

    cpu.set_zero_flag(result == 0);
    cpu.set_overflow_flag((data & 0b_0100_0000) != 0);
    cpu.set_negative_flag((data & 0b_1000_0000) != 0);
}

pub fn eor(cpu: &mut CPU, buses: &mut Buses) {
    let data = buses.read();
    let result = cpu.registers.a ^ data;

    cpu.registers.a = result;
    cpu.set_zero_flag(result == 0);
    cpu.set_negative_flag((result & 0b_1000_0000) != 0);
}

pub fn eor_indirect_y(cpu: &mut CPU, buses: &mut Buses) {
    eor(cpu, buses);

    if cpu.crossed_page {
        cpu.cycle_queue.push_back([get_effective_address, eor]);
    }
}

pub fn eor_absolute_indexed(cpu: &mut CPU, buses: &mut Buses) {
    eor(cpu, buses);

    if cpu.crossed_page {
        cpu.cycle_queue.push_back([get_effective_address, eor]);
    }
}

pub fn ora(cpu: &mut CPU, buses: &mut Buses) {
    let data = buses.read();
    let result = cpu.registers.a | data;

    cpu.registers.a = result;
    cpu.set_zero_flag(result == 0);
    cpu.set_negative_flag((result & 0b_1000_0000) != 0);
}

pub fn ora_indirect_y(cpu: &mut CPU, buses: &mut Buses) {
    ora(cpu, buses);

    if cpu.crossed_page {
        cpu.cycle_queue.push_back([get_effective_address, ora]);
    }
}

pub fn ora_absolute_indexed(cpu: &mut CPU, buses: &mut Buses) {
    ora(cpu, buses);

    if cpu.crossed_page {
        cpu.cycle_queue.push_back([get_effective_address, ora]);
    }
}
