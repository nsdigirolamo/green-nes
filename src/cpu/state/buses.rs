use crate::concat_u8;

pub const RAM_MIN_ADDRESS: u16 = 0x0000;
pub const RAM_MAX_ADDRESSS: u16 = 0x07FF;
pub const RAM_MAX_MIRRORED_ADDRESS: u16 = 0x1FFF;

pub const PPU_REGISTER_MIN_ADDRESS: u16 = 0x2000;
pub const PPU_REGISTER_MAX_ADDRESS: u16 = 0x2007;
pub const PPU_REGISTER_MAX_MIRRORED_ADDRESS: u16 = 0x3FFF;

pub const APU_IO_MIN_ADDRESS: u16 = 0x4000;
pub const APU_IO_MAX_ADDRESS: u16 = 0x4017;

pub const TEST_APU_IO_MIN_ADDRESS: u16 = 0x4018;
pub const TEST_APU_IO_MAX_ADDRESS: u16 = 0x401F;

pub const UNMAPPED_MIN_ADDRESS: u16 = 0x4020;
pub const UNMAPPED_MAX_ADDRESS: u16 = 0xFFFF;

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
        let address = concat_u8!(address.0, address.1);

        match address {
            RAM_MIN_ADDRESS..=RAM_MAX_MIRRORED_ADDRESS => {
                let mirror_address = address & 0b_0000_0111_1111_1111;
                let data = self.memory[mirror_address as usize];
                self.data = data;

                data
            }
            PPU_REGISTER_MIN_ADDRESS..=PPU_REGISTER_MAX_MIRRORED_ADDRESS => {
                todo!("ppu not yet supported")
            }
            APU_IO_MIN_ADDRESS..=APU_IO_MAX_ADDRESS => {
                todo!("apu not yet supported")
            }
            TEST_APU_IO_MIN_ADDRESS..=TEST_APU_IO_MAX_ADDRESS => {
                todo!("apu test not yet supported")
            }
            UNMAPPED_MIN_ADDRESS..=UNMAPPED_MAX_ADDRESS => {
                todo!("unmapped region")
            }
        }
    }

    pub fn write(&mut self, address: (u8, u8), data: u8) {
        let address = concat_u8!(address.0, address.1);

        match address {
            RAM_MIN_ADDRESS..=RAM_MAX_MIRRORED_ADDRESS => {
                let mirror_address = address & 0b_0000_0111_1111_1111;
                self.data = data;
                self.memory[mirror_address as usize] = data;
            }
            PPU_REGISTER_MIN_ADDRESS..=PPU_REGISTER_MAX_MIRRORED_ADDRESS => {
                todo!("ppu not yet supported")
            }
            APU_IO_MIN_ADDRESS..=APU_IO_MAX_ADDRESS => {
                todo!("apu not yet supported")
            }
            TEST_APU_IO_MIN_ADDRESS..=TEST_APU_IO_MAX_ADDRESS => {
                todo!("apu test not yet supported")
            }
            UNMAPPED_MIN_ADDRESS..=UNMAPPED_MAX_ADDRESS => {
                todo!("unmapped region")
            }
        }
    }
}
