use crate::emu::cpu::cycles::*;

pub mod miscellaneous;
pub mod read;
pub mod read_modify_write;
pub mod single_byte;
pub mod store;
pub mod unofficial;

pub trait Instruction<const CYCLE_COUNT: usize> {
    fn get_cycles(&self) -> [Cycle; CYCLE_COUNT];
}
