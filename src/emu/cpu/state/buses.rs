use crate::{emu::cartridge::Cartridge, emu::cpu::state::memory::Memory};

pub struct Buses {
    memory: Memory,
    // External Buses
    pub addr: (u8, u8), // Address Bus (ABH, ABL)
    pub data: u8,       // Data Bus
    // Internal Buses
    pub base_addr: (u8, u8),      // Base Address Bus (BAH, BAL)
    pub effective_addr: (u8, u8), // Effective Address Bus (ADH, ADL)
    pub indirect_addr: (u8, u8),  // Indirect Address Bus (IAH, IAL)
}

impl Buses {
    pub fn new(cartridge: Cartridge) -> Self {
        Buses {
            memory: Memory::new(cartridge),
            addr: (0, 0),
            data: 0,
            base_addr: (0, 0),
            effective_addr: (0, 0),
            indirect_addr: (0, 0),
        }
    }

    pub fn peek(&self, address: (u8, u8)) -> u8 {
        self.memory.read(address)
    }

    pub fn read(&mut self, address: (u8, u8)) -> u8 {
        let data = self.memory.read(address);
        self.data = data;

        data
    }

    pub fn write(&mut self, address: (u8, u8), data: u8) {
        self.data = data;
        self.memory.write(address, data)
    }

    pub fn load_cartridge(&mut self, cartridge: Cartridge) {
        self.memory.cartridge = cartridge;
    }
}
