use crate::emu::cpu::{
    cycles::Cycle,
    half_cycles::{HalfCycle, read_opcode},
    instructions::Instruction,
};

pub enum SingleByte {
    Default,
}

impl Instruction for SingleByte {
    fn get_cycles(&self, operation: HalfCycle) -> Vec<Cycle> {
        // @TODO: Some of the `operations` here are supposed to set the address
        // bus to PC + 1 but they just don't. Need to determine if this is
        // necessary or not.
        vec![[operation, read_opcode]]
    }
}
