use crate::emu::error::{CartridgeError, Error, FileError};

const INES_TAG: [u8; 4] = [0x4E, 0x45, 0x53, 0x1A]; // ASCII "NES" followed by MS-DOS EoF
const INES_HEADER_LENGTH: usize = 16;
const INES_PRG_ROM_SIZE_UNITS: usize = 16384;
const INES_CHR_ROM_SIZE_UNITS: usize = 8192;

pub enum NametableMirroring {
    Horizontal, // Vertically Arranged
    Vertical,   // Horizontally Arranged
    FourScreen,
}

pub struct Cartridge {
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
    pub mapper: u8,
    pub nametable_mirroring: NametableMirroring,
}

pub fn read_cartridge(path_to_ines_file: &str) -> Result<Cartridge, Error> {
    let data = std::fs::read(std::path::Path::new(path_to_ines_file)).map_err(|e| {
        Error::from(FileError::FileOpenFailed {
            message: e.to_string(),
        })
    })?;

    if data.len() < INES_HEADER_LENGTH {
        return Err(CartridgeError::MissingHeader.into());
    }

    if data[0..4] != INES_TAG {
        return Err(Error::from(CartridgeError::InvalidHeader {
            message: "file is not formatted as iNES".to_string(),
        }));
    }

    let is_ines_v2 = (data[7] & 0b_0000_1100) == 0b_0000_1100;
    if is_ines_v2 {
        return Err(Error::from(CartridgeError::NotSupported {
            message: "iNES format version 2 not supported".to_string(),
        }));
    }

    let prg_rom_size = (data[4] as usize) * INES_PRG_ROM_SIZE_UNITS;
    let chr_rom_size = (data[5] as usize) * INES_CHR_ROM_SIZE_UNITS;

    let nametable_arrangement = (data[6] & 0b_0000_0001) != 0;
    let alternative_nametable_layout = (data[6] & 0b_0000_1000) != 0;
    let nametable_mirroring = match (nametable_arrangement, alternative_nametable_layout) {
        (false, false) => NametableMirroring::Horizontal,
        (true, false) => NametableMirroring::Vertical,
        (_, true) => NametableMirroring::FourScreen,
    };

    let trainer_exists = (data[6] & 0b_0000_0100) != 0;

    let prg_rom_start = INES_HEADER_LENGTH + if trainer_exists { 512 } else { 0 };
    let prg_rom_end = prg_rom_start + prg_rom_size;

    let chr_rom_start = prg_rom_end;
    let chr_rom_end = chr_rom_start + chr_rom_size;

    let mapper = (data[7] & 0b_1111_0000) | (data[6] >> 4);

    Ok(Cartridge {
        prg_rom: data[prg_rom_start..prg_rom_end].to_vec(),
        chr_rom: data[chr_rom_start..chr_rom_end].to_vec(),
        mapper,
        nametable_mirroring,
    })
}
