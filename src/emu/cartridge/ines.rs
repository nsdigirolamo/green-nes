use std::{cell::RefCell, rc::Rc};

use crate::emu::{
    cartridge::{
        Cartridge,
        mappers::{Mapper, nrom::NROM},
    },
    error::{CartridgeError, Error, FileError},
};

const INES_HEADER_SIZE: usize = 16;
const TRAINER_SIZE: usize = 512;

const INES_TAG_INDEX: usize = 0;
const INES_TAG: [u8; 4] = [0x4E, 0x45, 0x53, 0x1A]; // ASCII "NES" followed by MS-DOS EoF

const PRG_ROM_SIZE_INDEX: usize = 4;
const INES_PRG_ROM_SIZE_UNITS: usize = 16384;

const CHR_ROM_SIZE_INDEX: usize = 5;
const INES_CHR_ROM_SIZE_UNITS: usize = 8192;

const FLAGS_6_INDEX: usize = 6;
const FLAGS_6_NAMETABLE_ARRANGEMENT_MASK: u8 = 0b_0000_0001;
const FLAGS_6_TRAINER_MASK: u8 = 0b_0000_0100;
const FLAGS_6_ALTERNATIVE_NAMETABLE_ARRANGEMENT_MASK: u8 = 0b_0000_1000;
const FLAGS_6_MAPPER_LOWER_NIBBLE_MASK: u8 = 0b_1111_0000;

const FLAGS_7_INDEX: usize = 7;
const FLAGS_7_INES2_FORMAT_MASK: u8 = 0b_0000_1100;
const FLAGS_7_MAPPER_UPPER_NIBBLE_MASK: u8 = 0b_1111_0000;

pub struct INes {
    pub prg_data: Vec<u8>,
    pub chr_data: Vec<u8>,
    pub mapper_index: u8,
    pub nametable_arrangement: bool,
    pub alternative_nametable_arrangement: bool,
}

pub fn read_cartridge(path_to_ines_file: &str) -> Result<Cartridge, Error> {
    let data = read_data(path_to_ines_file)?;

    let prg_rom_size = (data[PRG_ROM_SIZE_INDEX] as usize) * INES_PRG_ROM_SIZE_UNITS;
    let trainer_exists = (data[FLAGS_6_INDEX] & FLAGS_6_TRAINER_MASK) != 0;
    let prg_rom_start = INES_HEADER_SIZE + if trainer_exists { TRAINER_SIZE } else { 0 };
    let prg_rom_end = prg_rom_start + prg_rom_size;
    let prg_data = data[prg_rom_start..prg_rom_end].to_vec();

    let chr_rom_size = (data[CHR_ROM_SIZE_INDEX] as usize) * INES_CHR_ROM_SIZE_UNITS;
    let chr_rom_start = prg_rom_end;
    let chr_rom_end = chr_rom_start + chr_rom_size;
    let chr_data = data[chr_rom_start..chr_rom_end].to_vec();

    let mapper_index = get_mapper_index(&data);
    let nametable_arrangement = get_nametable_arrangement(&data);
    let alternative_nametable_arrangement = get_alternative_nametable_arrangement(&data);

    let ines = INes {
        prg_data,
        chr_data,
        mapper_index,
        nametable_arrangement,
        alternative_nametable_arrangement,
    };

    let mapper = create_mapper(ines)?;

    Ok(Cartridge {
        mapper: Rc::new(RefCell::new(mapper)),
    })
}

fn get_mapper_index(data: &[u8]) -> u8 {
    let upper_nibble = data[FLAGS_7_INDEX] & FLAGS_7_MAPPER_UPPER_NIBBLE_MASK;
    let lower_nibble = data[FLAGS_6_INDEX] & FLAGS_6_MAPPER_LOWER_NIBBLE_MASK;

    upper_nibble | lower_nibble
}

fn get_nametable_arrangement(data: &[u8]) -> bool {
    data[FLAGS_6_INDEX] & FLAGS_6_NAMETABLE_ARRANGEMENT_MASK != 0
}

fn get_alternative_nametable_arrangement(data: &[u8]) -> bool {
    data[FLAGS_6_INDEX] & FLAGS_6_ALTERNATIVE_NAMETABLE_ARRANGEMENT_MASK != 0
}

fn read_data(path_to_ines_file: &str) -> Result<Vec<u8>, Error> {
    let data = std::fs::read(std::path::Path::new(path_to_ines_file)).map_err(|e| {
        Error::from(FileError::FileOpenFailed {
            message: e.to_string(),
        })
    })?;

    if data.len() < INES_HEADER_SIZE {
        return Err(CartridgeError::MissingHeader.into());
    }

    if data[INES_TAG_INDEX..(INES_TAG_INDEX + INES_TAG.len())] != INES_TAG {
        return Err(CartridgeError::InvalidHeader {
            message: "file is not formatted as iNES".to_string(),
        }
        .into());
    }

    let is_ines_v2 = (data[FLAGS_7_INDEX] & FLAGS_7_INES2_FORMAT_MASK) == FLAGS_7_INES2_FORMAT_MASK;
    if is_ines_v2 {
        return Err(CartridgeError::NotSupported {
            message: "iNES format version 2 not supported".to_string(),
        }
        .into());
    }

    Ok(data)
}

pub fn create_mapper(ines: INes) -> Result<impl Mapper, Error> {
    match ines.mapper_index {
        0 => NROM::new(ines),
        i => Err(CartridgeError::NotSupported {
            message: format!("mapper {i} is not supported"),
        }
        .into()),
    }
}
