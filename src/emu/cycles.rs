use crate::emu::{
    half_cycles::{
        get_pc_address, read_high_base_address_byte, read_high_effective_address_byte,
        read_low_base_address_byte, read_low_effective_address_byte, read_opcode,
    },
    state::Cycle,
};

pub const FETCH_INSTRUCTION: Cycle = [get_pc_address, read_opcode];
pub const FETCH_HIGH_EFFECTIVE_ADDRESS_BYTE: Cycle =
    [get_pc_address, read_high_effective_address_byte];
pub const FETCH_LOW_EFFECTIVE_ADDRESS_BYTE: Cycle =
    [get_pc_address, read_low_effective_address_byte];
pub const FETCH_HIGH_BASE_ADDRESS_BYTE: Cycle = [get_pc_address, read_high_base_address_byte];
pub const FETCH_LOW_BASE_ADDRESS_BYTE: Cycle = [get_pc_address, read_low_base_address_byte];
