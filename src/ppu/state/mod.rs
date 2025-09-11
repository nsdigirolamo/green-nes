use crate::ppu::state::registers::Registers;

pub mod registers;

pub const MAX_MEMORY_ADDRESS: u16 = 0x3FFF;

pub const MEMORY_SIZE: usize = MAX_MEMORY_ADDRESS as usize + 1;
pub const OAM_SIZE: usize = 256;

pub struct State {
    pub memory: [u8; MEMORY_SIZE],
    pub oam: [u8; OAM_SIZE], // Object Attribute Memory
    pub registers: Registers,
}

impl Default for State {
    fn default() -> Self {
        State {
            memory: [0u8; MEMORY_SIZE],
            oam: [0u8; OAM_SIZE],
            registers: Registers::default(),
        }
    }
}
