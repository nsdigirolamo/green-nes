use crate::emu::{
    buses::Buses,
    cpu::{CPU, half_cycles::operations::other::nop},
};

pub fn do_branch(cpu: &mut CPU, buses: &mut Buses, condition: bool) {
    let offset = buses.read() as i8;

    if !condition {
        return;
    }

    let (pch, pcl) = cpu.registers.pc;
    let (pcl_offset, overflow) = pcl.overflowing_add_signed(offset);

    let pch_offset = if overflow && offset < 0 {
        pch.wrapping_sub(1)
    } else if overflow && offset > 0 {
        pch.wrapping_add(1)
    } else {
        pch
    };

    cpu.crossed_page = overflow;

    // TODO: Don't use NOPs below, actually do something.

    cpu.cycle_queue.push_back([nop, nop]);

    if cpu.crossed_page {
        cpu.cycle_queue.push_back([nop, nop]);
    }

    cpu.registers.pc = (pch_offset, pcl_offset)
}

pub fn bcs(cpu: &mut CPU, buses: &mut Buses) {
    do_branch(cpu, buses, cpu.get_carry_flag());
}

pub fn bcc(cpu: &mut CPU, buses: &mut Buses) {
    do_branch(cpu, buses, !cpu.get_carry_flag())
}

pub fn beq(cpu: &mut CPU, buses: &mut Buses) {
    do_branch(cpu, buses, cpu.get_zero_flag())
}

pub fn bne(cpu: &mut CPU, buses: &mut Buses) {
    do_branch(cpu, buses, !cpu.get_zero_flag())
}

pub fn bmi(cpu: &mut CPU, buses: &mut Buses) {
    do_branch(cpu, buses, cpu.get_negative_flag())
}

pub fn bpl(cpu: &mut CPU, buses: &mut Buses) {
    do_branch(cpu, buses, !cpu.get_negative_flag())
}

pub fn bvs(cpu: &mut CPU, buses: &mut Buses) {
    do_branch(cpu, buses, cpu.get_overflow_flag())
}

pub fn bvc(cpu: &mut CPU, buses: &mut Buses) {
    do_branch(cpu, buses, !cpu.get_overflow_flag())
}
