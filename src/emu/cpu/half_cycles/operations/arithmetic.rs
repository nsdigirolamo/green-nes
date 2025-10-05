use crate::{
    did_signed_overflow,
    emu::{
        buses::Buses,
        cpu::{CPU, half_cycles::get_effective_address},
    },
};

pub fn inc(cpu: &mut CPU, buses: &mut Buses) {
    let data = buses.read();
    let result = data.wrapping_add(1);

    cpu.set_zero_flag(result == 0);
    cpu.set_negative_flag((result & 0b_1000_0000) != 0);
    buses.write(result);
}

pub fn inx(cpu: &mut CPU, _: &mut Buses) {
    let data = cpu.registers.x_index;
    let result = data.wrapping_add(1);

    cpu.set_zero_flag(result == 0);
    cpu.set_negative_flag((result & 0b_1000_0000) != 0);
    cpu.registers.x_index = result;
}

pub fn iny(cpu: &mut CPU, _: &mut Buses) {
    let data = cpu.registers.y_index;
    let result = data.wrapping_add(1);

    cpu.set_zero_flag(result == 0);
    cpu.set_negative_flag((result & 0b_1000_0000) != 0);
    cpu.registers.y_index = result;
}

pub fn dec(cpu: &mut CPU, buses: &mut Buses) {
    let data = buses.read();
    let result = data.wrapping_sub(1);

    cpu.set_zero_flag(result == 0);
    cpu.set_negative_flag((result & 0b_1000_0000) != 0);
    buses.write(result);
}

pub fn dex(cpu: &mut CPU, _: &mut Buses) {
    let data = cpu.registers.x_index;
    let result = data.wrapping_sub(1);

    cpu.set_zero_flag(result == 0);
    cpu.set_negative_flag((result & 0b_1000_0000) != 0);
    cpu.registers.x_index = result;
}

pub fn dey(cpu: &mut CPU, _: &mut Buses) {
    let data = cpu.registers.y_index;
    let result = data.wrapping_sub(1);

    cpu.set_zero_flag(result == 0);
    cpu.set_negative_flag((result & 0b_1000_0000) != 0);
    cpu.registers.y_index = result;
}

pub fn adc(cpu: &mut CPU, buses: &mut Buses) {
    let data = buses.read();
    let accumulator = cpu.registers.a;
    let (sum, overflow1) = accumulator.overflowing_add(data);
    let (result, overflow2) = sum.overflowing_add(cpu.get_carry_flag() as u8);
    let did_unsigned_overflow = overflow1 | overflow2;
    let did_signed_overflow = did_signed_overflow!(accumulator, data, result);

    cpu.registers.a = result;
    cpu.set_carry_flag(did_unsigned_overflow);
    cpu.set_zero_flag(result == 0);
    cpu.set_overflow_flag(did_signed_overflow);
    cpu.set_negative_flag((result & 0b_1000_0000) != 0);
}

pub fn adc_indirect_y(cpu: &mut CPU, buses: &mut Buses) {
    adc(cpu, buses);

    if cpu.crossed_page {
        cpu.cycle_queue.push_back([get_effective_address, adc]);
    }
}

pub fn adc_absolute_indexed(cpu: &mut CPU, buses: &mut Buses) {
    adc(cpu, buses);

    if cpu.crossed_page {
        cpu.cycle_queue.push_back([get_effective_address, adc]);
    }
}

pub fn sbc(cpu: &mut CPU, buses: &mut Buses) {
    let data = buses.read();
    let accumulator = cpu.registers.a;
    let (sum, overflow1) = accumulator.overflowing_add(!data);
    let (result, overflow2) = sum.overflowing_add(cpu.get_carry_flag() as u8);
    let did_unsigned_overflow = overflow1 | overflow2;
    let did_signed_overflow = did_signed_overflow!(accumulator, !data, result);

    cpu.registers.a = result;
    cpu.set_carry_flag(did_unsigned_overflow);
    cpu.set_zero_flag(result == 0);
    cpu.set_overflow_flag(did_signed_overflow);
    cpu.set_negative_flag((result & 0b_1000_0000) != 0);
}

pub fn sbc_indirect_y(cpu: &mut CPU, buses: &mut Buses) {
    sbc(cpu, buses);

    if cpu.crossed_page {
        cpu.cycle_queue.push_back([get_effective_address, sbc]);
    }
}

pub fn sbc_absolute_indexed(cpu: &mut CPU, buses: &mut Buses) {
    sbc(cpu, buses);

    if cpu.crossed_page {
        cpu.cycle_queue.push_back([get_effective_address, sbc]);
    }
}
