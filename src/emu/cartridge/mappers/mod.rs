use crate::emu::cartridge::NametableMirroring;

pub mod nrom;

pub const ROWS_PER_PATTERN_TABLE: usize = 16;
pub const COLS_PER_PATTERN_TABLE: usize = 16;
pub const TILES_PER_PATTERN_TABLE: usize = ROWS_PER_PATTERN_TABLE * COLS_PER_PATTERN_TABLE;
pub const TILE_WIDTH: usize = 8;
pub const PLANE_COUNT: usize = 2;
pub const PATTERN_TABLE_SIZE: usize = TILE_WIDTH * PLANE_COUNT * TILES_PER_PATTERN_TABLE;

pub trait Mapper {
    fn prg_read(&self, addr: u16) -> u8;
    fn prg_write(&mut self, addr: u16, data: u8);
    fn chr_read(&self, addr: u16) -> u8;
    fn chr_write(&mut self, addr: u16, data: u8);
    fn get_nametable_arrangement(&self) -> NametableMirroring;
}
