const NAMETABLE_SIZE: usize = 1024;

#[derive(Copy, Clone)]
pub struct Nametable {
    memory: [u8; NAMETABLE_SIZE],
}

impl Nametable {
    pub fn new(&self, memory: [u8; NAMETABLE_SIZE]) -> Self {
        Self { memory }
    }

    pub fn read(&self, addr: u16) -> u8 {
        let mapped_addr = addr as usize % NAMETABLE_SIZE;

        self.memory[mapped_addr]
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        let mapped_addr = addr as usize % NAMETABLE_SIZE;
        self.memory[mapped_addr] = data;
    }
}

impl Default for Nametable {
    fn default() -> Self {
        Self {
            memory: [0; NAMETABLE_SIZE],
        }
    }
}
