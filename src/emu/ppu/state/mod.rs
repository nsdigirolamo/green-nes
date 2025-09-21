use crate::emu::ppu::state::registers::Registers;

pub mod registers;

pub const MAX_MEMORY_ADDRESS: u16 = 0x3FFF;

pub const MEMORY_SIZE: usize = MAX_MEMORY_ADDRESS as usize + 1;
pub const OAM_SIZE: usize = 256;

#[derive(Default)]
pub struct State {
    pub registers: Registers,
}
