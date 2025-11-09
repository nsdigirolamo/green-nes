use crate::emu::{
    cartridge::{
        NametableMirroring,
        ines::INes,
        mappers::{
            Mapper, PATTERN_TABLE_0_START_ADDR, PATTERN_TABLE_1_START_ADDR, PATTERN_TABLE_SIZE,
        },
    },
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
            message: "could not load PRG ROM".to_string(),
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
            message: "could not load CHR ROM".to_string(),
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
            _ => panic!("NROM mapper does not support reading PRG address 0x{addr:04X}"),
        }
    }

    fn prg_write(&mut self, addr: u16, data: u8) {
        panic!("NROM mapper does not support writing {data:04X} to PRG ROM address 0x{addr:04X}")
    }

    fn chr_read(&self, addr: u16) -> u8 {
        match addr {
            CHR_MIN_ADDR..=CHR_MAX_ADDR => {
                let mapped_addr = addr;
                self.chr_rom[mapped_addr as usize]
            }
            _ => panic!("NROM mapper does not support reading CHR address 0x{addr:04X}"),
        }
    }

    fn chr_write(&mut self, addr: u16, data: u8) {
        panic!("NROM mapper does not support writing {data:04X} to CHR ROM address 0x{addr:04X}")
    }

    fn dump_pattern_tables(&self) -> Vec<[u8; PATTERN_TABLE_SIZE]> {
        let mut pattern_tables = vec![[0u8; PATTERN_TABLE_SIZE], [0u8; PATTERN_TABLE_SIZE]];

        for (i, table) in pattern_tables.iter_mut().enumerate() {
            let start_addr = if i == 0 {
                PATTERN_TABLE_0_START_ADDR
            } else {
                PATTERN_TABLE_1_START_ADDR
            };

            for (offset, byte) in table.iter_mut().enumerate() {
                *byte = self.chr_read(start_addr + offset as u16)
            }
        }

        pattern_tables
    }

    fn get_nametable_arrangement(&self) -> NametableMirroring {
        self.nametable_arrangement
    }
}
