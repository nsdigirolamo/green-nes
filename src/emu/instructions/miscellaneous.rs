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
            Miscellaneous::Push => panic!("miscellaneous push not implemented"),
            Miscellaneous::Pull => panic!("miscellaneous pull not implemented"),
            Miscellaneous::JumpToSubroutine => {
                panic!("miscellaneous jump to subroutine not implemented")
            }
            Miscellaneous::Break => panic!("miscellaneous break not implemented"),
            Miscellaneous::ReturnFromInterrupt => {
                panic!("miscellaneous return from interrupt not implemented")
            }
            Miscellaneous::JumpAbsolute => panic!("miscellaneous jump absolute not implemented"),
            Miscellaneous::JumpIndirect => panic!("miscellaneous jump indirect not implemented"),
            Miscellaneous::ReturnFromSubroutine => {
                panic!("miscellaneous return from subroutine not implemented")
            }
            Miscellaneous::Branch => panic!("miscellaneous branch not implemented"),
        };
    }
}
