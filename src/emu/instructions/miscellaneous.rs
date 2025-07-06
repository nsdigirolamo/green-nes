use crate::emu::{
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
    fn get_cycles(&self, _operation: HalfCycle) -> Vec<Cycle> {
        match self {
            Miscellaneous::Push => panic!("help"),
            Miscellaneous::Pull => panic!("help"),
            Miscellaneous::JumpToSubroutine => panic!("help"),
            Miscellaneous::Break => panic!("help"),
            Miscellaneous::ReturnFromInterrupt => panic!("help"),
            Miscellaneous::JumpAbsolute => panic!("help"),
            Miscellaneous::JumpIndirect => panic!("help"),
            Miscellaneous::ReturnFromSubroutine => panic!("help"),
            Miscellaneous::Branch => panic!("help"),
        };
    }
}
