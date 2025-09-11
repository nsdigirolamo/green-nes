use crate::ppu::state::registers::Registers;

pub mod registers;

#[derive(Default)]
pub struct State {
    pub registers: Registers,
}
