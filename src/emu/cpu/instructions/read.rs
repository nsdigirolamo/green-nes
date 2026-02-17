use crate::emu::cpu::{cycles::*, half_cycles::*, instructions::Instruction};

pub struct Immediate {
    pub op: HalfCycle,
}

impl Instruction<1> for Immediate {
    fn get_cycles(&self) -> [Cycle; 1] {
        [[get_pc_with_inc, self.op]]
    }
}

pub struct ZeroPage {
    pub op: HalfCycle,
}

impl Instruction<2> for ZeroPage {
    fn get_cycles(&self) -> [Cycle; 2] {
        [
            FETCH_LOW_EFFECTIVE_ADDRESS_BYTE,
            [get_effective_zero_page_addr, self.op],
        ]
    }
}

pub struct Absolute {
    pub op: HalfCycle,
}

impl Instruction<3> for Absolute {
    fn get_cycles(&self) -> [Cycle; 3] {
        [
            FETCH_LOW_EFFECTIVE_ADDRESS_BYTE,
            FETCH_HIGH_EFFECTIVE_ADDRESS_BYTE,
            [get_effective_addr, self.op],
        ]
    }
}

pub struct IndirectX {
    pub op: HalfCycle,
}

impl Instruction<5> for IndirectX {
    fn get_cycles(&self) -> [Cycle; 5] {
        [
            FETCH_LOW_BASE_ADDRESS_BYTE,
            READ_FROM_BASE_ZERO_PAGE_ADDRESS,
            [
                get_base_zero_page_x_indexed_addr,
                read_effective_addr_low_byte,
            ],
            [
                get_base_zero_page_x_indexed_addr_high_byte,
                read_effective_addr_high_byte,
            ],
            [get_effective_addr, self.op],
        ]
    }
}

pub struct IndirectY {
    pub op: HalfCycle,
}

impl Instruction<4> for IndirectY {
    fn get_cycles(&self) -> [Cycle; 4] {
        [
            [get_pc_with_inc, read_indirect_addr_low_byte],
            [get_indirect_zero_page_addr, read_base_addr_low_byte],
            [
                get_indirect_zero_page_addr_high_byte,
                read_base_addr_high_byte,
            ],
            [get_base_addr_y_indexed_with_carry, self.op],
        ]
    }
}

pub struct AbsoluteX {
    pub op: HalfCycle,
}

impl Instruction<3> for AbsoluteX {
    fn get_cycles(&self) -> [Cycle; 3] {
        [
            FETCH_LOW_BASE_ADDRESS_BYTE,
            FETCH_HIGH_BASE_ADDRESS_BYTE,
            [get_base_addr_x_indexed_with_carry, self.op],
        ]
    }
}

pub struct AbsoluteY {
    pub op: HalfCycle,
}

impl Instruction<3> for AbsoluteY {
    fn get_cycles(&self) -> [Cycle; 3] {
        [
            FETCH_LOW_BASE_ADDRESS_BYTE,
            FETCH_HIGH_BASE_ADDRESS_BYTE,
            [get_base_addr_y_indexed_with_carry, self.op],
        ]
    }
}

pub struct ZeroPageX {
    pub op: HalfCycle,
}

impl Instruction<3> for ZeroPageX {
    fn get_cycles(&self) -> [Cycle; 3] {
        [
            FETCH_LOW_BASE_ADDRESS_BYTE,
            READ_FROM_BASE_ZERO_PAGE_ADDRESS,
            [get_base_zero_page_x_indexed_addr, self.op],
        ]
    }
}

pub struct ZeroPageY {
    pub op: HalfCycle,
}

impl Instruction<3> for ZeroPageY {
    fn get_cycles(&self) -> [Cycle; 3] {
        [
            FETCH_LOW_BASE_ADDRESS_BYTE,
            READ_FROM_BASE_ZERO_PAGE_ADDRESS,
            [get_base_zero_page_y_indexed_addr, self.op],
        ]
    }
}
