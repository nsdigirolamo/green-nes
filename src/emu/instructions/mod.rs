use crate::emu::state::{Cycle, HalfCycle};

pub mod miscellaneous;
pub mod read;
pub mod read_modify_write;
pub mod store;

pub trait Instruction {
    fn get_cycles(&self, operation: HalfCycle) -> Vec<Cycle>;
}
