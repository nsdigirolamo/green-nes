use crate::cpu::{
    cycles::FETCH_LOW_EFFECTIVE_ADDRESS_BYTE,
    half_cycles::{
        get_high_interrupt_vector, get_indirect_high_address_byte, get_indirect_low_address_byte,
        get_low_interrupt_vector, get_pc, get_pc_without_increment, get_sp, pop_stack, push_stack,
        read_data, read_high_effective_address_byte, read_high_indirect_address_byte,
        read_high_pc_address_byte, read_low_effective_address_byte, read_low_indirect_address_byte,
        read_low_pc_address_byte, write_pc_high, write_pc_low, write_status,
    },
    instructions::Instruction,
    state::{Cycle, HalfCycle},
};

pub enum Miscellaneous {
    Push,
    Pull,
    JumpToSubroutine,
    Break,
    ReturnFromInterrupt,
    JumpAbsolute,
    JumpIndirect,
    ReturnFromSubroutine,
    Branch,
}

impl Instruction for Miscellaneous {
    fn get_cycles(&self, operation: HalfCycle) -> Vec<Cycle> {
        match self {
            Miscellaneous::Push => vec![
                [get_pc_without_increment, read_data],
                [push_stack, operation],
            ],
            Miscellaneous::Pull => vec![
                [get_pc_without_increment, read_data],
                [pop_stack, read_data],
                [get_sp, operation],
            ],
            Miscellaneous::JumpToSubroutine => vec![
                FETCH_LOW_EFFECTIVE_ADDRESS_BYTE,
                [get_sp, read_data],
                [push_stack, write_pc_high],
                [push_stack, write_pc_low],
                [get_pc, operation],
            ],
            Miscellaneous::Break => vec![
                [get_pc, read_data],
                [push_stack, write_pc_high],
                [push_stack, write_pc_low],
                [push_stack, write_status],
                [get_low_interrupt_vector, read_high_effective_address_byte],
                [get_high_interrupt_vector, read_low_effective_address_byte],
            ],
            Miscellaneous::ReturnFromInterrupt => vec![
                [get_pc, read_data],
                [pop_stack, read_data],
                [pop_stack, operation],
                [pop_stack, read_low_pc_address_byte],
                [get_sp, read_high_pc_address_byte],
            ],
            Miscellaneous::JumpAbsolute => {
                vec![FETCH_LOW_EFFECTIVE_ADDRESS_BYTE, [get_pc, operation]]
            }
            Miscellaneous::JumpIndirect => vec![
                [get_pc, read_low_indirect_address_byte],
                [get_pc, read_high_indirect_address_byte],
                [get_indirect_low_address_byte, read_low_pc_address_byte],
                [get_indirect_high_address_byte, read_high_pc_address_byte],
            ],
            Miscellaneous::ReturnFromSubroutine => vec![
                [get_pc, read_data],
                [pop_stack, read_data],
                [pop_stack, read_low_pc_address_byte],
                [get_sp, read_high_pc_address_byte],
                [get_pc, read_data],
            ],
            Miscellaneous::Branch => vec![[get_pc, operation]],
        }
    }
}
