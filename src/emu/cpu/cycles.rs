use crate::emu::cpu::{
    CPU,
    half_cycles::{
        operations::{
            access::*, arithmetic::*, bitwise::*, branch::*, compare::*, flags::*, jump::*,
            other::*, shift::*, stack::*, transfer::*, unofficial::*,
        },
        *,
    },
    instructions::{
        Instruction, miscellaneous, read, read_modify_write, single_byte, store, unofficial,
    },
};

/// A single execution cycle for the CPU.
pub type Cycle = [HalfCycle; 2];

/// Fetches the value addressed by the program counter and stores it into the
/// instruction register.
pub const FETCH_INSTRUCTION: Cycle = [get_pc, read_opcode];

/// Fetches the value addressed by the program counter and stores it into the
/// effective address high byte.
pub const FETCH_HIGH_EFFECTIVE_ADDRESS_BYTE: Cycle = [get_pc, read_high_effective_address_byte];

/// Fetches the value addressed by the program counter and stores it into the
/// effective address low byte.
pub const FETCH_LOW_EFFECTIVE_ADDRESS_BYTE: Cycle = [get_pc, read_low_effective_address_byte];

/// Reads the value addressed by the effective address, placing it on the data
/// bus.
pub const READ_FROM_EFFECTIVE_ADDRESS: Cycle = [get_effective_address, read_data];

/// Writes the value from the data bus to the location addressed by the
/// effective address.
pub const WRITE_TO_EFFECTIVE_ADDRESS: Cycle = [get_effective_address, write_data];

/// Reads the value addressed by the low byte of the effective address from the
/// zero page, placing it on the data bus.
pub const READ_FROM_EFFECTIVE_ZERO_PAGE_ADDRESS: Cycle =
    [get_effective_zero_page_address, read_data];

/// Writes the value from the data bus to the zero page location addressed by
/// the low byte of the effective address.
pub const WRITE_TO_EFFECTIVE_ZERO_PAGE_ADDRESS: Cycle =
    [get_effective_zero_page_address, write_data];

/// Fetches the value addressed by the program counter and stores it into the
/// base address high byte.
pub const FETCH_HIGH_BASE_ADDRESS_BYTE: Cycle = [get_pc, read_high_base_address_byte];

/// Fetches the value addressed by the program counter and stores it into the
/// base address low byte.
pub const FETCH_LOW_BASE_ADDRESS_BYTE: Cycle = [get_pc, read_low_base_address_byte];

/// Reads from the value addressed by the low byte of the base address from the
/// zero page, placing it on the data bus.
pub const READ_FROM_BASE_ZERO_PAGE_ADDRESS: Cycle = [get_base_zero_page_address, read_data];

/// Pushes the high byte of the program counter to the top of the stack.
pub const PUSH_PC_HIGH_TO_STACK: Cycle = [push_stack, write_pc_high];

/// Pushes the low byte of the program counter to the top of the stack.
pub const PUSH_PC_LOW_TO_STACK: Cycle = [push_stack, write_pc_low];

/// Same functionality as BRK but with different vector.
pub const HANDLE_NMI: [Cycle; 6] = [
    [get_pc_without_increment, read_data],
    [push_stack, write_pc_high],
    [push_stack, write_pc_low],
    [push_stack, write_break_status],
    [get_low_nmi_vector, read_low_pc_address_byte],
    [get_high_nmi_vector, read_high_pc_address_byte],
];

/// Same functionality as BRK.
pub const HANDLE_IRQ: [Cycle; 6] = [
    [get_pc_without_increment, read_data],
    [push_stack, write_pc_high],
    [push_stack, write_pc_low],
    [push_stack, write_break_status],
    [get_low_irq_vector, read_low_pc_address_byte],
    [get_high_irq_vector, read_high_pc_address_byte],
];

