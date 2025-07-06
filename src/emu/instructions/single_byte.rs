use crate::emu::{
    half_cycles::read_opcode,
    instructions::Instruction,
    state::{Cycle, HalfCycle},
};

pub enum SingleByte {
    Default,
}

impl Instruction for SingleByte {
    fn get_cycles(&self, operation: HalfCycle) -> Vec<Cycle> {
        vec![[operation, read_opcode]]
    }
}
