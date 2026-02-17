use crate::emu::cpu::{cycles::*, half_cycles::*, instructions::Instruction};

pub struct Push {
    pub op: HalfCycle,
}

impl Instruction<2> for Push {
    fn get_cycles(&self) -> [Cycle; 2] {
        [[get_pc, read_data], [push_stack, self.op]]
    }
}

pub struct Pull {
    pub op: HalfCycle,
}

impl Instruction<3> for Pull {
    fn get_cycles(&self) -> [Cycle; 3] {
        [
            [get_pc, read_data],
            [pop_stack, read_data],
            [get_sp, self.op],
        ]
    }
}

pub struct JumpToSubroutine {
    pub op: HalfCycle,
}

impl Instruction<5> for JumpToSubroutine {
    fn get_cycles(&self) -> [Cycle; 5] {
        [
            GET_EFFECTIVE_ADDR_LOW_BYTE,
            [get_sp, read_data],
            [push_stack, write_pc_high_byte],
            [push_stack, write_pc_low_byte],
            [get_pc_with_inc, self.op],
        ]
    }
}

pub struct Break {}

impl Instruction<6> for Break {
    fn get_cycles(&self) -> [Cycle; 6] {
        [
            [get_pc_with_inc, read_data],
            [push_stack, write_pc_high_byte],
            [push_stack, write_pc_low_byte],
            [push_stack, write_break_status],
            [get_irq_vector_low_byte, read_pc_low_byte],
            [get_irq_vector_high_byte, read_pc_high_byte],
        ]
    }
}

pub struct ReturnFromInterrupt {
    pub op: HalfCycle,
}

impl Instruction<5> for ReturnFromInterrupt {
    fn get_cycles(&self) -> [Cycle; 5] {
        [
            [get_pc_with_inc, read_data],
            [pop_stack, read_data],
            [pop_stack, self.op],
            [pop_stack, read_pc_low_byte],
            [get_sp, read_pc_high_byte],
        ]
    }
}

pub struct JumpAbsolute {
    pub op: HalfCycle,
}

impl Instruction<2> for JumpAbsolute {
    fn get_cycles(&self) -> [Cycle; 2] {
        [GET_EFFECTIVE_ADDR_LOW_BYTE, [get_pc_with_inc, self.op]]
    }
}

pub struct JumpIndirect {}

impl Instruction<4> for JumpIndirect {
    fn get_cycles(&self) -> [Cycle; 4] {
        [
            [get_pc_with_inc, read_indirect_addr_low_byte],
            [get_pc_with_inc, read_indirect_addr_high_byte],
            [get_indirect_addr, read_pc_low_byte],
            [get_indirect_addr_high_byte, read_pc_high_byte],
        ]
    }
}

pub struct ReturnFromSubroutine {}

impl Instruction<5> for ReturnFromSubroutine {
    fn get_cycles(&self) -> [Cycle; 5] {
        [
            [get_pc_with_inc, read_data],
            [pop_stack, read_data],
            [pop_stack, read_pc_low_byte],
            [get_sp, read_pc_high_byte],
            [get_pc_with_inc, read_data],
        ]
    }
}

pub struct Branch {
    pub op: HalfCycle,
}

impl Instruction<1> for Branch {
    fn get_cycles(&self) -> [Cycle; 1] {
        [[get_pc_with_inc, self.op]]
    }
}
