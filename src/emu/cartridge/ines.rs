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
const FLAGS_6_TRAINER_MASK: u8 = 0b_0000_0100;

const FLAGS_7_INDEX: usize = 7;
const FLAGS_7_INES2_FORMAT_MASK: u8 = 0b_0000_1100;
const FLAGS_7_MAPPER_UPPER_NIBBLE_MASK: u8 = 0b_1111_0000;

pub fn read_cartridge(path_to_ines_file: &str) -> Result<Cartridge, Error> {
    let data = read_data(path_to_ines_file)?;

    let prg_rom_size = (data[PRG_ROM_SIZE_INDEX] as usize) * INES_PRG_ROM_SIZE_UNITS;
    let trainer_exists = (data[FLAGS_6_INDEX] & FLAGS_6_TRAINER_MASK) != 0;
    let prg_rom_start = INES_HEADER_SIZE + if trainer_exists { TRAINER_SIZE } else { 0 };
    let prg_rom_end = prg_rom_start + prg_rom_size;
    let prg_rom = data[prg_rom_start..prg_rom_end].to_vec();

    let chr_rom_size = (data[CHR_ROM_SIZE_INDEX] as usize) * INES_CHR_ROM_SIZE_UNITS;
    let chr_rom_start = prg_rom_end;
    let chr_rom_end = chr_rom_start + chr_rom_size;
    let chr_rom = data[chr_rom_start..chr_rom_end].to_vec();

    let mapper_index =
        (data[FLAGS_7_INDEX] & FLAGS_7_MAPPER_UPPER_NIBBLE_MASK) | (data[FLAGS_6_INDEX] >> 4);

    let mapper = create_mapper(mapper_index, prg_rom, chr_rom)?;

    Ok(Cartridge {
        mapper: Box::new(mapper),
    })
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

pub fn create_mapper(
    mapper_index: u8,
    prg_rom: Vec<u8>,
    chr_rom: Vec<u8>,
) -> Result<impl Mapper, Error> {
    match mapper_index {
        0 => NROM::new(prg_rom, chr_rom),
        _ => Err(CartridgeError::NotSupported {
            message: format!("mapper {mapper_index} is not supported"),
        }
        .into()),
    }
}
