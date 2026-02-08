use crate::emu::{
    buses::Buses,
    cpu::{CPU, half_cycles::get_effective_address},
};

/// # No Operation
///
/// Does nothing.
pub fn nop(_: &mut CPU, _: &mut Buses) {}

/// # No Operation
///
/// Does nothing. Uses an additional cycle if a page is crossed.
pub fn nop_abs_index(cpu: &mut CPU, _: &mut Buses) {
    if cpu.crossed_page {
        cpu.cycle_queue.push_back([get_effective_address, nop]);
    }
}
