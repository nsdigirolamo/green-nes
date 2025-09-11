use crate::concat_u8;

pub const MEMORY_SIZE: usize = 65536;

pub struct Buses {
    memory: [u8; MEMORY_SIZE],
    // External Buses
    pub addr: (u8, u8), // Address Bus (ABH, ABL)
    pub data: u8,       // Data Bus
    // Internal Buses
    pub base_addr: (u8, u8),      // Base Address Bus (BAH, BAL)
    pub effective_addr: (u8, u8), // Effective Address Bus (ADH, ADL)
    pub indirect_addr: (u8, u8),  // Indirect Address Bus (IAH, IAL)
}

impl Default for Buses {
    fn default() -> Self {
        Buses {
            memory: [0u8; MEMORY_SIZE],
            addr: (0, 0),
            data: 0,
            base_addr: (0, 0),
            effective_addr: (0, 0),
            indirect_addr: (0, 0),
        }
    }
}

impl Buses {
    pub fn read(&mut self, address: (u8, u8)) -> u8 {
        let data = self.memory[concat_u8!(address.0, address.1) as usize];
        self.data = data;

        data
    }

    pub fn write(&mut self, address: (u8, u8), data: u8) {
        self.data = data;
        self.memory[concat_u8!(address.0, address.1) as usize] = data;
    }
}
