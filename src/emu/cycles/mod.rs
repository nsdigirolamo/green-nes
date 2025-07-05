use crate::emu::{
    operations::{get_pc_address, read_high_address_byte, read_instruction, read_low_address_byte},
    state::Cycle,
};

pub mod absolute;
pub mod zero_page;
pub mod zero_page_indexed;

pub const FETCH_INSTRUCTION: Cycle = [get_pc_address, read_instruction];
pub const FETCH_LOW_ADDR_BYTE: Cycle = [get_pc_address, read_low_address_byte];
pub const FETCH_HIGH_ADDR_BYTE: Cycle = [get_pc_address, read_high_address_byte];