pub fn get_cycles(cpu: &mut CPU, opcode: u8) {
    let cycles: &[Cycle] = match opcode {
        0x00 => &miscellaneous::Break {}.get_cycles(),
        0x01 => &read::IndirectX { op: ora }.get_cycles(),
        0x02 => &unofficial::Halt {}.get_cycles(),
        0x03 => &unofficial::IndirectX { op: slo }.get_cycles(),
        0x04 => &read::ZeroPage { op: nop }.get_cycles(),
        0x05 => &read::ZeroPage { op: ora }.get_cycles(),
        0x06 => &read_modify_write::ZeroPage { op: asl_m }.get_cycles(),
        0x07 => &unofficial::ZeroPage { op: slo }.get_cycles(),
        0x08 => &miscellaneous::Push { op: php }.get_cycles(),
        0x09 => &read::Immediate { op: ora }.get_cycles(),
        0x0A => &single_byte::SingleByte { op: asl_a }.get_cycles(),
        0x0B => &read::Immediate { op: anc }.get_cycles(),
        0x0C => &read::Absolute { op: nop }.get_cycles(),
        0x0D => &read::Absolute { op: ora }.get_cycles(),
        0x0E => &read_modify_write::Absolute { op: asl_m }.get_cycles(),
        0x0F => &unofficial::Absolute { op: slo }.get_cycles(),

        0x10 => &miscellaneous::Branch { op: bpl }.get_cycles(),
        0x11 => &read::IndirectY { op: ora_indirect_y }.get_cycles(),
        0x12 => &unofficial::Halt {}.get_cycles(),
        0x13 => &unofficial::IndirectY { op: slo }.get_cycles(),
        0x14 => &read::ZeroPageX { op: nop }.get_cycles(),
        0x15 => &read::ZeroPageX { op: ora }.get_cycles(),
        0x16 => &read_modify_write::ZeroPageX { op: asl_m }.get_cycles(),
        0x17 => &unofficial::ZeroPageX { op: slo }.get_cycles(),
        0x18 => &single_byte::SingleByte { op: clc }.get_cycles(),
        0x19 => &read::AbsoluteY { op: ora }.get_cycles(),
        0x1A => &single_byte::SingleByte { op: nop }.get_cycles(),
        0x1B => &unofficial::AbsoluteY { op: slo }.get_cycles(),
        0x1C => &read::AbsoluteX { op: nop_abs_index }.get_cycles(),
        0x1D => &read::AbsoluteX { op: ora_abs_index }.get_cycles(),
        0x1E => &read_modify_write::AbsoluteX { op: asl_m }.get_cycles(),
        0x1F => &unofficial::AbsoluteX { op: slo }.get_cycles(),

        0x20 => &miscellaneous::JumpToSubroutine { op: jsr }.get_cycles(),
        0x21 => &read::IndirectX { op: and }.get_cycles(),
        0x22 => &unofficial::Halt {}.get_cycles(),
        0x23 => &unofficial::IndirectX { op: rla }.get_cycles(),
        0x24 => &read::ZeroPage { op: bit }.get_cycles(),
        0x25 => &read::ZeroPage { op: and }.get_cycles(),
        0x26 => &read_modify_write::ZeroPage { op: rol }.get_cycles(),
        0x27 => &unofficial::ZeroPage { op: rla }.get_cycles(),
        0x28 => &miscellaneous::Pull { op: plp }.get_cycles(),
        0x29 => &read::Immediate { op: and }.get_cycles(),
        0x2A => &single_byte::SingleByte { op: rol_a }.get_cycles(),
        0x2B => &read::Immediate { op: anc }.get_cycles(),
        0x2C => &read::Absolute { op: bit }.get_cycles(),
        0x2D => &read::Absolute { op: and }.get_cycles(),
        0x2E => &read_modify_write::Absolute { op: rol }.get_cycles(),
        0x2F => &unofficial::Absolute { op: rla }.get_cycles(),

        0x30 => &miscellaneous::Branch { op: bmi }.get_cycles(),
        0x31 => &read::IndirectY { op: and_indirect_y }.get_cycles(),
        0x32 => &unofficial::Halt {}.get_cycles(),
        0x33 => &unofficial::IndirectY { op: rla }.get_cycles(),
        0x34 => &read::ZeroPageX { op: nop }.get_cycles(),
        0x35 => &read::ZeroPageX { op: and }.get_cycles(),
        0x36 => &read_modify_write::ZeroPageX { op: rol }.get_cycles(),
        0x37 => &unofficial::ZeroPageX { op: rla }.get_cycles(),
        0x38 => &single_byte::SingleByte { op: sec }.get_cycles(),
        0x39 => &read::AbsoluteY { op: and }.get_cycles(),
        0x3A => &single_byte::SingleByte { op: nop }.get_cycles(),
        0x3B => &unofficial::AbsoluteY { op: rla }.get_cycles(),
        0x3C => &read::AbsoluteX { op: nop_abs_index }.get_cycles(),
        0x3D => &read::AbsoluteX { op: and_abs_index }.get_cycles(),
        0x3E => &read_modify_write::AbsoluteX { op: rol }.get_cycles(),
        0x3F => &unofficial::AbsoluteX { op: rla }.get_cycles(),

        0x40 => &miscellaneous::ReturnFromInterrupt { op: rti }.get_cycles(),
        0x41 => &read::IndirectX { op: eor }.get_cycles(),
        0x42 => &unofficial::Halt {}.get_cycles(),
        0x43 => &unofficial::IndirectX { op: sre }.get_cycles(),
        0x44 => &read::ZeroPage { op: nop }.get_cycles(),
        0x45 => &read::ZeroPage { op: eor }.get_cycles(),
        0x46 => &read_modify_write::ZeroPage { op: lsr_m }.get_cycles(),
        0x47 => &unofficial::ZeroPage { op: sre }.get_cycles(),
        0x48 => &miscellaneous::Push { op: pha }.get_cycles(),
        0x49 => &read::Immediate { op: eor }.get_cycles(),
        0x4A => &single_byte::SingleByte { op: lsr_a }.get_cycles(),
        0x4B => &read::Immediate { op: alr }.get_cycles(),
        0x4C => &miscellaneous::JumpAbsolute { op: jmp_absolute }.get_cycles(),
        0x4D => &read::Absolute { op: eor }.get_cycles(),
        0x4E => &read_modify_write::Absolute { op: lsr_m }.get_cycles(),
        0x4F => &unofficial::Absolute { op: sre }.get_cycles(),

        0x50 => &miscellaneous::Branch { op: bvc }.get_cycles(),
        0x51 => &read::IndirectY { op: eor_indirect_y }.get_cycles(),
        0x52 => &unofficial::Halt {}.get_cycles(),
        0x53 => &unofficial::IndirectY { op: sre }.get_cycles(),
        0x54 => &read::ZeroPageX { op: nop }.get_cycles(),
        0x55 => &read::ZeroPageX { op: eor }.get_cycles(),
        0x56 => &read_modify_write::ZeroPageX { op: lsr_m }.get_cycles(),
        0x57 => &unofficial::ZeroPageX { op: sre }.get_cycles(),
        0x58 => &single_byte::SingleByte { op: cli }.get_cycles(),
        0x59 => &read::AbsoluteY { op: eor }.get_cycles(),
        0x5A => &single_byte::SingleByte { op: nop }.get_cycles(),
        0x5B => &unofficial::AbsoluteY { op: sre }.get_cycles(),
        0x5C => &read::AbsoluteX { op: nop_abs_index }.get_cycles(),
        0x5D => &read::AbsoluteX { op: eor_abs_index }.get_cycles(),
        0x5E => &read_modify_write::AbsoluteX { op: lsr_m }.get_cycles(),
        0x5F => &unofficial::AbsoluteX { op: sre }.get_cycles(),

        0x60 => &miscellaneous::ReturnFromSubroutine {}.get_cycles(),
        0x61 => &read::IndirectX { op: adc }.get_cycles(),
        0x62 => &unofficial::Halt {}.get_cycles(),
        0x63 => &unofficial::IndirectX { op: rra }.get_cycles(),
        0x64 => &read::ZeroPage { op: nop }.get_cycles(),
        0x65 => &read::ZeroPage { op: adc }.get_cycles(),
        0x66 => &read_modify_write::ZeroPage { op: ror_m }.get_cycles(),
        0x67 => &unofficial::ZeroPage { op: rra }.get_cycles(),
        0x68 => &miscellaneous::Pull { op: pla }.get_cycles(),
        0x69 => &read::Immediate { op: adc }.get_cycles(),
        0x6A => &single_byte::SingleByte { op: ror_a }.get_cycles(),
        0x6B => &read::Immediate { op: arr }.get_cycles(),
        0x6C => &miscellaneous::JumpIndirect {}.get_cycles(),
        0x6D => &read::Absolute { op: adc }.get_cycles(),
        0x6E => &read_modify_write::Absolute { op: ror_m }.get_cycles(),
        0x6F => &unofficial::Absolute { op: rra }.get_cycles(),

        0x70 => &miscellaneous::Branch { op: bvs }.get_cycles(),
        0x71 => &read::IndirectY { op: adc_indirect_y }.get_cycles(),
        0x72 => &unofficial::Halt {}.get_cycles(),
        0x73 => &unofficial::IndirectY { op: rra }.get_cycles(),
        0x74 => &read::ZeroPageX { op: nop }.get_cycles(),
        0x75 => &read::ZeroPageX { op: adc }.get_cycles(),
        0x76 => &read_modify_write::ZeroPageX { op: ror_m }.get_cycles(),
        0x77 => &unofficial::ZeroPageX { op: rra }.get_cycles(),
        0x78 => &single_byte::SingleByte { op: sei }.get_cycles(),
        0x79 => &read::AbsoluteY { op: adc }.get_cycles(),
        0x7A => &single_byte::SingleByte { op: nop }.get_cycles(),
        0x7B => &unofficial::AbsoluteY { op: rra }.get_cycles(),
        0x7C => &read::AbsoluteX { op: nop_abs_index }.get_cycles(),
        0x7D => &read::AbsoluteX { op: adc_abs_index }.get_cycles(),
        0x7E => &read_modify_write::AbsoluteX { op: ror_m }.get_cycles(),
        0x7F => &unofficial::AbsoluteX { op: rra }.get_cycles(),

        0x80 => &read::Immediate { op: nop }.get_cycles(),
        0x81 => &store::IndirectX { op: sta }.get_cycles(),
        0x82 => &read::Immediate { op: nop }.get_cycles(),
        0x83 => &store::IndirectX { op: sax }.get_cycles(),
        0x84 => &store::ZeroPage { op: sty }.get_cycles(),
        0x85 => &store::ZeroPage { op: sta }.get_cycles(),
        0x86 => &store::ZeroPage { op: stx }.get_cycles(),
        0x87 => &store::ZeroPage { op: sax }.get_cycles(),
        0x88 => &single_byte::SingleByte { op: dey }.get_cycles(),
        0x89 => &read::Immediate { op: nop }.get_cycles(),
        0x8A => &single_byte::SingleByte { op: txa }.get_cycles(),
        0x8B => todo!("opcode not yet implemented: {opcode:02X}"),
        0x8C => &store::Absolute { op: sty }.get_cycles(),
        0x8D => &store::Absolute { op: sta }.get_cycles(),
        0x8E => &store::Absolute { op: stx }.get_cycles(),
        0x8F => &store::Absolute { op: sax }.get_cycles(),

        0x90 => &miscellaneous::Branch { op: bcc }.get_cycles(),
        0x91 => &store::IndirectY { op: sta }.get_cycles(),
        0x92 => &unofficial::Halt {}.get_cycles(),
        0x93 => todo!("opcode not yet implemented: {opcode:02X}"),
        0x94 => &store::ZeroPageX { op: sty }.get_cycles(),
        0x95 => &store::ZeroPageX { op: sta }.get_cycles(),
        0x96 => &store::ZeroPageY { op: stx }.get_cycles(),
        0x97 => &store::ZeroPageY { op: sax }.get_cycles(),
        0x98 => &single_byte::SingleByte { op: tya }.get_cycles(),
        0x99 => &store::AbsoluteY { op: sta }.get_cycles(),
        0x9A => &single_byte::SingleByte { op: txs }.get_cycles(),
        0x9B => todo!("opcode not yet implemented: {opcode:02X}"),
        0x9C => todo!("opcode not yet implemented: {opcode:02X}"),
        0x9D => &store::AbsoluteX { op: sta }.get_cycles(),
        0x9E => todo!("opcode not yet implemented: {opcode:02X}"),
        0x9F => todo!("opcode not yet implemented: {opcode:02X}"),

        0xA0 => &read::Immediate { op: ldy }.get_cycles(),
        0xA1 => &read::IndirectX { op: lda }.get_cycles(),
        0xA2 => &read::Immediate { op: ldx }.get_cycles(),
        0xA3 => &read::IndirectX { op: lax }.get_cycles(),
        0xA4 => &read::ZeroPage { op: ldy }.get_cycles(),
        0xA5 => &read::ZeroPage { op: lda }.get_cycles(),
        0xA6 => &read::ZeroPage { op: ldx }.get_cycles(),
        0xA7 => &read::ZeroPage { op: lax }.get_cycles(),
        0xA8 => &single_byte::SingleByte { op: tay }.get_cycles(),
        0xA9 => &read::Immediate { op: lda }.get_cycles(),
        0xAA => &single_byte::SingleByte { op: tax }.get_cycles(),
        0xAB => todo!("opcode not yet implemented: {opcode:02X}"),
        0xAC => &read::Absolute { op: ldy }.get_cycles(),
        0xAD => &read::Absolute { op: lda }.get_cycles(),
        0xAE => &read::Absolute { op: ldx }.get_cycles(),
        0xAF => &read::Absolute { op: lax }.get_cycles(),

        0xB0 => &miscellaneous::Branch { op: bcs }.get_cycles(),
        0xB1 => &read::IndirectY { op: lda_indirect_y }.get_cycles(),
        0xB2 => &unofficial::Halt {}.get_cycles(),
        0xB3 => &read::IndirectY { op: lax_indirect_y }.get_cycles(),
        0xB4 => &read::ZeroPageX { op: ldy }.get_cycles(),
        0xB5 => &read::ZeroPageX { op: lda }.get_cycles(),
        0xB6 => &read::ZeroPageY { op: ldx }.get_cycles(),
        0xB7 => &read::ZeroPageY { op: lax }.get_cycles(),
        0xB8 => &single_byte::SingleByte { op: clv }.get_cycles(),
        0xB9 => &read::AbsoluteY { op: lda_abs_index }.get_cycles(),
        0xBA => &single_byte::SingleByte { op: tsx }.get_cycles(),
        0xBB => todo!("opcode not yet implemented: {opcode:02X}"),
        0xBC => &read::AbsoluteX { op: ldy_abs_index }.get_cycles(),
        0xBD => &read::AbsoluteX { op: lda_abs_index }.get_cycles(),
        0xBE => &read::AbsoluteY { op: ldx_abs_index }.get_cycles(),
        0xBF => &read::AbsoluteY { op: lax_abs_index }.get_cycles(),

        0xC0 => &read::Immediate { op: cpy }.get_cycles(),
        0xC1 => &read::IndirectX { op: cmp }.get_cycles(),
        0xC2 => &read::Immediate { op: nop }.get_cycles(),
        0xC3 => &unofficial::IndirectX { op: dcp }.get_cycles(),
        0xC4 => &read::ZeroPage { op: cpy }.get_cycles(),
        0xC5 => &read::ZeroPage { op: cmp }.get_cycles(),
        0xC6 => &read_modify_write::ZeroPage { op: dec }.get_cycles(),
        0xC7 => &unofficial::ZeroPage { op: dcp }.get_cycles(),
        0xC8 => &single_byte::SingleByte { op: iny }.get_cycles(),
        0xC9 => &read::Immediate { op: cmp }.get_cycles(),
        0xCA => &single_byte::SingleByte { op: dex }.get_cycles(),
        0xCB => &read::Immediate { op: axs }.get_cycles(),
        0xCC => &read::Absolute { op: cpy }.get_cycles(),
        0xCD => &read::Absolute { op: cmp }.get_cycles(),
        0xCE => &read_modify_write::Absolute { op: dec }.get_cycles(),
        0xCF => &unofficial::Absolute { op: dcp }.get_cycles(),

        0xD0 => &miscellaneous::Branch { op: bne }.get_cycles(),
        0xD1 => &read::IndirectY { op: cmp_indirect_y }.get_cycles(),
        0xD2 => &unofficial::Halt {}.get_cycles(),
        0xD3 => &unofficial::IndirectY { op: dcp }.get_cycles(),
        0xD4 => &read::ZeroPageX { op: nop }.get_cycles(),
        0xD5 => &read::ZeroPageX { op: cmp }.get_cycles(),
        0xD6 => &read_modify_write::ZeroPageX { op: dec }.get_cycles(),
        0xD7 => &unofficial::ZeroPageX { op: dcp }.get_cycles(),
        0xD8 => &single_byte::SingleByte { op: cld }.get_cycles(),
        0xD9 => &read::AbsoluteY { op: cmp }.get_cycles(),
        0xDA => &single_byte::SingleByte { op: nop }.get_cycles(),
        0xDB => &unofficial::AbsoluteY { op: dcp }.get_cycles(),
        0xDC => &read::AbsoluteX { op: nop_abs_index }.get_cycles(),
        0xDD => &read::AbsoluteX { op: cmp_abs_index }.get_cycles(),
        0xDE => &read_modify_write::AbsoluteX { op: dec }.get_cycles(),
        0xDF => &unofficial::AbsoluteX { op: dcp }.get_cycles(),

        0xE0 => &read::Immediate { op: cpx }.get_cycles(),
        0xE1 => &read::IndirectX { op: sbc }.get_cycles(),
        0xE2 => &read::Immediate { op: nop }.get_cycles(),
        0xE3 => &unofficial::IndirectX { op: isc }.get_cycles(),
        0xE4 => &read::ZeroPage { op: cpx }.get_cycles(),
        0xE5 => &read::ZeroPage { op: sbc }.get_cycles(),
        0xE6 => &read_modify_write::ZeroPage { op: inc }.get_cycles(),
        0xE7 => &unofficial::ZeroPage { op: isc }.get_cycles(),
        0xE8 => &single_byte::SingleByte { op: inx }.get_cycles(),
        0xE9 => &read::Immediate { op: sbc }.get_cycles(),
        0xEA => &single_byte::SingleByte { op: nop }.get_cycles(),
        0xEB => &read::Immediate { op: sbc }.get_cycles(),
        0xEC => &read::Absolute { op: cpx }.get_cycles(),
        0xED => &read::Absolute { op: sbc }.get_cycles(),
        0xEE => &read_modify_write::Absolute { op: inc }.get_cycles(),
        0xEF => &unofficial::Absolute { op: isc }.get_cycles(),

        0xF0 => &miscellaneous::Branch { op: beq }.get_cycles(),
        0xF1 => &read::IndirectY { op: sbc_indirect_y }.get_cycles(),
        0xF2 => &unofficial::Halt {}.get_cycles(),
        0xF3 => &unofficial::IndirectY { op: isc }.get_cycles(),
        0xF4 => &read::ZeroPageX { op: nop }.get_cycles(),
        0xF5 => &read::ZeroPageX { op: sbc }.get_cycles(),
        0xF6 => &read_modify_write::ZeroPageX { op: inc }.get_cycles(),
        0xF7 => &unofficial::ZeroPageX { op: isc }.get_cycles(),
        0xF8 => &single_byte::SingleByte { op: sed }.get_cycles(),
        0xF9 => &read::AbsoluteY { op: sbc }.get_cycles(),
        0xFA => &single_byte::SingleByte { op: nop }.get_cycles(),
        0xFB => &unofficial::AbsoluteY { op: isc }.get_cycles(),
        0xFC => &read::AbsoluteX { op: nop_abs_index }.get_cycles(),
        0xFD => &read::AbsoluteX { op: sbc_abs_index }.get_cycles(),
        0xFE => &read_modify_write::AbsoluteX { op: inc }.get_cycles(),
        0xFF => &unofficial::AbsoluteX { op: isc }.get_cycles(),
    };

    cpu.cycle_queue.extend(cycles);
}
