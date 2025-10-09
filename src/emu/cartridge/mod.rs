use crate::emu::cartridge::mappers::Mapper;

pub mod ines;
pub mod mappers;

pub struct Cartridge {
    pub mapper: Box<dyn Mapper>,
}
