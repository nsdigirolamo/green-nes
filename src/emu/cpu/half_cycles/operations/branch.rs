use crate::emu::{
    buses::Buses,
    cpu::{
        CPU,
        half_cycles::{branch_across_page, get_effective_address, read_opcode},
    },
};

pub fn do_branch(cpu: &mut CPU, buses: &mut Buses, condition: bool) {
    let offset = buses.read();

    if condition {
        let (pc_high, pc_low) = cpu.registers.pc;
        let (pc_low_offset, overflow) = pc_low.overflowing_add_signed(offset as i8);

        cpu.buses.effective_addr = (pc_high, pc_low_offset);
        cpu.crossed_page = overflow;

        cpu.cycle_queue
            .push_back([get_effective_address, read_opcode]);

        if cpu.crossed_page {
            cpu.cycle_queue.push_back([branch_across_page, read_opcode]);
        } else {
            cpu.registers.pc = (pc_high, pc_low_offset);
        }
    }
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
