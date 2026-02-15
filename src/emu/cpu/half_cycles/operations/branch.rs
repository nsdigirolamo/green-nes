use crate::emu::{
    buses::Buses,
    cpu::{CPU, half_cycles::operations::other::nop},
};

/// # Branch If Carry Set
///
/// If the carry flag is set, branch to a nearby location by adding the branch
/// offset to the program counter.
pub fn bcs(cpu: &mut CPU, buses: &mut Buses) {
    do_branch(cpu, buses, cpu.registers.psr.get_carry());
}

/// # Branch If Carry Clear
///
/// If the carry flag is clear, branch to a nearby location by adding the branch
/// offset to the program counter.
pub fn bcc(cpu: &mut CPU, buses: &mut Buses) {
    do_branch(cpu, buses, !cpu.registers.psr.get_carry())
}

/// # Branch If Equal
///
/// If the zero flag is set, branch to a nearby location by adding the branch
/// offset to the program counter.
pub fn beq(cpu: &mut CPU, buses: &mut Buses) {
    do_branch(cpu, buses, cpu.registers.psr.get_zero())
}

/// # Branch If Not Equal
///
/// If the zero flag is clear, branch to a nearby location by adding the branch
/// offset to the program counter.
pub fn bne(cpu: &mut CPU, buses: &mut Buses) {
    do_branch(cpu, buses, !cpu.registers.psr.get_zero())
}

/// # Branch If Minus
///
/// If the negative flag is set, branch to a nearby location by adding the
/// branch offset to the program counter.
pub fn bmi(cpu: &mut CPU, buses: &mut Buses) {
    do_branch(cpu, buses, cpu.registers.psr.get_negative())
}

/// # Branch If Plus
///
/// If the negative flag is clear, branch to a nearby location by adding the
/// branch offset to the program counter.
pub fn bpl(cpu: &mut CPU, buses: &mut Buses) {
    do_branch(cpu, buses, !cpu.registers.psr.get_negative())
}

/// # Branch If Overflow Set
///
/// If the overflow flag is set, branch to a nearby location by adding the
/// branch offset to the program counter.
pub fn bvs(cpu: &mut CPU, buses: &mut Buses) {
    do_branch(cpu, buses, cpu.registers.psr.get_overflow())
}

/// # Branch If Overflow Clear
///
/// If the overflow flag is clear, branch to a nearby location by adding the
/// branch offset to the program counter.
pub fn bvc(cpu: &mut CPU, buses: &mut Buses) {
    do_branch(cpu, buses, !cpu.registers.psr.get_overflow())
}

fn do_branch(cpu: &mut CPU, buses: &mut Buses, condition: bool) {
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
