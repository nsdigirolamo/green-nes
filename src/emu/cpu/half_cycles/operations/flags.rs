use crate::emu::{buses::Buses, cpu::CPU};

pub fn sec(cpu: &mut CPU, _: &mut Buses) {
    cpu.set_carry_flag(true);
}

pub fn clc(cpu: &mut CPU, _: &mut Buses) {
    cpu.set_carry_flag(false);
}

pub fn sed(cpu: &mut CPU, _: &mut Buses) {
    cpu.set_decimal_mode_flag(true);
}

pub fn cld(cpu: &mut CPU, _: &mut Buses) {
    cpu.set_decimal_mode_flag(false);
}

pub fn sei(cpu: &mut CPU, _: &mut Buses) {
    cpu.set_interrupt_disable_flag(true);
}

pub fn cli(cpu: &mut CPU, _: &mut Buses) {
    cpu.set_interrupt_disable_flag(false);
}

pub fn clv(cpu: &mut CPU, _: &mut Buses) {
    cpu.set_overflow_flag(false);
}
