use crate::emu::{
    cycles::FETCH_LOW_EFFECTIVE_ADDRESS_BYTE,
    half_cycles::{
        get_pc_address, get_pc_without_increment, get_sp_address, pop_stack, push_stack, read_data,
        write_pc_high, write_pc_low,
    },
    instructions::Instruction,
    operations::jump::jmp_absolute,
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
                [pop_stack, operation],
            ],
            Miscellaneous::JumpToSubroutine => vec![
                FETCH_LOW_EFFECTIVE_ADDRESS_BYTE,
                [get_sp_address, read_data],
                [push_stack, write_pc_high],
                [push_stack, write_pc_low],
                [get_pc_address, operation],
            ],
            Miscellaneous::Break => panic!("miscellaneous break not implemented"),
            Miscellaneous::ReturnFromInterrupt => {
                panic!("miscellaneous return from interrupt not implemented")
            }
            Miscellaneous::JumpAbsolute => vec![
                FETCH_LOW_EFFECTIVE_ADDRESS_BYTE,
                [get_pc_address, jmp_absolute],
            ],
            Miscellaneous::JumpIndirect => panic!("miscellaneous jump indirect not implemented"),
            Miscellaneous::ReturnFromSubroutine => {
                panic!("miscellaneous return from subroutine not implemented")
            }
            Miscellaneous::Branch => vec![[get_pc_address, operation]],
        }
    }
}
