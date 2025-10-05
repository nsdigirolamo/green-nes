use crate::emu::{
    buses::Buses,
    cpu::{CPU, half_cycles::get_effective_address},
};

pub fn nop(_: &mut CPU, _: &mut Buses) {}

pub fn nop_absolute_indexed(cpu: &mut CPU, _: &mut Buses) {
    if cpu.crossed_page {
        cpu.cycle_queue.push_back([get_effective_address, nop]);
    }
}
