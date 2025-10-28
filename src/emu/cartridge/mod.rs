use crate::emu::cartridge::mappers::Mapper;
use std::{cell::RefCell, rc::Rc};

pub mod ines;
pub mod mappers;

#[derive(Clone)]
pub struct Cartridge {
    // Use Rc<RefCell<T>> for interior mutability and derived Clone trait.
    // https://stackoverflow.com/a/52994358
    pub mapper: Rc<RefCell<dyn Mapper>>,
}
