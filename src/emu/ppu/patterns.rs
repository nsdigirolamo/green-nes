use std::fmt;

use crate::emu::{
    cartridge::Cartridge,
    ppu::buses::{PATTERN_TABLE_SIZE, PATTERN_TABLES_START_ADDR},
};

pub const PATTERN_PLANE_SIZE: u16 = 8;
pub const PATTERN_PLANE_COUNT: u16 = 2;
pub const PATTERN_SIZE: u16 = PATTERN_PLANE_SIZE * PATTERN_PLANE_COUNT;

pub const PATTERN_HEIGHT_PIXELS: u16 = PATTERN_PLANE_SIZE;
pub const PATTERN_WIDTH_PIXELS: u16 = PATTERN_HEIGHT_PIXELS;

pub const PATTERNS_PER_TABLE: u16 = PATTERN_TABLE_SIZE / PATTERN_SIZE;
pub const PATTERN_TABLES_COUNT: u16 = 2;

#[derive(Clone, Copy, Default)]
pub struct Pattern {
    pub data: [[(bool, bool); PATTERN_WIDTH_PIXELS as usize]; PATTERN_HEIGHT_PIXELS as usize],
}

impl fmt::Debug for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result: String = "".to_string();

        for row_of_pixels in self.data.iter() {
            for pixel in row_of_pixels.iter() {
                let char: &'static str = match pixel {
                    (true, true) => "3",
                    (true, false) => "2",
                    (false, true) => "1",
                    (false, false) => ".",
                };

                result = format!("{result}{char}");
            }
            result = format!("{result}\n");
        }

        write!(f, "{result}")
    }
}

#[derive(Clone, Copy)]
pub struct PatternTable {
    pub data: [Pattern; PATTERNS_PER_TABLE as usize],
}

impl Default for PatternTable {
    fn default() -> Self {
        Self {
            data: [Pattern::default(); PATTERNS_PER_TABLE as usize],
        }
    }
}

impl fmt::Debug for PatternTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result: String = "".to_string();

        for (i, pattern) in self.data.iter().enumerate() {
            result = format!("{result}\n{i}\n{pattern:?}");
        }

        write!(f, "{result}")
    }
}

pub fn dump_pattern_tables(cart: Cartridge) -> [PatternTable; PATTERN_TABLES_COUNT as usize] {
    let mapper = cart.mapper.borrow();
    let mut pattern_tables = [PatternTable::default(); PATTERN_TABLES_COUNT as usize];

    for (table_index, table) in pattern_tables.iter_mut().enumerate() {
        for (pattern_index, pattern) in table.data.iter_mut().enumerate() {
            let addr = PATTERN_TABLES_START_ADDR
                + (table_index as u16 * PATTERN_TABLE_SIZE)
                + (pattern_index as u16 * PATTERN_SIZE);

            for (row, row_of_pixels) in pattern.data.iter_mut().enumerate() {
                let byte_offset = row as u16;
                let low_bits = mapper.chr_read(addr + byte_offset);
                let high_bits = mapper.chr_read(addr + PATTERN_HEIGHT_PIXELS + byte_offset);

                for (col, pixel) in row_of_pixels.iter_mut().enumerate() {
                    let mask = 0b_1000_0000 >> col;
                    *pixel = ((high_bits >> col & mask) != 0, (low_bits & mask) != 0);
                }
            }
        }
    }

    pattern_tables
}
