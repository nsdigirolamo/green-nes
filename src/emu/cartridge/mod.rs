pub mod ines;

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
