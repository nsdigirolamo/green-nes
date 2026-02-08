use crate::emu::cpu::{cycles::*, half_cycles::*, instructions::Instruction};

pub struct Immediate {
    pub op: HalfCycle,
}

impl Instruction<1> for Immediate {
    fn get_cycles(&self) -> [Cycle; 1] {
        [[get_pc, self.op]]
    }
}

pub struct ZeroPage {
    pub op: HalfCycle,
}

impl Instruction<2> for ZeroPage {
    fn get_cycles(&self) -> [Cycle; 2] {
        [
            FETCH_LOW_EFFECTIVE_ADDRESS_BYTE,
            [get_effective_zero_page_address, self.op],
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
            [get_effective_address, self.op],
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
            [get_base_zero_page_address, read_data],
            [
                get_indirect_x_indexed_low_address_byte,
                read_low_effective_address_byte,
            ],
            [
                get_indirect_x_indexed_high_address_byte,
                read_high_effective_address_byte,
            ],
            [get_effective_address, self.op],
        ]
    }
}

pub struct IndirectY {
    pub op: HalfCycle,
}

impl Instruction<4> for IndirectY {
    fn get_cycles(&self) -> [Cycle; 4] {
        [
            [get_pc, read_low_indirect_address_byte],
            [
                get_indirect_zero_page_low_address_byte,
                read_low_base_address_byte,
            ],
            [
                get_indirect_zero_page_high_address_byte,
                read_high_base_address_byte,
            ],
            [get_y_indexed_base_address_with_carry, self.op],
        ]
    }
}

pub struct AbsoluteX {
    pub op: HalfCycle,
}

impl Instruction<3> for AbsoluteX {
    fn get_cycles(&self) -> [Cycle; 3] {
        [
            [get_pc, read_low_base_address_byte],
            [get_pc, read_high_base_address_byte],
            [get_x_indexed_base_address_with_carry, self.op],
        ]
    }
}

pub struct AbsoluteY {
    pub op: HalfCycle,
}

impl Instruction<3> for AbsoluteY {
    fn get_cycles(&self) -> [Cycle; 3] {
        [
            [get_pc, read_low_base_address_byte],
            [get_pc, read_high_base_address_byte],
            [get_y_indexed_base_address_with_carry, self.op],
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
            [get_base_zero_page_address, read_data],
            [get_effective_zero_page_x_indexed_address, self.op],
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
            [get_base_zero_page_address, read_data],
            [get_effective_zero_page_y_indexed_address, self.op],
        ]
    }
}
