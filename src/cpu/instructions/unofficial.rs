use crate::cpu::{
    cycles::{
        FETCH_HIGH_BASE_ADDRESS_BYTE, FETCH_HIGH_EFFECTIVE_ADDRESS_BYTE,
        FETCH_LOW_BASE_ADDRESS_BYTE, FETCH_LOW_EFFECTIVE_ADDRESS_BYTE,
    },
    half_cycles::{
        get_base_zero_page_address, get_effective_address, get_effective_zero_page_address,
        get_effective_zero_page_x_indexed_address, get_indirect_x_indexed_high_address_byte,
        get_indirect_x_indexed_low_address_byte, get_indirect_zero_page_high_address_byte,
        get_indirect_zero_page_low_address_byte, get_pc, get_x_indexed_base_address_with_carry,
        get_y_indexed_base_address_with_carry, read_data, read_high_base_address_byte,
        read_high_effective_address_byte, read_low_base_address_byte,
        read_low_effective_address_byte, read_low_indirect_address_byte, write_data,
    },
    instructions::Instruction,
    operations::other::nop,
    state::{Cycle, HalfCycle},
};

pub enum Unofficial {
    ZeroPage,
    ZeroPageX,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    IndirectX,
    IndirectY,
    Halt,
}

/*
The functionality here is closest to the ReadModifyWrite (RMW) instructions.
This is because the unofficial opcodes here mostly use RMW operations, with some
other addressing modes mixed in.
Reference: https://www.nesdev.org/wiki/CPU_unofficial_opcodes#Arrangement
 */
impl Instruction for Unofficial {
    fn get_cycles(&self, operation: HalfCycle) -> Vec<Cycle> {
        match self {
            Unofficial::ZeroPage => vec![
                FETCH_LOW_EFFECTIVE_ADDRESS_BYTE,
                [get_effective_zero_page_address, read_data],
                [get_effective_zero_page_address, write_data],
                [get_effective_zero_page_address, operation],
            ],
            Unofficial::ZeroPageX => vec![
                FETCH_LOW_BASE_ADDRESS_BYTE,
                [get_base_zero_page_address, read_data],
                [get_effective_zero_page_x_indexed_address, read_data],
                [get_effective_zero_page_x_indexed_address, write_data],
                [get_effective_zero_page_x_indexed_address, operation],
            ],
            Unofficial::Absolute => vec![
                FETCH_LOW_EFFECTIVE_ADDRESS_BYTE,
                FETCH_HIGH_EFFECTIVE_ADDRESS_BYTE,
                [get_effective_address, read_data],
                [get_effective_address, write_data],
                [get_effective_address, operation],
            ],
            Unofficial::AbsoluteX => vec![
                FETCH_LOW_BASE_ADDRESS_BYTE,
                FETCH_HIGH_BASE_ADDRESS_BYTE,
                [get_x_indexed_base_address_with_carry, read_data],
                [get_effective_address, read_data],
                [get_effective_address, write_data],
                [get_effective_address, operation],
            ],
            Unofficial::AbsoluteY => vec![
                FETCH_LOW_BASE_ADDRESS_BYTE,
                FETCH_HIGH_BASE_ADDRESS_BYTE,
                [get_y_indexed_base_address_with_carry, read_data],
                [get_effective_address, read_data],
                [get_effective_address, write_data],
                [get_effective_address, operation],
            ],
            Unofficial::IndirectX => vec![
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
                [get_effective_address, read_data],
                [get_effective_address, write_data],
                [get_effective_address, operation],
            ],
            Unofficial::IndirectY => vec![
                [get_pc, read_low_indirect_address_byte],
                [
                    get_indirect_zero_page_low_address_byte,
                    read_low_base_address_byte,
                ],
                [
                    get_indirect_zero_page_high_address_byte,
                    read_high_base_address_byte,
                ],
                [get_y_indexed_base_address_with_carry, read_data],
                [get_effective_address, read_data],
                [get_effective_address, write_data],
                [get_effective_address, operation],
            ],
            Unofficial::Halt => vec![[nop, operation]],
        }
    }
}
