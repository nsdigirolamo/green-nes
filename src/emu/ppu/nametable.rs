use crate::emu::ppu::{frame::PATTERN_COLS_PER_FRAME, patterns::PATTERN_TABLES_END_ADDR};

pub const NAMETABLE_SIZE: u16 = 960;
pub const ATTRIBUTE_TABLE_SIZE: u16 = 64;
pub const NAMETABLES_COUNT: u16 = 4;

pub const NAMETABLES_START_ADDR: u16 = PATTERN_TABLES_END_ADDR;

pub const NAMETABLE_0_START_ADDR: u16 = NAMETABLES_START_ADDR;
pub const NAMETABLE_0_END_ADDR: u16 = NAMETABLE_0_START_ADDR + NAMETABLE_SIZE;
pub const ATTRIBUTE_TABLE_0_START_ADDR: u16 = NAMETABLE_0_END_ADDR;
pub const ATTRIBUTE_TABLE_0_END_ADDR: u16 = ATTRIBUTE_TABLE_0_START_ADDR + ATTRIBUTE_TABLE_SIZE;

pub const NAMETABLE_1_START_ADDR: u16 = ATTRIBUTE_TABLE_0_END_ADDR;
pub const NAMETABLE_1_END_ADDR: u16 = NAMETABLE_1_START_ADDR + NAMETABLE_SIZE;
pub const ATTRIBUTE_TABLE_1_START_ADDR: u16 = NAMETABLE_1_END_ADDR;
pub const ATTRIBUTE_TABLE_1_END_ADDR: u16 = ATTRIBUTE_TABLE_1_START_ADDR + ATTRIBUTE_TABLE_SIZE;

pub const NAMETABLE_2_START_ADDR: u16 = ATTRIBUTE_TABLE_1_END_ADDR;
pub const NAMETABLE_2_END_ADDR: u16 = NAMETABLE_2_START_ADDR + NAMETABLE_SIZE;
pub const ATTRIBUTE_TABLE_2_START_ADDR: u16 = NAMETABLE_2_END_ADDR;
pub const ATTRIBUTE_TABLE_2_END_ADDR: u16 = ATTRIBUTE_TABLE_2_START_ADDR + ATTRIBUTE_TABLE_SIZE;

pub const NAMETABLE_3_START_ADDR: u16 = ATTRIBUTE_TABLE_2_END_ADDR;
pub const NAMETABLE_3_END_ADDR: u16 = NAMETABLE_3_START_ADDR + NAMETABLE_SIZE;
pub const ATTRIBUTE_TABLE_3_START_ADDR: u16 = NAMETABLE_3_END_ADDR;
pub const ATTRIBUTE_TABLE_3_END_ADDR: u16 = ATTRIBUTE_TABLE_3_START_ADDR + ATTRIBUTE_TABLE_SIZE;

pub const NAMETABLES_END_ADDR: u16 = ATTRIBUTE_TABLE_3_END_ADDR;

#[derive(Clone, Copy)]
pub struct Nametable {
    data: [u8; (NAMETABLE_SIZE + ATTRIBUTE_TABLE_SIZE) as usize],
}

impl Nametable {
    pub fn new(data: [u8; (NAMETABLE_SIZE + ATTRIBUTE_TABLE_SIZE) as usize]) -> Self {
        Self { data }
    }

    pub fn read(&self, addr: u16) -> u8 {
        let mapped_addr = addr % (NAMETABLE_SIZE + ATTRIBUTE_TABLE_SIZE);

        self.data[mapped_addr as usize]
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        let mapped_addr = addr % (NAMETABLE_SIZE + ATTRIBUTE_TABLE_SIZE);
        self.data[mapped_addr as usize] = data;
    }
}

impl std::fmt::Debug for Nametable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result: String = "".to_string();

        for (i, entry) in self.iter().enumerate() {
            result = format!("{result} {entry:>3}");
            if (i as u16 % PATTERN_COLS_PER_FRAME) == PATTERN_COLS_PER_FRAME - 1 {
                result = format!("{result}\n")
            }
        }

        write!(f, "{result}")
    }
}

impl Default for Nametable {
    fn default() -> Self {
        Self {
            data: [0u8; (NAMETABLE_SIZE + ATTRIBUTE_TABLE_SIZE) as usize],
        }
    }
}

impl std::ops::Deref for Nametable {
    type Target = [u8; (NAMETABLE_SIZE + ATTRIBUTE_TABLE_SIZE) as usize];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
