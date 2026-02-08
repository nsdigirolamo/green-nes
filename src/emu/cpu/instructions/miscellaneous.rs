use crate::emu::cpu::{cycles::*, half_cycles::*, instructions::Instruction};

pub struct Push {
    pub op: HalfCycle,
}

impl Instruction<2> for Push {
    fn get_cycles(&self) -> [Cycle; 2] {
        [[get_pc_without_increment, read_data], [push_stack, self.op]]
    }
}

pub struct Pull {
    pub op: HalfCycle,
}

impl Instruction<3> for Pull {
    fn get_cycles(&self) -> [Cycle; 3] {
        [
            [get_pc_without_increment, read_data],
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
            FETCH_LOW_EFFECTIVE_ADDRESS_BYTE,
            [get_sp, read_data],
            PUSH_PC_HIGH_TO_STACK,
            PUSH_PC_LOW_TO_STACK,
            [get_pc, self.op],
        ]
    }
}

pub struct Break {}

impl Instruction<6> for Break {
    fn get_cycles(&self) -> [Cycle; 6] {
        [
            [get_pc, read_data],
            PUSH_PC_HIGH_TO_STACK,
            PUSH_PC_LOW_TO_STACK,
            [push_stack, write_break_status],
            [get_low_irq_vector, read_low_pc_address_byte],
            [get_high_irq_vector, read_high_pc_address_byte],
        ]
    }
}

pub struct ReturnFromInterrupt {
    pub op: HalfCycle,
}

impl Instruction<5> for ReturnFromInterrupt {
    fn get_cycles(&self) -> [Cycle; 5] {
        [
            [get_pc, read_data],
            [pop_stack, read_data],
            [pop_stack, self.op],
            [pop_stack, read_low_pc_address_byte],
            [get_sp, read_high_pc_address_byte],
        ]
    }
}

pub struct JumpAbsolute {
    pub op: HalfCycle,
}

impl Instruction<2> for JumpAbsolute {
    fn get_cycles(&self) -> [Cycle; 2] {
        [FETCH_LOW_EFFECTIVE_ADDRESS_BYTE, [get_pc, self.op]]
    }
}

pub struct JumpIndirect {}

impl Instruction<4> for JumpIndirect {
    fn get_cycles(&self) -> [Cycle; 4] {
        [
            [get_pc, read_low_indirect_address_byte],
            [get_pc, read_high_indirect_address_byte],
            [get_indirect_low_address_byte, read_low_pc_address_byte],
            [get_indirect_high_address_byte, read_high_pc_address_byte],
        ]
    }
}

pub struct ReturnFromSubroutine {}

impl Instruction<5> for ReturnFromSubroutine {
    fn get_cycles(&self) -> [Cycle; 5] {
        [
            [get_pc, read_data],
            [pop_stack, read_data],
            [pop_stack, read_low_pc_address_byte],
            [get_sp, read_high_pc_address_byte],
            [get_pc, read_data],
        ]
    }
}

pub struct Branch {
    pub op: HalfCycle,
}

impl Instruction<1> for Branch {
    fn get_cycles(&self) -> [Cycle; 1] {
        [[get_pc, self.op]]
    }
}
