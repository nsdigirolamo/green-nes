use crate::emu::ppu::mappings::{ATTRIBUTE_TABLE_SIZE, NAMETABLE_SIZE};

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
