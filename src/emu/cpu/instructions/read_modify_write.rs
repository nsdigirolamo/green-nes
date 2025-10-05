use crate::emu::cpu::{
    cycles::{
        Cycle, FETCH_HIGH_BASE_ADDRESS_BYTE, FETCH_HIGH_EFFECTIVE_ADDRESS_BYTE,
        FETCH_LOW_BASE_ADDRESS_BYTE, FETCH_LOW_EFFECTIVE_ADDRESS_BYTE,
    },
    half_cycles::{
        HalfCycle, get_base_zero_page_address, get_effective_address,
        get_effective_zero_page_address, get_effective_zero_page_x_indexed_address,
        get_x_indexed_base_address_with_carry, read_data, write_data,
    },
    instructions::Instruction,
};

pub enum ReadModifyWrite {
    ZeroPage,
    Absolute,
    ZeroPageX,
    AbsoluteX,
}

impl Instruction for ReadModifyWrite {
    fn get_cycles(&self, operation: HalfCycle) -> Vec<Cycle> {
        match self {
            ReadModifyWrite::ZeroPage => vec![
                FETCH_LOW_EFFECTIVE_ADDRESS_BYTE,
                [get_effective_zero_page_address, read_data],
                [get_effective_zero_page_address, write_data],
                [get_effective_zero_page_address, operation],
            ],
            ReadModifyWrite::Absolute => vec![
                FETCH_LOW_EFFECTIVE_ADDRESS_BYTE,
                FETCH_HIGH_EFFECTIVE_ADDRESS_BYTE,
                [get_effective_address, read_data],
                [get_effective_address, write_data],
                [get_effective_address, operation],
            ],
            ReadModifyWrite::ZeroPageX => vec![
                FETCH_LOW_BASE_ADDRESS_BYTE,
                [get_base_zero_page_address, read_data],
                [get_effective_zero_page_x_indexed_address, read_data],
                [get_effective_zero_page_x_indexed_address, write_data],
                [get_effective_zero_page_x_indexed_address, operation],
            ],
            ReadModifyWrite::AbsoluteX => vec![
                FETCH_LOW_BASE_ADDRESS_BYTE,
                FETCH_HIGH_BASE_ADDRESS_BYTE,
                [get_x_indexed_base_address_with_carry, read_data],
                [get_effective_address, read_data],
                [get_effective_address, write_data],
                [get_effective_address, operation],
            ],
        }
    }
}
