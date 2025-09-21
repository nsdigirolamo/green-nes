use crate::{concat_u8, emu::cartridge::Cartridge};

pub const RAM_MIN_ADDRESS: u16 = 0x0000;
pub const RAM_MAX_ADDRESSS: u16 = 0x07FF;
pub const RAM_SIZE: usize = (RAM_MAX_ADDRESSS - RAM_MIN_ADDRESS + 1) as usize;
pub const RAM_MAX_MIRRORED_ADDRESS: u16 = 0x1FFF;

pub const PPU_REGISTER_MIN_ADDRESS: u16 = 0x2000;
pub const PPU_REGISTER_MAX_ADDRESS: u16 = 0x2007;
pub const PPU_SIZE: usize = (PPU_REGISTER_MAX_ADDRESS - PPU_REGISTER_MIN_ADDRESS + 1) as usize;
pub const PPU_REGISTER_MAX_MIRRORED_ADDRESS: u16 = 0x3FFF;

pub const APU_IO_MIN_ADDRESS: u16 = 0x4000;
pub const APU_IO_MAX_ADDRESS: u16 = 0x4017;
pub const TEST_APU_IO_MIN_ADDRESS: u16 = 0x4018;
pub const TEST_APU_IO_MAX_ADDRESS: u16 = 0x401F;

pub const UNMAPPED_MIN_ADDRESS: u16 = 0x4020;
pub const UNMAPPED_MAX_ADDRESS: u16 = 0xFFFF;

pub const CARTRIDGE_RAM_MIN_ADDRESS: u16 = 0x6000;
pub const CARTRIDGE_RAM_MAX_ADDRESS: u16 = 0x2000;

pub const CARTRIDGE_ROM_MAPPER_MIN_ADDRESS: u16 = 0x8000;
pub const CARTRIDGE_ROM_MAPPER_MAX_ADDRESS: u16 = 0xFFFF;

pub const MEMORY_SIZE: usize = 65536;

pub struct Memory {
    ram: [u8; RAM_SIZE],
    pub cartridge: Cartridge,
}

impl Memory {
    pub fn new(cartridge: Cartridge) -> Self {
        Memory {
            ram: [0; RAM_SIZE],
            cartridge,
        }
    }

    pub fn read(&self, address: (u8, u8)) -> u8 {
        let address = concat_u8!(address.0, address.1);
        match address {
            RAM_MIN_ADDRESS..=RAM_MAX_MIRRORED_ADDRESS => {
                let mirror_address = address & 0b_0000_0111_1111_1111;
                self.ram[mirror_address as usize]
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
            CARTRIDGE_ROM_MAPPER_MIN_ADDRESS..=CARTRIDGE_ROM_MAPPER_MAX_ADDRESS => {
                self.read_prg_rom(address)
            }
            _ => {
                todo!("unmapped")
            }
        }
    }

    pub fn write(&mut self, address: (u8, u8), data: u8) {
        let address = concat_u8!(address.0, address.1);
        match address {
            RAM_MIN_ADDRESS..=RAM_MAX_MIRRORED_ADDRESS => {
                let mirror_address = address & 0b_0000_0111_1111_1111;
                self.ram[mirror_address as usize] = data;
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

    fn read_prg_rom(&self, address: u16) -> u8 {
        let address = address - CARTRIDGE_ROM_MAPPER_MIN_ADDRESS;

        let address = if self.cartridge.prg_rom.len() == 0x4000 && address >= 0x4000 {
            address % 0x4000
        } else {
            address
        };

        self.cartridge.prg_rom[address as usize]
    }
}
