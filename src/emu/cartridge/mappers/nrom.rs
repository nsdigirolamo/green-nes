use crate::emu::{
    cartridge::{NametableMirroring, ines::INes, mappers::Mapper},
    error::{CartridgeError, Error},
};

const PRG_BANKS_MIN_ADDR: u16 = 0x8000;
const PRG_BANKS_MAX_ADDR: u16 = 0xFFFF;
const PRG_ROM_SIZE: usize = (PRG_BANKS_MAX_ADDR - PRG_BANKS_MIN_ADDR + 1) as usize;
const PRG_ROM_BANK_SIZE: usize = PRG_ROM_SIZE / 2;

const CHR_MIN_ADDR: u16 = 0x0000;
const CHR_MAX_ADDR: u16 = 0x1FFF;
const CHR_ROM_SIZE: usize = (CHR_MAX_ADDR - CHR_MIN_ADDR + 1) as usize;

pub struct NROM {
    prg_rom: [u8; PRG_ROM_SIZE],
    chr_rom: [u8; CHR_ROM_SIZE],

    nametable_arrangement: NametableMirroring,
}

impl NROM {
    pub fn new(ines: INes) -> Result<Self, Error> {
        let prg_rom = create_prg_rom(ines.prg_data)?;
        let chr_rom = create_chr_rom(ines.chr_data)?;
        let nametable_arrangement = match ines.nametable_arrangement {
            true => NametableMirroring::Horizontal,
            false => NametableMirroring::Vertical,
        };

        Ok(NROM {
            prg_rom,
            chr_rom,
            nametable_arrangement,
        })
    }
}

fn create_prg_rom(prg_data: Vec<u8>) -> Result<[u8; PRG_ROM_SIZE], Error> {
    if prg_data.len() > PRG_ROM_SIZE {
        return Err(CartridgeError::NotSupported {
            message: "NROM mapper failed: PRG ROM too large".to_string(),
        }
        .into());
    }

    let mut data = [0; PRG_ROM_SIZE];
    let has_two_banks = prg_data.len() > PRG_ROM_BANK_SIZE;
    if has_two_banks {
        for (i, byte) in prg_data.iter().enumerate() {
            data[i] = *byte;
        }
    } else {
        // Two mirrored 16 KiB PRG banks
        for (i, byte) in prg_data.iter().enumerate() {
            data[i] = *byte;
            data[PRG_ROM_BANK_SIZE + i] = *byte;
        }
    }

    Ok(data)
}

fn create_chr_rom(chr_data: Vec<u8>) -> Result<[u8; CHR_ROM_SIZE], Error> {
    if chr_data.len() > CHR_ROM_SIZE {
        return Err(CartridgeError::NotSupported {
            message: "NROM mapper failed: CHR ROM too large".to_string(),
        }
        .into());
    }

    let mut data = [0; CHR_ROM_SIZE];
    for (i, byte) in chr_data.iter().enumerate() {
        data[i] = *byte;
    }

    Ok(data)
}

impl Mapper for NROM {
    fn prg_read(&self, addr: u16) -> u8 {
        match addr {
            PRG_BANKS_MIN_ADDR..=PRG_BANKS_MAX_ADDR => {
                let mapped_addr = addr - PRG_BANKS_MIN_ADDR;
                self.prg_rom[mapped_addr as usize]
            }
            _ => 0, // ignore; unmapped
        }
    }

    fn prg_write(&mut self, _: u16, _: u8) {
        // ignore; PRG ROM is fixed
    }

    fn chr_read(&self, addr: u16) -> u8 {
        match addr {
            CHR_MIN_ADDR..=CHR_MAX_ADDR => {
                let mapped_addr = addr;
                self.chr_rom[mapped_addr as usize]
            }
            _ => 0, // ignore; unmapped
        }
    }

    fn chr_write(&mut self, _: u16, _: u8) {
        // ignore; CHR ROM is fixed
    }

    fn get_nametable_arrangement(&self) -> NametableMirroring {
        self.nametable_arrangement
    }
}
