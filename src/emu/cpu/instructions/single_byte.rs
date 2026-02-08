use crate::emu::cpu::{cycles::*, half_cycles::*, instructions::Instruction};

pub struct SingleByte {
    pub op: HalfCycle,
}

impl Instruction<1> for SingleByte {
    fn get_cycles(&self) -> [Cycle; 1] {
        // @TODO: Some of the `operations` here are supposed to set the address
        // bus to PC + 1 but they just don't. Need to determine if this is
        // necessary or not.
        [[self.op, read_opcode]]
    }
}
