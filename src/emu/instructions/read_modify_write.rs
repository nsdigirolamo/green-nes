use crate::emu::{
    cycles::{
        FETCH_HIGH_BASE_ADDRESS_BYTE, FETCH_HIGH_EFFECTIVE_ADDRESS_BYTE,
        FETCH_LOW_BASE_ADDRESS_BYTE, FETCH_LOW_EFFECTIVE_ADDRESS_BYTE,
    },
    half_cycles::{
        get_absolute_x_indexed_address, get_base_zero_page_address, get_effective_absolute_address,
        get_effective_zero_page_address, get_effective_zero_page_x_indexed_address, read_data,
        write_data,
    },
    instructions::Instruction,
    state::{Cycle, HalfCycle},
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
                FETCH_HIGH_EFFECTIVE_ADDRESS_BYTE,
                [get_effective_zero_page_address, read_data],
                [get_effective_zero_page_address, write_data],
                [get_effective_zero_page_address, operation],
            ],
            ReadModifyWrite::Absolute => vec![
                FETCH_HIGH_EFFECTIVE_ADDRESS_BYTE,
                FETCH_LOW_EFFECTIVE_ADDRESS_BYTE,
                [get_effective_absolute_address, read_data],
                [get_effective_absolute_address, write_data],
                [get_effective_absolute_address, operation],
            ],
            ReadModifyWrite::ZeroPageX => vec![
                FETCH_LOW_EFFECTIVE_ADDRESS_BYTE,
                [get_base_zero_page_address, read_data],
                [get_effective_zero_page_x_indexed_address, read_data],
                [get_effective_zero_page_x_indexed_address, write_data],
                [get_effective_zero_page_x_indexed_address, operation],
            ],
            ReadModifyWrite::AbsoluteX => vec![
                FETCH_LOW_BASE_ADDRESS_BYTE,
                FETCH_HIGH_BASE_ADDRESS_BYTE,
                [get_absolute_x_indexed_address, read_data],
                [get_absolute_x_indexed_address, read_data],
                [get_absolute_x_indexed_address, write_data],
                [get_absolute_x_indexed_address, operation],
            ],
        }
    }
}
