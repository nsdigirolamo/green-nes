use crate::cpu::state::{Cycle, HalfCycle};

pub mod miscellaneous;
pub mod read;
pub mod read_modify_write;
pub mod single_byte;
pub mod store;
pub mod unofficial;

pub trait Instruction {
    fn get_cycles(&self, operation: HalfCycle) -> Vec<Cycle>;
}
