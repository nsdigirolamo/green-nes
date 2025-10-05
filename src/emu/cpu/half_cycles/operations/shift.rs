use crate::emu::{buses::Buses, cpu::CPU};

pub fn asl(cpu: &mut CPU, buses: &mut Buses) {
    let data = buses.read();
    let result = data << 1;

    cpu.set_carry_flag((data & 0b_1000_0000) != 0);
    cpu.set_zero_flag(result == 0);
    cpu.set_negative_flag((result & 0b_1000_0000) != 0);
    buses.write(result);
}

pub fn asl_accumulator(cpu: &mut CPU, _: &mut Buses) {
    let data = cpu.registers.a;
    let result = data << 1;

    cpu.set_carry_flag((data & 0b_1000_0000) != 0);
    cpu.set_zero_flag(result == 0);
    cpu.set_negative_flag((result & 0b_1000_0000) != 0);
    cpu.registers.a = result;
}

pub fn lsr(cpu: &mut CPU, buses: &mut Buses) {
    let data = buses.read();
    let result = data >> 1;

    cpu.set_carry_flag((data & 0b_0000_0001) != 0);
    cpu.set_zero_flag(result == 0);
    cpu.set_negative_flag(false);
    buses.write(result);
}

pub fn lsr_accumulator(cpu: &mut CPU, _: &mut Buses) {
    let data = cpu.registers.a;
    let result = data >> 1;

    cpu.set_carry_flag((data & 0b_0000_0001) != 0);
    cpu.set_zero_flag(result == 0);
    cpu.set_negative_flag(false);
    cpu.registers.a = result;
}

pub fn rol(cpu: &mut CPU, buses: &mut Buses) {
    let data = buses.read();
    let result = (data << 1) | (cpu.get_carry_flag() as u8);

    cpu.set_carry_flag((data & 0b_1000_0000) != 0);
    cpu.set_zero_flag(result == 0);
    cpu.set_negative_flag((result & 0b_1000_0000) != 0);
    buses.write(result);
}

pub fn rol_accumulator(cpu: &mut CPU, _: &mut Buses) {
    let data = cpu.registers.a;
    let result = (data << 1) | (cpu.get_carry_flag() as u8);

    cpu.set_carry_flag((data & 0b_1000_0000) != 0);
    cpu.set_zero_flag(result == 0);
    cpu.set_negative_flag((result & 0b_1000_0000) != 0);
    cpu.registers.a = result;
}

pub fn ror(cpu: &mut CPU, buses: &mut Buses) {
    let data = buses.read();
    let masked_data = data & 0b_1111_1110;
    let result = (masked_data | cpu.get_carry_flag() as u8).rotate_right(1);

    cpu.set_carry_flag((data & 0b_0000_0001) != 0);
    cpu.set_zero_flag(result == 0);
    cpu.set_negative_flag((result & 0b_1000_0000) != 0);
    buses.write(result);
}

pub fn ror_accumulator(cpu: &mut CPU, _: &mut Buses) {
    let data = cpu.registers.a;
    let masked_data = data & 0b_1111_1110;
    let result = (masked_data | cpu.get_carry_flag() as u8).rotate_right(1);

    cpu.set_carry_flag((data & 0b_0000_0001) != 0);
    cpu.set_zero_flag(result == 0);
    cpu.set_negative_flag((result & 0b_1000_0000) != 0);
    cpu.registers.a = result;
}
