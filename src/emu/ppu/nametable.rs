use crate::emu::ppu::buses::{ATTRIBUTE_TABLE_SIZE, NAMETABLE_SIZE};

#[derive(Copy, Clone)]
pub struct Nametable {
    memory: [u8; (NAMETABLE_SIZE + ATTRIBUTE_TABLE_SIZE) as usize],
}

impl Nametable {
    pub fn new(&self, memory: [u8; (NAMETABLE_SIZE + ATTRIBUTE_TABLE_SIZE) as usize]) -> Self {
        Self { memory }
    }

    pub fn read(&self, addr: u16) -> u8 {
        let mapped_addr = addr % (NAMETABLE_SIZE + ATTRIBUTE_TABLE_SIZE);

        self.memory[mapped_addr as usize]
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        let mapped_addr = addr % (NAMETABLE_SIZE + ATTRIBUTE_TABLE_SIZE);
        self.memory[mapped_addr as usize] = data;
    }
}

impl Default for Nametable {
    fn default() -> Self {
        Self {
            memory: [0; (NAMETABLE_SIZE + ATTRIBUTE_TABLE_SIZE) as usize],
        }
    }
}
