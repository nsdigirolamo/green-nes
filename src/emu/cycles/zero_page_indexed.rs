use crate::emu::{
    cycles::FETCH_LOW_ADDR_BYTE,
    operations::{
        add_x_index_to_address, add_y_index_to_address, get_effective_absolute_address,
        get_effective_zero_page_address, read_effective_address, write_effective_data,
    },
    state::{Cycle, HalfCycle},
};

pub enum ZeroPageXIndexedAddressing {
    Read,
    ReadModifyWrite,
    Write,
}

impl ZeroPageXIndexedAddressing {
    pub fn get_cycles(&self, operation: HalfCycle) -> Vec<Cycle> {
        match self {
            ZeroPageXIndexedAddressing::Read => vec![
                FETCH_LOW_ADDR_BYTE,
                [get_effective_zero_page_address, add_x_index_to_address],
                [get_effective_absolute_address, operation],
            ],
            ZeroPageXIndexedAddressing::ReadModifyWrite => vec![
                FETCH_LOW_ADDR_BYTE,
                [get_effective_zero_page_address, add_x_index_to_address],
                [get_effective_absolute_address, read_effective_address],
                [get_effective_absolute_address, operation],
                [get_effective_absolute_address, write_effective_data],
            ],
            ZeroPageXIndexedAddressing::Write => vec![
                FETCH_LOW_ADDR_BYTE,
                [get_effective_zero_page_address, add_x_index_to_address],
                [get_effective_absolute_address, operation],
            ],
        }
    }
}

pub enum ZeroPageYIndexedAddressing {
    Read,
    ReadModifyWrite,
    Write,
}

impl ZeroPageYIndexedAddressing {
    pub fn get_cycles(&self, operation: HalfCycle) -> Vec<Cycle> {
        match self {
            ZeroPageYIndexedAddressing::Read => vec![
                FETCH_LOW_ADDR_BYTE,
                [get_effective_zero_page_address, add_y_index_to_address],
                [get_effective_absolute_address, operation],
            ],
            ZeroPageYIndexedAddressing::ReadModifyWrite => vec![
                FETCH_LOW_ADDR_BYTE,
                [get_effective_zero_page_address, add_y_index_to_address],
                [get_effective_absolute_address, read_effective_address],
                [get_effective_absolute_address, operation],
                [get_effective_absolute_address, write_effective_data],
            ],
            ZeroPageYIndexedAddressing::Write => vec![
                FETCH_LOW_ADDR_BYTE,
                [get_effective_zero_page_address, add_y_index_to_address],
                [get_effective_absolute_address, operation],
            ],
        }
    }
}
