use crate::emu::{
    cartridge::{Cartridge, NametableMirroring},
    error::{CartridgeError, Error, FileError},
};

/*
iNES File Format (https://www.nesdev.org/wiki/INES#iNES_file_format)
Bytes   Description
0-3 	Constant $4E $45 $53 $1A (ASCII "NES" followed by MS-DOS end-of-file)
4 	    Size of PRG ROM in 16 KB units
5 	    Size of CHR ROM in 8 KB units (value 0 means the board uses CHR RAM)
6 	    Flags 6 – Mapper, mirroring, battery, trainer
7 	    Flags 7 – Mapper, VS/Playchoice, NES 2.0
8 	    Flags 8 – PRG-RAM size (rarely used extension)
9 	    Flags 9 – TV system (rarely used extension)
10 	    Flags 10 – TV system, PRG-RAM presence (unofficial, rarely used extension)
11-15 	Unused padding (should be filled with zero, but some rippers put their name across bytes 7-15)
*/

const INES_HEADER_SIZE: usize = 16;
const TRAINER_SIZE: usize = 512;

const INES_TAG_INDEX: usize = 0;
const INES_TAG: [u8; 4] = [0x4E, 0x45, 0x53, 0x1A]; // ASCII "NES" followed by MS-DOS EoF

const PRG_ROM_SIZE_INDEX: usize = 4;
const INES_PRG_ROM_SIZE_UNITS: usize = 16384;

const CHR_ROM_SIZE_INDEX: usize = 5;
const INES_CHR_ROM_SIZE_UNITS: usize = 8192;

const FLAGS_6_INDEX: usize = 6;
const FLAGS_6_NAMETABLE_MASK: u8 = 0b_0000_0001;
const FLAGS_6_TRAINER_MASK: u8 = 0b_0000_0100;
const FLAGS_6_ALT_NAMETABLE_MASK: u8 = 0b_0000_1000;

/*
Flags 6 (https://www.nesdev.org/wiki/INES#Flags_6)
76543210
||||||||
|||||||+- Nametable arrangement: 0: vertical arrangement ("horizontal mirrored") (CIRAM A10 = PPU A11)
|||||||                          1: horizontal arrangement ("vertically mirrored") (CIRAM A10 = PPU A10)
||||||+-- 1: Cartridge contains battery-backed PRG RAM ($6000-7FFF) or other persistent memory
|||||+--- 1: 512-byte trainer at $7000-$71FF (stored before PRG data)
||||+---- 1: Alternative nametable layout
++++----- Lower nybble of mapper number
*/

const FLAGS_7_INDEX: usize = 7;
const FLAGS_7_INES2_FORMAT_MASK: u8 = 0b_0000_1100;
const FLAGS_7_MAPPER_UPPER_NIBBLE_MASK: u8 = 0b_1111_0000;

/*
Flags 7 (https://www.nesdev.org/wiki/INES#Flags_7)
76543210
||||||||
|||||||+- VS Unisystem
||||||+-- PlayChoice-10 (8 KB of Hint Screen data stored after CHR data)
||||++--- If equal to 2, flags 8-15 are in NES 2.0 format
++++----- Upper nybble of mapper number
*/

pub fn read_cartridge(path_to_ines_file: &str) -> Result<Cartridge, Error> {
    let data = std::fs::read(std::path::Path::new(path_to_ines_file)).map_err(|e| {
        Error::from(FileError::FileOpenFailed {
            message: e.to_string(),
        })
    })?;

    if data.len() < INES_HEADER_SIZE {
        return Err(CartridgeError::MissingHeader.into());
    }

    if data[INES_TAG_INDEX..(INES_TAG_INDEX + INES_TAG.len())] != INES_TAG {
        return Err(Error::from(CartridgeError::InvalidHeader {
            message: "file is not formatted as iNES".to_string(),
        }));
    }

    let is_ines_v2 = (data[FLAGS_7_INDEX] & FLAGS_7_INES2_FORMAT_MASK) == FLAGS_7_INES2_FORMAT_MASK;
    if is_ines_v2 {
        return Err(Error::from(CartridgeError::NotSupported {
            message: "iNES format version 2 not supported".to_string(),
        }));
    }

    let prg_rom_size = (data[PRG_ROM_SIZE_INDEX] as usize) * INES_PRG_ROM_SIZE_UNITS;
    let chr_rom_size = (data[CHR_ROM_SIZE_INDEX] as usize) * INES_CHR_ROM_SIZE_UNITS;

    let nametable_arrangement = (data[FLAGS_6_INDEX] & FLAGS_6_NAMETABLE_MASK) != 0;
    let alternative_nametable_layout = (data[FLAGS_6_INDEX] & FLAGS_6_ALT_NAMETABLE_MASK) != 0;
    let nametable_mirroring = match (nametable_arrangement, alternative_nametable_layout) {
        (false, false) => NametableMirroring::Horizontal,
        (true, false) => NametableMirroring::Vertical,
        (_, true) => NametableMirroring::FourScreen,
    };

    let trainer_exists = (data[FLAGS_6_INDEX] & FLAGS_6_TRAINER_MASK) != 0;

    let prg_rom_start = INES_HEADER_SIZE + if trainer_exists { TRAINER_SIZE } else { 0 };
    let prg_rom_end = prg_rom_start + prg_rom_size;

    let chr_rom_start = prg_rom_end;
    let chr_rom_end = chr_rom_start + chr_rom_size;

    let mapper =
        (data[FLAGS_7_INDEX] & FLAGS_7_MAPPER_UPPER_NIBBLE_MASK) | (data[FLAGS_6_INDEX] >> 4);

    Ok(Cartridge {
        prg_rom: data[prg_rom_start..prg_rom_end].to_vec(),
        chr_rom: data[chr_rom_start..chr_rom_end].to_vec(),
        mapper,
        nametable_mirroring,
    })
}
