use crate::emu::{
    cycles::{
        FETCH_HIGH_EFFECTIVE_ADDRESS_BYTE, FETCH_LOW_BASE_ADDRESS_BYTE,
        FETCH_LOW_EFFECTIVE_ADDRESS_BYTE,
    },
    half_cycles::{
        get_base_zero_page_address, get_effective_address, get_effective_zero_page_address,
        get_effective_zero_page_x_indexed_address, get_effective_zero_page_y_indexed_address,
        get_indirect_x_indexed_high_address_byte, get_indirect_x_indexed_low_address_byte,
        get_indirect_zero_page_high_address_byte, get_indirect_zero_page_low_address_byte, get_pc,
        get_x_indexed_base_address_with_carry, get_y_indexed_base_address_with_carry, read_data,
        read_high_base_address_byte, read_high_effective_address_byte, read_low_base_address_byte,
        read_low_effective_address_byte, read_low_indirect_address_byte,
    },
    instructions::Instruction,
    state::{Cycle, HalfCycle},
};

pub enum Read {
    Immediate,
    ZeroPage,
    Absolute,
    IndirectX,
    IndirectY,
    AbsoluteX,
    AbsoluteY,
    ZeroPageX,
    ZeroPageY,
}

impl Instruction for Read {
    fn get_cycles(&self, operation: HalfCycle) -> Vec<Cycle> {
        match self {
            Read::Immediate => vec![[get_pc, operation]],
            Read::ZeroPage => vec![
                FETCH_LOW_EFFECTIVE_ADDRESS_BYTE,
                [get_effective_zero_page_address, operation],
            ],
            Read::Absolute => vec![
                FETCH_LOW_EFFECTIVE_ADDRESS_BYTE,
                FETCH_HIGH_EFFECTIVE_ADDRESS_BYTE,
                [get_effective_address, operation],
            ],
            Read::IndirectX => vec![
                FETCH_LOW_BASE_ADDRESS_BYTE,
                [get_base_zero_page_address, read_data],
                [
                    get_indirect_x_indexed_low_address_byte,
                    read_low_effective_address_byte,
                ],
                [
                    get_indirect_x_indexed_high_address_byte,
                    read_high_effective_address_byte,
                ],
                [get_effective_address, operation],
            ],
            Read::IndirectY => vec![
                [get_pc, read_low_indirect_address_byte],
                [
                    get_indirect_zero_page_low_address_byte,
                    read_low_base_address_byte,
                ],
                [
                    get_indirect_zero_page_high_address_byte,
                    read_high_base_address_byte,
                ],
                [get_y_indexed_base_address_with_carry, operation],
            ],
            Read::AbsoluteX => vec![
                [get_pc, read_low_base_address_byte],
                [get_pc, read_high_base_address_byte],
                [get_x_indexed_base_address_with_carry, operation],
            ],
            Read::AbsoluteY => vec![
                [get_pc, read_low_base_address_byte],
                [get_pc, read_high_base_address_byte],
                [get_y_indexed_base_address_with_carry, operation],
            ],
            Read::ZeroPageX => vec![
                FETCH_LOW_BASE_ADDRESS_BYTE,
                [get_base_zero_page_address, read_data],
                [get_effective_zero_page_x_indexed_address, operation],
            ],
            Read::ZeroPageY => vec![
                FETCH_LOW_BASE_ADDRESS_BYTE,
                [get_base_zero_page_address, read_data],
                [get_effective_zero_page_y_indexed_address, operation],
            ],
        }
    }
}
