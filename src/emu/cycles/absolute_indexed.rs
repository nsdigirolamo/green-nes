use crate::emu::{
    cycles::FETCH_LOW_ADDR_BYTE,
    operations::{
        fix_high_address_byte, get_effective_absolute_address, get_pc_address,
        read_high_address_byte_x_indexed, write_effective_data,
    },
    state::{Cycle, HalfCycle},
};

pub enum AbsoluteXIndexedAddressing {
    Read,
    ReadModifyWrite,
    Write,
}

impl AbsoluteXIndexedAddressing {
    pub fn get_cycles(&self, operation: HalfCycle) -> Vec<Cycle> {
        match self {
            AbsoluteXIndexedAddressing::Read => vec![
                FETCH_LOW_ADDR_BYTE,
                [get_pc_address, read_high_address_byte_x_indexed],
                [get_effective_absolute_address, operation],
            ],
            AbsoluteXIndexedAddressing::ReadModifyWrite => vec![
                FETCH_LOW_ADDR_BYTE,
                [get_pc_address, read_high_address_byte_x_indexed],
                [get_effective_absolute_address, fix_high_address_byte],
                [get_effective_absolute_address, operation],
                [get_effective_absolute_address, write_effective_data],
            ],
            AbsoluteXIndexedAddressing::Write => vec![
                FETCH_LOW_ADDR_BYTE,
                [get_pc_address, read_high_address_byte_x_indexed],
                [get_effective_absolute_address, fix_high_address_byte],
                [get_effective_absolute_address, operation],
            ],
        }
    }
}
