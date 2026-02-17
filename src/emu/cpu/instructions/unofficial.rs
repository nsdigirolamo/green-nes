use crate::emu::cpu::{
    cycles::*,
    half_cycles::{
        operations::other::{jam, nop},
        *,
    },
    instructions::Instruction,
};

pub struct ZeroPage {
    pub op: HalfCycle,
}

impl Instruction<4> for ZeroPage {
    fn get_cycles(&self) -> [Cycle; 4] {
        [
            GET_EFFECTIVE_ADDR_LOW_BYTE,
            [get_effective_zero_page_addr, read_data],
            [get_effective_zero_page_addr, write_data],
            [get_effective_zero_page_addr, self.op],
        ]
    }
}

pub struct ZeroPageX {
    pub op: HalfCycle,
}

impl Instruction<5> for ZeroPageX {
    fn get_cycles(&self) -> [Cycle; 5] {
        [
            GET_BASE_ADDR_LOW_BYTE,
            READ_FROM_BASE_ZERO_PAGE_ADDR,
            [get_base_zero_page_x_indexed_addr, read_data],
            [get_base_zero_page_x_indexed_addr, write_data],
            [get_base_zero_page_x_indexed_addr, self.op],
        ]
    }
}

pub struct Absolute {
    pub op: HalfCycle,
}

impl Instruction<5> for Absolute {
    fn get_cycles(&self) -> [Cycle; 5] {
        [
            GET_EFFECTIVE_ADDR_LOW_BYTE,
            GET_EFFECTIVE_ADDR_HIGH_BYTE,
            READ_FROM_EFFECTIVE_ADDRESS,
            WRITE_TO_EFFECTIVE_ADDRESS,
            [get_effective_addr, self.op],
        ]
    }
}

pub struct AbsoluteX {
    pub op: HalfCycle,
}

impl Instruction<6> for AbsoluteX {
    fn get_cycles(&self) -> [Cycle; 6] {
        [
            GET_BASE_ADDR_LOW_BYTE,
            GET_BASE_ADDR_HIGH_BYTE,
            [get_base_addr_x_indexed_with_carry, read_data],
            READ_FROM_EFFECTIVE_ADDRESS,
            WRITE_TO_EFFECTIVE_ADDRESS,
            [get_effective_addr, self.op],
        ]
    }
}

pub struct AbsoluteY {
    pub op: HalfCycle,
}

impl Instruction<6> for AbsoluteY {
    fn get_cycles(&self) -> [Cycle; 6] {
        [
            GET_BASE_ADDR_LOW_BYTE,
            GET_BASE_ADDR_HIGH_BYTE,
            [get_base_addr_y_indexed_with_carry, read_data],
            READ_FROM_EFFECTIVE_ADDRESS,
            WRITE_TO_EFFECTIVE_ADDRESS,
            [get_effective_addr, self.op],
        ]
    }
}

pub struct IndirectX {
    pub op: HalfCycle,
}

impl Instruction<7> for IndirectX {
    fn get_cycles(&self) -> [Cycle; 7] {
        [
            GET_BASE_ADDR_LOW_BYTE,
            [get_base_zero_page_addr, read_data],
            [
                get_base_zero_page_x_indexed_addr,
                read_effective_addr_low_byte,
            ],
            [
                get_base_zero_page_x_indexed_addr_high_byte,
                read_effective_addr_high_byte,
            ],
            READ_FROM_EFFECTIVE_ADDRESS,
            WRITE_TO_EFFECTIVE_ADDRESS,
            [get_effective_addr, self.op],
        ]
    }
}

pub struct IndirectY {
    pub op: HalfCycle,
}

impl Instruction<7> for IndirectY {
    fn get_cycles(&self) -> [Cycle; 7] {
        [
            [get_pc_with_inc, read_indirect_addr_low_byte],
            [get_indirect_zero_page_addr, read_base_addr_low_byte],
            [
                get_indirect_zero_page_addr_high_byte,
                read_base_addr_high_byte,
            ],
            [get_base_addr_y_indexed_with_carry, read_data],
            READ_FROM_EFFECTIVE_ADDRESS,
            WRITE_TO_EFFECTIVE_ADDRESS,
            [get_effective_addr, self.op],
        ]
    }
}

pub struct Halt {}

impl Instruction<1> for Halt {
    fn get_cycles(&self) -> [Cycle; 1] {
        [[nop, jam]]
    }
}
