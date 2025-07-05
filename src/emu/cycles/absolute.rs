use crate::emu::{
    cycles::{FETCH_HIGH_ADDR_BYTE, FETCH_LOW_ADDR_BYTE},
    operations::{
        get_effective_absolute_address, get_pc_address, read_effective_address,
        write_effective_data,
    },
    state::{Cycle, HalfCycle},
};

pub enum AbsoluteAddressing {
    Jump,
    Read,
    ReadModifyWrite,
    Write,
}

impl AbsoluteAddressing {
    pub fn get_cycles(&self, operation: HalfCycle) -> Vec<Cycle> {
        match self {
            AbsoluteAddressing::Jump => {
                vec![FETCH_LOW_ADDR_BYTE, [get_pc_address, operation]]
            }
            AbsoluteAddressing::Read => vec![
                FETCH_LOW_ADDR_BYTE,
                FETCH_HIGH_ADDR_BYTE,
                [get_effective_absolute_address, operation],
            ],
            AbsoluteAddressing::ReadModifyWrite => vec![
                FETCH_LOW_ADDR_BYTE,
                FETCH_HIGH_ADDR_BYTE,
                [get_effective_absolute_address, read_effective_address],
                [get_effective_absolute_address, operation],
                [get_effective_absolute_address, write_effective_data],
            ],
            AbsoluteAddressing::Write => vec![
                FETCH_LOW_ADDR_BYTE,
                FETCH_HIGH_ADDR_BYTE,
                [get_effective_absolute_address, operation],
            ],
        }
    }
}
