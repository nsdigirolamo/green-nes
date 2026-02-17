use crate::emu::cpu::{cycles::*, half_cycles::*, instructions::Instruction};

pub struct ZeroPage {
    pub op: HalfCycle,
}

impl Instruction<4> for ZeroPage {
    fn get_cycles(&self) -> [Cycle; 4] {
        [
            FETCH_LOW_EFFECTIVE_ADDRESS_BYTE,
            READ_FROM_EFFECTIVE_ZERO_PAGE_ADDRESS,
            WRITE_TO_EFFECTIVE_ZERO_PAGE_ADDRESS,
            [get_effective_zero_page_addr, self.op],
        ]
    }
}

pub struct Absolute {
    pub op: HalfCycle,
}

impl Instruction<5> for Absolute {
    fn get_cycles(&self) -> [Cycle; 5] {
        [
            FETCH_LOW_EFFECTIVE_ADDRESS_BYTE,
            FETCH_HIGH_EFFECTIVE_ADDRESS_BYTE,
            READ_FROM_EFFECTIVE_ADDRESS,
            WRITE_TO_EFFECTIVE_ADDRESS,
            [get_effective_addr, self.op],
        ]
    }
}

pub struct ZeroPageX {
    pub op: HalfCycle,
}

impl Instruction<5> for ZeroPageX {
    fn get_cycles(&self) -> [Cycle; 5] {
        [
            FETCH_LOW_BASE_ADDRESS_BYTE,
            READ_FROM_BASE_ZERO_PAGE_ADDRESS,
            [get_base_zero_page_x_indexed_addr, read_data],
            [get_base_zero_page_x_indexed_addr, write_data],
            [get_base_zero_page_x_indexed_addr, self.op],
        ]
    }
}

pub struct AbsoluteX {
    pub op: HalfCycle,
}

impl Instruction<6> for AbsoluteX {
    fn get_cycles(&self) -> [Cycle; 6] {
        [
            FETCH_LOW_BASE_ADDRESS_BYTE,
            FETCH_HIGH_BASE_ADDRESS_BYTE,
            [get_base_addr_x_indexed_with_carry, read_data],
            READ_FROM_EFFECTIVE_ADDRESS,
            WRITE_TO_EFFECTIVE_ADDRESS,
            [get_effective_addr, self.op],
        ]
    }
}
