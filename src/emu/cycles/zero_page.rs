use crate::emu::{
    cycles::FETCH_LOW_ADDR_BYTE,
    operations::{get_effective_zero_page_address, read_effective_address, write_effective_data},
    state::{Cycle, HalfCycle},
};

pub enum ZeroPageAddressing {
    Read,
    ReadModifyWrite,
    Write,
}

impl ZeroPageAddressing {
    pub fn get_cycles(&self, operation: HalfCycle) -> Vec<Cycle> {
        match self {
            ZeroPageAddressing::Read => vec![
                FETCH_LOW_ADDR_BYTE,
                [get_effective_zero_page_address, operation],
            ],
            ZeroPageAddressing::ReadModifyWrite => vec![
                FETCH_LOW_ADDR_BYTE,
                [get_effective_zero_page_address, read_effective_address],
                [get_effective_zero_page_address, operation],
                [get_effective_zero_page_address, write_effective_data],
            ],
            ZeroPageAddressing::Write => vec![
                FETCH_LOW_ADDR_BYTE,
                [get_effective_zero_page_address, operation],
            ],
        }
    }
}
