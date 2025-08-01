use crate::emu::{
    half_cycles::{
        get_pc, read_high_base_address_byte, read_high_effective_address_byte,
        read_low_base_address_byte, read_low_effective_address_byte, read_opcode,
    },
    instructions::{
        Instruction, miscellaneous::Miscellaneous, read::Read, read_modify_write::ReadModifyWrite,
        single_byte::SingleByte, store::Store, unofficial::Unofficial,
    },
    operations::{
        access, arithmetic, bitwise, branch, compare, flags, jump, other, shift, stack, transfer,
        unofficial,
    },
    state::Cycle,
};

pub const FETCH_INSTRUCTION: Cycle = [get_pc, read_opcode];
pub const FETCH_HIGH_EFFECTIVE_ADDRESS_BYTE: Cycle = [get_pc, read_high_effective_address_byte];
pub const FETCH_LOW_EFFECTIVE_ADDRESS_BYTE: Cycle = [get_pc, read_low_effective_address_byte];
pub const FETCH_HIGH_BASE_ADDRESS_BYTE: Cycle = [get_pc, read_high_base_address_byte];
pub const FETCH_LOW_BASE_ADDRESS_BYTE: Cycle = [get_pc, read_low_base_address_byte];

pub fn get_cycles(opcode: u8) -> Vec<Cycle> {
    match opcode {
        0x00 => Miscellaneous::Break.get_cycles(other::nop),
        0x01 => Read::IndirectX.get_cycles(bitwise::ora),
        0x02 => Unofficial::Halt.get_cycles(unofficial::jam),
        0x03 => Unofficial::IndirectX.get_cycles(unofficial::slo),
        0x04 => Read::ZeroPage.get_cycles(other::nop),
        0x05 => Read::ZeroPage.get_cycles(bitwise::ora),
        0x06 => ReadModifyWrite::ZeroPage.get_cycles(shift::asl),
        0x07 => Unofficial::ZeroPage.get_cycles(unofficial::slo),
        0x08 => Miscellaneous::Push.get_cycles(stack::php),
        0x09 => Read::Immediate.get_cycles(bitwise::ora),
        0x0A => SingleByte::Default.get_cycles(shift::asl_accumulator),
        0x0B => panic!("Opcode 0x0B not implemented"),
        0x0C => Read::Absolute.get_cycles(other::nop),
        0x0D => Read::Absolute.get_cycles(bitwise::ora),
        0x0E => ReadModifyWrite::Absolute.get_cycles(shift::asl),
        0x0F => Unofficial::Absolute.get_cycles(unofficial::slo),
        0x10 => Miscellaneous::Branch.get_cycles(branch::bpl),
        0x11 => Read::IndirectY.get_cycles(bitwise::ora_indirect_y),
        0x12 => Unofficial::Halt.get_cycles(unofficial::jam),
        0x13 => Unofficial::IndirectY.get_cycles(unofficial::slo),
        0x14 => Read::ZeroPageX.get_cycles(other::nop),
        0x15 => Read::ZeroPageX.get_cycles(bitwise::ora),
        0x16 => ReadModifyWrite::ZeroPageX.get_cycles(shift::asl),
        0x17 => Unofficial::ZeroPageX.get_cycles(unofficial::slo),
        0x18 => SingleByte::Default.get_cycles(flags::clc),
        0x19 => Read::AbsoluteY.get_cycles(bitwise::ora),
        0x1A => SingleByte::Default.get_cycles(other::nop),
        0x1B => Unofficial::AbsoluteY.get_cycles(unofficial::slo),
        0x1C => Read::AbsoluteX.get_cycles(other::nop_absolute_indexed),
        0x1D => Read::AbsoluteX.get_cycles(bitwise::ora_absolute_indexed),
        0x1E => ReadModifyWrite::AbsoluteX.get_cycles(shift::asl),
        0x1F => Unofficial::AbsoluteX.get_cycles(unofficial::slo),
        0x20 => Miscellaneous::JumpToSubroutine.get_cycles(jump::jsr),
        0x21 => Read::IndirectX.get_cycles(bitwise::and),
        0x22 => Unofficial::Halt.get_cycles(unofficial::jam),
        0x23 => Unofficial::IndirectX.get_cycles(unofficial::rla),
        0x24 => Read::ZeroPage.get_cycles(bitwise::bit),
        0x25 => Read::ZeroPage.get_cycles(bitwise::and),
        0x26 => ReadModifyWrite::ZeroPage.get_cycles(shift::rol),
        0x27 => Unofficial::ZeroPage.get_cycles(unofficial::rla),
        0x28 => Miscellaneous::Pull.get_cycles(stack::plp),
        0x29 => Read::Immediate.get_cycles(bitwise::and),
        0x2A => SingleByte::Default.get_cycles(shift::rol_accumulator),
        0x2B => panic!("Opcode 0x2B not implemented"),
        0x2C => Read::Absolute.get_cycles(bitwise::bit),
        0x2D => Read::Absolute.get_cycles(bitwise::and),
        0x2E => ReadModifyWrite::Absolute.get_cycles(shift::rol),
        0x2F => Unofficial::Absolute.get_cycles(unofficial::rla),
        0x30 => Miscellaneous::Branch.get_cycles(branch::bmi),
        0x31 => Read::IndirectY.get_cycles(bitwise::and_indirect_y),
        0x32 => Unofficial::Halt.get_cycles(unofficial::jam),
        0x33 => Unofficial::IndirectY.get_cycles(unofficial::rla),
        0x34 => Read::ZeroPageX.get_cycles(other::nop),
        0x35 => Read::ZeroPageX.get_cycles(bitwise::and),
        0x36 => ReadModifyWrite::ZeroPageX.get_cycles(shift::rol),
        0x37 => Unofficial::ZeroPageX.get_cycles(unofficial::rla),
        0x38 => SingleByte::Default.get_cycles(flags::sec),
        0x39 => Read::AbsoluteY.get_cycles(bitwise::and),
        0x3A => SingleByte::Default.get_cycles(other::nop),
        0x3B => Unofficial::AbsoluteY.get_cycles(unofficial::rla),
        0x3C => Read::AbsoluteX.get_cycles(other::nop_absolute_indexed),
        0x3D => Read::AbsoluteX.get_cycles(bitwise::and_absolute_indexed),
        0x3E => ReadModifyWrite::AbsoluteX.get_cycles(shift::rol),
        0x3F => Unofficial::AbsoluteX.get_cycles(unofficial::rla),
        0x40 => Miscellaneous::ReturnFromInterrupt.get_cycles(jump::rti),
        0x41 => Read::IndirectX.get_cycles(bitwise::eor),
        0x42 => Unofficial::Halt.get_cycles(unofficial::jam),
        0x43 => Unofficial::IndirectX.get_cycles(unofficial::sre),
        0x44 => Read::ZeroPage.get_cycles(other::nop),
        0x45 => Read::ZeroPage.get_cycles(bitwise::eor),
        0x46 => ReadModifyWrite::ZeroPage.get_cycles(shift::lsr),
        0x47 => Unofficial::ZeroPage.get_cycles(unofficial::sre),
        0x48 => Miscellaneous::Push.get_cycles(stack::pha),
        0x49 => Read::Immediate.get_cycles(bitwise::eor),
        0x4A => SingleByte::Default.get_cycles(shift::lsr_accumulator),
        0x4B => panic!("Opcode 0x4B not implemented"),
        0x4C => Miscellaneous::JumpAbsolute.get_cycles(jump::jmp_absolute),
        0x4D => Read::Absolute.get_cycles(bitwise::eor),
        0x4E => ReadModifyWrite::Absolute.get_cycles(shift::lsr),
        0x4F => Unofficial::Absolute.get_cycles(unofficial::sre),
        0x50 => Miscellaneous::Branch.get_cycles(branch::bvc),
        0x51 => Read::IndirectY.get_cycles(bitwise::eor_indirect_y),
        0x52 => Unofficial::Halt.get_cycles(unofficial::jam),
        0x53 => Unofficial::IndirectY.get_cycles(unofficial::sre),
        0x54 => Read::ZeroPageX.get_cycles(other::nop),
        0x55 => Read::ZeroPageX.get_cycles(bitwise::eor),
        0x56 => ReadModifyWrite::ZeroPageX.get_cycles(shift::lsr),
        0x57 => Unofficial::ZeroPageX.get_cycles(unofficial::sre),
        0x58 => SingleByte::Default.get_cycles(flags::cli),
        0x59 => Read::AbsoluteY.get_cycles(bitwise::eor),
        0x5A => SingleByte::Default.get_cycles(other::nop),
        0x5B => Unofficial::AbsoluteY.get_cycles(unofficial::sre),
        0x5C => Read::AbsoluteX.get_cycles(other::nop_absolute_indexed),
        0x5D => Read::AbsoluteX.get_cycles(bitwise::eor_absolute_indexed),
        0x5E => ReadModifyWrite::AbsoluteX.get_cycles(shift::lsr),
        0x5F => Unofficial::AbsoluteX.get_cycles(unofficial::sre),
        0x60 => Miscellaneous::ReturnFromSubroutine.get_cycles(other::nop), // RTS needs no operation
        0x61 => Read::IndirectX.get_cycles(arithmetic::adc),
        0x62 => Unofficial::Halt.get_cycles(unofficial::jam),
        0x63 => Unofficial::IndirectX.get_cycles(unofficial::rra),
        0x64 => Read::ZeroPage.get_cycles(other::nop),
        0x65 => Read::ZeroPage.get_cycles(arithmetic::adc),
        0x66 => ReadModifyWrite::ZeroPage.get_cycles(shift::ror),
        0x67 => Unofficial::ZeroPage.get_cycles(unofficial::rra),
        0x68 => Miscellaneous::Pull.get_cycles(stack::pla),
        0x69 => Read::Immediate.get_cycles(arithmetic::adc),
        0x6A => SingleByte::Default.get_cycles(shift::ror_accumulator),
        0x6B => panic!("Opcode 0x6B not implemented"),
        0x6C => Miscellaneous::JumpIndirect.get_cycles(other::nop), // JMP (Indirect) needs no operation
        0x6D => Read::Absolute.get_cycles(arithmetic::adc),
        0x6E => ReadModifyWrite::Absolute.get_cycles(shift::ror),
        0x6F => Unofficial::Absolute.get_cycles(unofficial::rra),
        0x70 => Miscellaneous::Branch.get_cycles(branch::bvs),
        0x71 => Read::IndirectY.get_cycles(arithmetic::adc_indirect_y),
        0x72 => Unofficial::Halt.get_cycles(unofficial::jam),
        0x73 => Unofficial::IndirectY.get_cycles(unofficial::rra),
        0x74 => Read::ZeroPageX.get_cycles(other::nop),
        0x75 => Read::ZeroPageX.get_cycles(arithmetic::adc),
        0x76 => ReadModifyWrite::ZeroPageX.get_cycles(shift::ror),
        0x77 => Unofficial::ZeroPageX.get_cycles(unofficial::rra),
        0x78 => SingleByte::Default.get_cycles(flags::sei),
        0x79 => Read::AbsoluteY.get_cycles(arithmetic::adc),
        0x7A => SingleByte::Default.get_cycles(other::nop),
        0x7B => Unofficial::AbsoluteY.get_cycles(unofficial::rra),
        0x7C => Read::AbsoluteX.get_cycles(other::nop_absolute_indexed),
        0x7D => Read::AbsoluteX.get_cycles(arithmetic::adc_absolute_indexed),
        0x7E => ReadModifyWrite::AbsoluteX.get_cycles(shift::ror),
        0x7F => Unofficial::AbsoluteX.get_cycles(unofficial::rra),
        0x80 => Read::Immediate.get_cycles(other::nop),
        0x81 => Store::IndirectX.get_cycles(access::sta),
        0x82 => Read::Immediate.get_cycles(other::nop),
        0x83 => Store::IndirectX.get_cycles(unofficial::sax),
        0x84 => Store::ZeroPage.get_cycles(access::sty),
        0x85 => Store::ZeroPage.get_cycles(access::sta),
        0x86 => Store::ZeroPage.get_cycles(access::stx),
        0x87 => Store::ZeroPage.get_cycles(unofficial::sax),
        0x88 => SingleByte::Default.get_cycles(arithmetic::dey),
        0x89 => Read::Immediate.get_cycles(other::nop),
        0x8A => SingleByte::Default.get_cycles(transfer::txa),
        0x8B => panic!("Opcode 0x8B not implemented"),
        0x8C => Store::Absolute.get_cycles(access::sty),
        0x8D => Store::Absolute.get_cycles(access::sta),
        0x8E => Store::Absolute.get_cycles(access::stx),
        0x8F => Store::Absolute.get_cycles(unofficial::sax),
        0x90 => Miscellaneous::Branch.get_cycles(branch::bcc),
        0x91 => Store::IndirectY.get_cycles(access::sta),
        0x92 => Unofficial::Halt.get_cycles(unofficial::jam),
        0x93 => panic!("Opcode 0x93 not implemented"),
        0x94 => Store::ZeroPageX.get_cycles(access::sty),
        0x95 => Store::ZeroPageX.get_cycles(access::sta),
        0x96 => Store::ZeroPageY.get_cycles(access::stx),
        0x97 => Store::ZeroPageY.get_cycles(unofficial::sax),
        0x98 => SingleByte::Default.get_cycles(transfer::tya),
        0x99 => Store::AbsoluteY.get_cycles(access::sta),
        0x9A => SingleByte::Default.get_cycles(transfer::txs),
        0x9B => panic!("Opcode 0x9B not implemented"),
        0x9C => panic!("Opcode 0x9C not implemented"),
        0x9D => Store::AbsoluteX.get_cycles(access::sta),
        0x9E => panic!("Opcode 0x9E not implemented"),
        0x9F => panic!("Opcode 0x9F not implemented"),
        0xA0 => Read::Immediate.get_cycles(access::ldy),
        0xA1 => Read::IndirectX.get_cycles(access::lda),
        0xA2 => Read::Immediate.get_cycles(access::ldx),
        0xA3 => Read::IndirectX.get_cycles(unofficial::lax),
        0xA4 => Read::ZeroPage.get_cycles(access::ldy),
        0xA5 => Read::ZeroPage.get_cycles(access::lda),
        0xA6 => Read::ZeroPage.get_cycles(access::ldx),
        0xA7 => Read::ZeroPage.get_cycles(unofficial::lax),
        0xA8 => SingleByte::Default.get_cycles(transfer::tay),
        0xA9 => Read::Immediate.get_cycles(access::lda),
        0xAA => SingleByte::Default.get_cycles(transfer::tax),
        0xAB => panic!("Opcode 0xAB not implemented"),
        0xAC => Read::Absolute.get_cycles(access::ldy),
        0xAD => Read::Absolute.get_cycles(access::lda),
        0xAE => Read::Absolute.get_cycles(access::ldx),
        0xAF => Read::Absolute.get_cycles(unofficial::lax),
        0xB0 => Miscellaneous::Branch.get_cycles(branch::bcs),
        0xB1 => Read::IndirectY.get_cycles(access::lda_indirect_y),
        0xB2 => Unofficial::Halt.get_cycles(unofficial::jam),
        0xB3 => Read::IndirectY.get_cycles(unofficial::lax_indirect_y),
        0xB4 => Read::ZeroPageX.get_cycles(access::ldy),
        0xB5 => Read::ZeroPageX.get_cycles(access::lda),
        0xB6 => Read::ZeroPageY.get_cycles(access::ldx),
        0xB7 => Read::ZeroPageY.get_cycles(unofficial::lax),
        0xB8 => SingleByte::Default.get_cycles(flags::clv),
        0xB9 => Read::AbsoluteY.get_cycles(access::lda_absolute_indexed),
        0xBA => SingleByte::Default.get_cycles(transfer::tsx),
        0xBB => panic!("Opcode 0xBB not implemented"),
        0xBC => Read::AbsoluteX.get_cycles(access::ldy_absolute_indexed),
        0xBD => Read::AbsoluteX.get_cycles(access::lda_absolute_indexed),
        0xBE => Read::AbsoluteY.get_cycles(access::ldx_absolute_indexed),
        0xBF => Read::AbsoluteY.get_cycles(unofficial::lax_absolute_indexed),
        0xC0 => Read::Immediate.get_cycles(compare::cpy),
        0xC1 => Read::IndirectX.get_cycles(compare::cmp),
        0xC2 => Read::Immediate.get_cycles(other::nop),
        0xC3 => Unofficial::IndirectX.get_cycles(unofficial::dcp),
        0xC4 => Read::ZeroPage.get_cycles(compare::cpy),
        0xC5 => Read::ZeroPage.get_cycles(compare::cmp),
        0xC6 => ReadModifyWrite::ZeroPage.get_cycles(arithmetic::dec),
        0xC7 => Unofficial::ZeroPage.get_cycles(unofficial::dcp),
        0xC8 => SingleByte::Default.get_cycles(arithmetic::iny),
        0xC9 => Read::Immediate.get_cycles(compare::cmp),
        0xCA => SingleByte::Default.get_cycles(arithmetic::dex),
        0xCB => panic!("Opcode 0xCB not implemented"),
        0xCC => Read::Absolute.get_cycles(compare::cpy),
        0xCD => Read::Absolute.get_cycles(compare::cmp),
        0xCE => ReadModifyWrite::Absolute.get_cycles(arithmetic::dec),
        0xCF => Unofficial::Absolute.get_cycles(unofficial::dcp),
        0xD0 => Miscellaneous::Branch.get_cycles(branch::bne),
        0xD1 => Read::IndirectY.get_cycles(compare::cmp_indirect_y),
        0xD2 => Unofficial::Halt.get_cycles(unofficial::jam),
        0xD3 => Unofficial::IndirectY.get_cycles(unofficial::dcp),
        0xD4 => Read::ZeroPageX.get_cycles(other::nop),
        0xD5 => Read::ZeroPageX.get_cycles(compare::cmp),
        0xD6 => ReadModifyWrite::ZeroPageX.get_cycles(arithmetic::dec),
        0xD7 => Unofficial::ZeroPageX.get_cycles(unofficial::dcp),
        0xD8 => SingleByte::Default.get_cycles(flags::cld),
        0xD9 => Read::AbsoluteY.get_cycles(compare::cmp),
        0xDA => SingleByte::Default.get_cycles(other::nop),
        0xDB => Unofficial::AbsoluteY.get_cycles(unofficial::dcp),
        0xDC => Read::AbsoluteX.get_cycles(other::nop_absolute_indexed),
        0xDD => Read::AbsoluteX.get_cycles(compare::cmp_absolute_indexed),
        0xDE => ReadModifyWrite::AbsoluteX.get_cycles(arithmetic::dec),
        0xDF => Unofficial::AbsoluteX.get_cycles(unofficial::dcp),
        0xE0 => Read::Immediate.get_cycles(compare::cpx),
        0xE1 => Read::IndirectX.get_cycles(arithmetic::sbc),
        0xE2 => Read::Immediate.get_cycles(other::nop),
        0xE3 => Unofficial::IndirectX.get_cycles(unofficial::isc),
        0xE4 => Read::ZeroPage.get_cycles(compare::cpx),
        0xE5 => Read::ZeroPage.get_cycles(arithmetic::sbc),
        0xE6 => ReadModifyWrite::ZeroPage.get_cycles(arithmetic::inc),
        0xE7 => Unofficial::ZeroPage.get_cycles(unofficial::isc),
        0xE8 => SingleByte::Default.get_cycles(arithmetic::inx),
        0xE9 => Read::Immediate.get_cycles(arithmetic::sbc),
        0xEA => SingleByte::Default.get_cycles(other::nop),
        0xEB => Read::Immediate.get_cycles(unofficial::usbc),
        0xEC => Read::Absolute.get_cycles(compare::cpx),
        0xED => Read::Absolute.get_cycles(arithmetic::sbc),
        0xEE => ReadModifyWrite::Absolute.get_cycles(arithmetic::inc),
        0xEF => Unofficial::Absolute.get_cycles(unofficial::isc),
        0xF0 => Miscellaneous::Branch.get_cycles(branch::beq),
        0xF1 => Read::IndirectY.get_cycles(arithmetic::sbc_indirect_y),
        0xF2 => Unofficial::Halt.get_cycles(unofficial::jam),
        0xF3 => Unofficial::IndirectY.get_cycles(unofficial::isc),
        0xF4 => Read::ZeroPageX.get_cycles(other::nop),
        0xF5 => Read::ZeroPageX.get_cycles(arithmetic::sbc),
        0xF6 => ReadModifyWrite::ZeroPageX.get_cycles(arithmetic::inc),
        0xF7 => Unofficial::ZeroPageX.get_cycles(unofficial::isc),
        0xF8 => SingleByte::Default.get_cycles(flags::sed),
        0xF9 => Read::AbsoluteY.get_cycles(arithmetic::sbc),
        0xFA => SingleByte::Default.get_cycles(other::nop),
        0xFB => Unofficial::AbsoluteY.get_cycles(unofficial::isc),
        0xFC => Read::AbsoluteX.get_cycles(other::nop_absolute_indexed),
        0xFD => Read::AbsoluteX.get_cycles(arithmetic::sbc_absolute_indexed),
        0xFE => ReadModifyWrite::AbsoluteX.get_cycles(arithmetic::inc),
        0xFF => Unofficial::AbsoluteX.get_cycles(unofficial::isc),
    }
}
