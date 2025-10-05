use crate::emu::{buses::Buses, cpu::CPU};

pub fn tax(cpu: &mut CPU, buses: &mut Buses) {
    let result = cpu.registers.a;

    cpu.registers.x_index = result;
    buses.addr = cpu.registers.pc;
    cpu.set_zero_flag(result == 0);
    cpu.set_negative_flag((result & 0b_1000_0000) != 0);
}

pub fn tay(cpu: &mut CPU, buses: &mut Buses) {
    let result = cpu.registers.a;

    cpu.registers.y_index = result;
    buses.addr = cpu.registers.pc;
    cpu.set_zero_flag(result == 0);
    cpu.set_negative_flag((result & 0b_1000_0000) != 0);
}

pub fn tsx(cpu: &mut CPU, buses: &mut Buses) {
    let result = cpu.registers.sp;

    cpu.registers.x_index = result;
    buses.addr = cpu.registers.pc;
    cpu.set_zero_flag(result == 0);
    cpu.set_negative_flag((result & 0b_1000_0000) != 0);
}

pub fn txa(cpu: &mut CPU, buses: &mut Buses) {
    let result = cpu.registers.x_index;

    cpu.registers.a = result;
    buses.addr = cpu.registers.pc;
    cpu.set_zero_flag(result == 0);
    cpu.set_negative_flag((result & 0b_1000_0000) != 0);
}

pub fn txs(cpu: &mut CPU, buses: &mut Buses) {
    cpu.registers.sp = cpu.registers.x_index;
    buses.addr = cpu.registers.pc;
}

pub fn tya(cpu: &mut CPU, buses: &mut Buses) {
    let result = cpu.registers.y_index;

    cpu.registers.a = result;
    buses.addr = cpu.registers.pc;
    cpu.set_zero_flag(result == 0);
    cpu.set_negative_flag((result & 0b_1000_0000) != 0);
}
