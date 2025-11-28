use crate::{concat_u8, emu::nes::NES};

#[derive(Debug)]
pub enum AddressingMode {
    Accumulator,
    Relative,
    Immediate,
    Absolute,
    Implied,
    ZeroPage,
    Indirect,
    AbsoluteIndexedX,
    AbsoluteIndexedY,
    ZeroPageIndexedX,
    ZeroPageIndexedY,
    IndirectIndexedX,
    IndirectIndexedY,
}

pub fn get_debug_text(nes: NES) -> String {
    let (pch, pcl) = nes.cpu.get_registers().pc;
    let pc = concat_u8!(pch, pcl);
    let opcode = nes.buses.peek(pc);

    let label = get_label(opcode);
    let addressing_mode = get_addressing_mode(opcode);

    get_instruction_text(label, addressing_mode, nes)
}

pub fn get_instruction_text(label: &str, addressing_mode: AddressingMode, nes: NES) -> String {
    match addressing_mode {
        AddressingMode::Accumulator => format!("{label} A"),
        AddressingMode::Relative => {
            let (pch, pcl) = nes.cpu.get_registers().pc;
            let pc = concat_u8!(pch, pcl);

            let addr = pc + 1;
            let op = nes.buses.peek(addr);

            create_relative_debug_text(label, op, addr)
        }
        AddressingMode::Immediate => {
            let (pch, pcl) = nes.cpu.get_registers().pc;
            let pc = concat_u8!(pch, pcl);

            let op = nes.buses.peek(pc + 1);

            create_immediate_debug_text(label, op)
        }
        AddressingMode::Absolute => {
            let (pch, pcl) = nes.cpu.get_registers().pc;
            let pc = concat_u8!(pch, pcl);

            let op1 = nes.buses.peek(pc + 1);
            let op2 = nes.buses.peek(pc + 2);
            let addr = concat_u8!(op2, op1);

            create_absolute_debug_text(label, addr, nes.buses.peek(addr))
        }
        AddressingMode::Implied => label.to_string(),
        AddressingMode::ZeroPage => {
            let (pch, pcl) = nes.cpu.get_registers().pc;
            let pc = concat_u8!(pch, pcl);

            let op = nes.buses.peek(pc + 1);
            let addr = concat_u8!(0x00, op);

            create_zero_page_debug_text(label, op, nes.buses.peek(addr))
        }
        AddressingMode::Indirect => {
            let (pch, pcl) = nes.cpu.get_registers().pc;
            let pc = concat_u8!(pch, pcl);

            let op1 = nes.buses.peek(pc + 1);
            let op2 = nes.buses.peek(pc + 2);
            let pointer = concat_u8!(op2, op1);

            let addr_low_byte = nes.buses.peek(pointer);
            let addr_high_byte = nes.buses.peek(pointer + 1);
            let addr = concat_u8!(addr_high_byte, addr_low_byte);

            create_indirect_debug_text(label, pointer, addr)
        }
        AddressingMode::AbsoluteIndexedX => {
            let (pch, pcl) = nes.cpu.get_registers().pc;
            let pc = concat_u8!(pch, pcl);

            let op1 = nes.buses.peek(pc + 1);
            let op2 = nes.buses.peek(pc + 2);
            let operand = concat_u8!(op2, op1);

            let x_contents = nes.cpu.get_registers().x_index;
            let addr = operand + x_contents as u16;
            let value = nes.buses.peek(addr);

            create_absolute_indexed_x_debug_text(label, operand, x_contents, value)
        }
        AddressingMode::AbsoluteIndexedY => {
            let (pch, pcl) = nes.cpu.get_registers().pc;
            let pc = concat_u8!(pch, pcl);

            let op1 = nes.buses.peek(pc + 1);
            let op2 = nes.buses.peek(pc + 2);
            let operand = concat_u8!(op2, op1);

            let y_contents = nes.cpu.get_registers().y_index;
            let addr = operand + y_contents as u16;
            let value = nes.buses.peek(addr);

            create_absolute_indexed_y_debug_text(label, operand, y_contents, value)
        }
        AddressingMode::ZeroPageIndexedX => {
            let (pch, pcl) = nes.cpu.get_registers().pc;
            let pc = concat_u8!(pch, pcl);

            let op = nes.buses.peek(pc + 1);

            let x_contents = nes.cpu.get_registers().x_index;
            let zero_page_addr = concat_u8!(0x00, op);
            let addr = zero_page_addr + x_contents as u16;
            let value = nes.buses.peek(addr);

            create_zero_page_indexed_x_debug_text(label, op, x_contents, value)
        }
        AddressingMode::ZeroPageIndexedY => {
            let (pch, pcl) = nes.cpu.get_registers().pc;
            let pc = concat_u8!(pch, pcl);

            let op = nes.buses.peek(pc + 1);

            let y_contents = nes.cpu.get_registers().y_index;
            let zero_page_addr = concat_u8!(0x00, op);
            let addr = zero_page_addr + y_contents as u16;
            let value = nes.buses.peek(addr);

            create_zero_page_indexed_y_debug_text(label, op, y_contents, value)
        }
        AddressingMode::IndirectIndexedX => {
            let (pch, pcl) = nes.cpu.get_registers().pc;
            let pc = concat_u8!(pch, pcl);

            let op = nes.buses.peek(pc + 1);

            let zero_page_address = concat_u8!(0x00, op);
            let pointer_high_byte = nes.buses.peek(zero_page_address);
            let pointer_low_byte = nes.buses.peek(zero_page_address + 1);
            let pointer = concat_u8!(pointer_high_byte, pointer_low_byte);

            let x_contents = nes.cpu.get_registers().x_index;
            let addr = pointer + x_contents as u16;
            let value = nes.buses.peek(addr);

            create_indirect_indexed_x_debug_text(label, op, pointer, x_contents, value)
        }
        AddressingMode::IndirectIndexedY => {
            let (pch, pcl) = nes.cpu.get_registers().pc;
            let pc = concat_u8!(pch, pcl);

            let op = nes.buses.peek(pc + 1);

            let zero_page_address = concat_u8!(0x00, op);
            let pointer_high_byte = nes.buses.peek(zero_page_address);
            let pointer_low_byte = nes.buses.peek(zero_page_address + 1);
            let pointer = concat_u8!(pointer_high_byte, pointer_low_byte);

            let y_contents = nes.cpu.get_registers().y_index;
            let addr = pointer + y_contents as u16;
            let value = nes.buses.peek(addr);

            create_indirect_indexed_y_debug_text(label, op, pointer, y_contents, value)
        }
    }
}

/// Creates the debug text for the Relative addressing mode.
///
/// # Arguments
///
/// * `label`: The label for the instruction being debugged.
/// * `operand`: The amount by which the program counter will be offset.
/// * `program_counter`: The program counter to offset.
///
/// # Examples
///
/// ```Rust
/// create_relative_debug_text("BEQ", 0x09, 0xC997) // => "BEQ #09 = &C9A1"
/// ```
///
fn create_relative_debug_text(label: &str, operand: u8, program_counter: u16) -> String {
    let relative_addr = program_counter.wrapping_add_signed(operand as i16);
    format!("{label} #{operand:02X} = &{relative_addr:04X}")
}

/// Creates the debug text for the Immediate addressing mode.
///
/// # Arguments
///
/// * `label`: The label for the instruction being debugged.
/// * `operand`: The value used in the instruction.
///
/// # Examples
///
/// ```Rust
/// create_immediate_debug_text("ADC", 0x69) // => "ADC #69"
/// ```
///
fn create_immediate_debug_text(label: &str, operand: u8) -> String {
    format!("{label} #{operand:02X}")
}

/// Creates the debug text for the Absolute addressing mode.
///
/// # Arguments
///
/// * `label`: The label for the instruction being debugged.
/// * `address`: The memory location accessed by the instruction.
/// * `value`: The value stored at the above address.
///
/// # Examples
///
/// ```Rust
/// create_absolute_debug_text("ADC", 0x0678, 0x69) // => "ADC &0678 -> #69"
/// ```
///
fn create_absolute_debug_text(label: &str, address: u16, value: u8) -> String {
    format!("{label} &{address:04X} -> #{value:02X}")
}

/// Creates the debug text for the Zero Page addressing mode.
///
/// # Arguments
///
/// * `label`: The label for the instruction being debugged.
/// * `operand`: The low byte for the zero page address accessed by the instruction.
/// * `value`: The value stored at the zero page address accessed by the instruction.
///
/// # Examples
///
/// ```Rust
/// create_zero_page_debug_text("ADC", 0x78, 0x69) // => "ADC #78 = &0078 -> #69"
/// ```
///
fn create_zero_page_debug_text(label: &str, operand: u8, value: u8) -> String {
    let zero_page_addr = concat_u8!(0x00, operand);
    format!("{label} #{operand:02X} = &{zero_page_addr:04X} -> #{value:02X}")
}

/// Creates the debug text for the Indirect addressing mode.
///
/// # Arguments
///
/// * `label`: The label for the instruction being debugged.
/// * `pointer`: The memory location accessed by the instruction.
/// * `address`: The value stored at the above pointer address.
///
/// # Examples
///
/// ```Rust
/// create_indirect_debug_text("JMP", 0x0200, 0xDB7E) // => "JMP &0200 -> &DB7E"
/// ```
///
fn create_indirect_debug_text(label: &str, pointer: u16, address: u16) -> String {
    format!("{label} &{pointer:04X} -> &{address:04X}")
}

/// Creates the debug text for the Absolute Indexed X addressing mode.
///
/// # Arguments
///
/// * `label`: The label for the instruction being debugged.
/// * `operand`: The value used in the instruction.
/// * `x_contents`: The contents of the X index register.
/// * `value`: The value stored at the memory location accessed by the instruction.
///
/// # Examples
///
/// ```Rust
/// create_absolute_indexed_x_debug_text("ADC", 0x0600, 0x78, 0x69) // => "ADC (#0600, X) = &0678 -> #69"
/// ```
///
fn create_absolute_indexed_x_debug_text(
    label: &str,
    operand: u16,
    x_contents: u8,
    value: u8,
) -> String {
    let addr = operand + x_contents as u16;
    format!("{label} (#{operand:04X}, X) = &{addr} -> #{value:02X}")
}

/// Creates the debug text for the Absolute Indexed Y addressing mode.
///
/// # Arguments
///
/// * `label`: The label for the instruction being debugged.
/// * `operand`: The value used in the instruction.
/// * `y_contents`: The contents of the Y index register.
/// * `value`: The value stored at the memory location accessed by the instruction.
///
/// # Examples
///
/// ```Rust
/// create_absolute_indexed_x_debug_text("ADC", 0x0600, 0x78, 0x69) // => "ADC (#0600, Y) = &0678 -> #69"
/// ```
///
fn create_absolute_indexed_y_debug_text(
    label: &str,
    operand: u16,
    y_contents: u8,
    value: u8,
) -> String {
    let addr = operand + y_contents as u16;
    format!("{label} (#{operand:04X}, Y) = &{addr} -> #{value:02X}")
}

/// Creates the debug text for the Zero Page Indexed X addressing mode.
///
/// # Arguments
///
/// * `label`: The label for the instruction being debugged.
/// * `operand`: The value used in the instruction.
/// * `x_contents`: The contents of the X index register.
/// * `value`: The value stored at the memory location accessed by the instruction.
///
/// # Examples
///
/// ```Rust
/// create_zero_page_indexed_x_debug_text("ADC", 0x80, 0x02, 0x69) // => "ADC (#80 = &0080, X) = &0082 -> #69"
/// ```
///
fn create_zero_page_indexed_x_debug_text(
    label: &str,
    operand: u8,
    x_contents: u8,
    value: u8,
) -> String {
    let zero_page_addr = concat_u8!(0x00, operand);
    let addr = zero_page_addr + x_contents as u16;
    format!("{label} (#{operand:02X} = &{zero_page_addr:04X}, X) = &{addr} -> #{value:02X}")
}

/// Creates the debug text for the Zero Page Indexed X addressing mode.
///
/// # Arguments
///
/// * `label`: The label for the instruction being debugged.
/// * `operand`: The value used in the instruction.
/// * `y_contents`: The contents of the Y index register.
/// * `value`: The value stored at the memory location accessed by the instruction.
///
/// # Examples
///
/// ```Rust
/// // Returns: "ADC (#80 = &0080, Y) = &0082 -> #69"
/// create_zero_page_indexed_y_debug_text("ADC", 0x80, 0x02, 0x69)
/// ```
fn create_zero_page_indexed_y_debug_text(
    label: &str,
    operand: u8,
    y_contents: u8,
    value: u8,
) -> String {
    let zero_page_addr = concat_u8!(0x00, operand);
    let addr = zero_page_addr + y_contents as u16;
    format!("{label} (#{operand:02X} = &{zero_page_addr:04X}, X) = &{addr} -> #{value:02X}")
}

/// Creates the debug text for the Indirect Indexed X addressing mode.
///
/// # Arguments
///
/// * `label`: The label for the instruction being debugged.
/// * `operand`: The value used in the instruction.
/// * `pointer`: The value stored at the zero page pointer address.
/// * `x_contents`: The contents of the X index register.
/// * `value`: The value stored at the above address.
/// *
///
/// # Examples
///
/// ```Rust
/// // Returns: "LDA (#52 = &0052 -> &EF05, X) = &EF08 -> #3A"
/// create_indirect_indexed_x_debug_text("LDA", 0x52, 0xEF05, 0x03, 0x3A)
/// ```
///
fn create_indirect_indexed_x_debug_text(
    label: &str,
    operand: u8,
    pointer: u16,
    x_contents: u8,
    value: u8,
) -> String {
    let zero_page_addr = concat_u8!(0x00, operand);
    let addr = pointer + x_contents as u16;
    format!(
        "{label}, (#{operand:02X} = &{zero_page_addr:04X} -> {pointer:04X}, X) = &{addr:04X} -> #{value:02X}"
    )
}

/// Creates the debug text for the Indirect Indexed Y addressing mode.
///
/// # Arguments
///
/// * `label`: The label for the instruction being debugged.
/// * `operand`: The value used in the instruction.
/// * `pointer`: The value stored at the zero page pointer address.
/// * `y_contents`: The contents of the Y index register.
/// * `value`: The value stored at the above address.
/// *
///
/// # Examples
///
/// ```Rust
/// // Returns: "LDA (#52 = &0052 -> &EF05, Y) = &EF08 -> #3A"
/// create_indirect_indexed_y_debug_text("LDA", 0x52, 0xEF05, 0x03, 0x3A)
/// ```
///
fn create_indirect_indexed_y_debug_text(
    label: &str,
    operand: u8,
    pointer: u16,
    y_contents: u8,
    value: u8,
) -> String {
    let zero_page_addr = concat_u8!(0x00, operand);
    let addr = pointer + y_contents as u16;
    format!(
        "{label}, (#{operand:02X} = &{zero_page_addr:04X} -> {pointer:04X}, X) = &{addr:04X} -> #{value:02X}"
    )
}

/// Returns the label that corresponds to the given opcode.
fn get_label(opcode: u8) -> &'static str {
    match opcode {
        0x00 => "BRK",
        0x01 => "ORA",
        0x02 => "JAM",
        0x03 => "SLO",
        0x04 => "NOP",
        0x05 => "ORA",
        0x06 => "ASL",
        0x07 => "SLO",
        0x08 => "PHP",
        0x09 => "ORA",
        0x0A => "ASL",
        0x0B => todo!("Opcode 0x{opcode:02X} not implemented"),
        0x0C => "NOP",
        0x0D => "ORA",
        0x0E => "ASL",
        0x0F => "SLO",
        0x10 => "BPL",
        0x11 => "ORA",
        0x12 => "JAM",
        0x13 => "SLO",
        0x14 => "NOP",
        0x15 => "ORA",
        0x16 => "ASL",
        0x17 => "SLO",
        0x18 => "CLC",
        0x19 => "ORA",
        0x1A => "NOP",
        0x1B => "SLO",
        0x1C => "NOP",
        0x1D => "ORA",
        0x1E => "ASL",
        0x1F => "SLO",
        0x20 => "JSR",
        0x21 => "AND",
        0x22 => "JAM",
        0x23 => "RLA",
        0x24 => "BIT",
        0x25 => "AND",
        0x26 => "ROL",
        0x27 => "RLA",
        0x28 => "PLP",
        0x29 => "AND",
        0x2A => "ROL",
        0x2B => todo!("Opcode 0x{opcode:02X} not implemented"),
        0x2C => "BIT",
        0x2D => "AND",
        0x2E => "ROL",
        0x2F => "RLA",
        0x30 => "BMI",
        0x31 => "AND",
        0x32 => "JAM",
        0x33 => "RLA",
        0x34 => "NOP",
        0x35 => "AND",
        0x36 => "ROL",
        0x37 => "RLA",
        0x38 => "SEC",
        0x39 => "AND",
        0x3A => "NOP",
        0x3B => "RLA",
        0x3C => "NOP",
        0x3D => "AND",
        0x3E => "ROL",
        0x3F => "RLA",
        0x40 => "RTI",
        0x41 => "EOR",
        0x42 => "JAM",
        0x43 => "SRE",
        0x44 => "NOP",
        0x45 => "EOR",
        0x46 => "LSR",
        0x47 => "SRE",
        0x48 => "PHA",
        0x49 => "EOR",
        0x4A => "LSR",
        0x4B => todo!("Opcode 0x{opcode:02X} not implemented"),
        0x4C => "JMP",
        0x4D => "EOR",
        0x4E => "LSR",
        0x4F => "SRE",
        0x50 => "BVC",
        0x51 => "EOR",
        0x52 => "JAM",
        0x53 => "SRE",
        0x54 => "NOP",
        0x55 => "EOR",
        0x56 => "LSR",
        0x57 => "SRE",
        0x58 => "CLI",
        0x59 => "EOR",
        0x5A => "NOP",
        0x5B => "SRE",
        0x5C => "NOP",
        0x5D => "EOR",
        0x5E => "LSR",
        0x5F => "SRE",
        0x60 => "RTS",
        0x61 => "ADC",
        0x62 => "JAM",
        0x63 => "RRA",
        0x64 => "NOP",
        0x65 => "ADC",
        0x66 => "ROR",
        0x67 => "RRA",
        0x68 => "PLA",
        0x69 => "ADC",
        0x6A => "ROR",
        0x6B => todo!("Opcode 0x{opcode:02X} not implemented"),
        0x6C => "JMP",
        0x6D => "ADC",
        0x6E => "ROR",
        0x6F => "RRA",
        0x70 => "BVS",
        0x71 => "ADC",
        0x72 => "JAM",
        0x73 => "RRA",
        0x74 => "NOP",
        0x75 => "ADC",
        0x76 => "ROR",
        0x77 => "RRA",
        0x78 => "SEI",
        0x79 => "ADC",
        0x7A => "NOP",
        0x7B => "RRA",
        0x7C => "NOP",
        0x7D => "ADC",
        0x7E => "ROR",
        0x7F => "RRA",
        0x80 => "NOP",
        0x81 => "STA",
        0x82 => "NOP",
        0x83 => "SAX",
        0x84 => "STY",
        0x85 => "STA",
        0x86 => "STX",
        0x87 => "SAX",
        0x88 => "DEY",
        0x89 => "NOP",
        0x8A => "TXA",
        0x8B => todo!("Opcode 0x{opcode:02X} not implemented"),
        0x8C => "STY",
        0x8D => "STA",
        0x8E => "STX",
        0x8F => "SAX",
        0x90 => "BCC",
        0x91 => "STA",
        0x92 => "JAM",
        0x93 => todo!("Opcode 0x{opcode:02X} not implemented"),
        0x94 => "STY",
        0x95 => "STA",
        0x96 => "STX",
        0x97 => "SAX",
        0x98 => "TYA",
        0x99 => "STA",
        0x9A => "TXS",
        0x9B => todo!("Opcode 0x{opcode:02X} not implemented"),
        0x9C => todo!("Opcode 0x{opcode:02X} not implemented"),
        0x9D => "STA",
        0x9E => todo!("Opcode 0x{opcode:02X} not implemented"),
        0x9F => todo!("Opcode 0x{opcode:02X} not implemented"),
        0xA0 => "LDY",
        0xA1 => "LDA",
        0xA2 => "LDX",
        0xA3 => "LAX",
        0xA4 => "LDY",
        0xA5 => "LDA",
        0xA6 => "LDX",
        0xA7 => "LAX",
        0xA8 => "TAY",
        0xA9 => "LDA",
        0xAA => "TAX",
        0xAB => todo!("Opcode 0x{opcode:02X} not implemented"),
        0xAC => "LDY",
        0xAD => "LDA",
        0xAE => "LDX",
        0xAF => "LAX",
        0xB0 => "BCS",
        0xB1 => "LDA",
        0xB2 => "JAM",
        0xB3 => "LAX",
        0xB4 => "LDY",
        0xB5 => "LDA",
        0xB6 => "LDX",
        0xB7 => "LAX",
        0xB8 => "CLV",
        0xB9 => "LDA",
        0xBA => "TSX",
        0xBB => todo!("Opcode 0x{opcode:02X} not implemented"),
        0xBC => "LDY",
        0xBD => "LDA",
        0xBE => "LDX",
        0xBF => "LAX",
        0xC0 => "CPY",
        0xC1 => "CMP",
        0xC2 => "NOP",
        0xC3 => "DCP",
        0xC4 => "CPY",
        0xC5 => "CMP",
        0xC6 => "DEC",
        0xC7 => "DCP",
        0xC8 => "INY",
        0xC9 => "CMP",
        0xCA => "DEX",
        0xCB => todo!("Opcode 0x{opcode:02X} not implemented"),
        0xCC => "CPY",
        0xCD => "CMP",
        0xCE => "DEC",
        0xCF => "DCP",
        0xD0 => "BNE",
        0xD1 => "CMP",
        0xD2 => "JAM",
        0xD3 => "DCP",
        0xD4 => "NOP",
        0xD5 => "CMP",
        0xD6 => "DEC",
        0xD7 => "DCP",
        0xD8 => "CLD",
        0xD9 => "CMP",
        0xDA => "NOP",
        0xDB => "DCP",
        0xDC => "NOP",
        0xDD => "CMP",
        0xDE => "DEC",
        0xDF => "DCP",
        0xE0 => "CPX",
        0xE1 => "SBC",
        0xE2 => "NOP",
        0xE3 => "ISC",
        0xE4 => "CPX",
        0xE5 => "SBC",
        0xE6 => "INC",
        0xE7 => "ISC",
        0xE8 => "INX",
        0xE9 => "SBC",
        0xEA => "NOP",
        0xEB => "USBC",
        0xEC => "CPX",
        0xED => "SBC",
        0xEE => "INC",
        0xEF => "ISC",
        0xF0 => "BEQ",
        0xF1 => "SBC",
        0xF2 => "JAM",
        0xF3 => "ISC",
        0xF4 => "NOP",
        0xF5 => "SBC",
        0xF6 => "INC",
        0xF7 => "ISC",
        0xF8 => "SED",
        0xF9 => "SBC",
        0xFA => "NOP",
        0xFB => "ISC",
        0xFC => "NOP",
        0xFD => "SBC",
        0xFE => "INC",
        0xFF => "ISC",
    }
}

/// Returns the addressing mode that corresponds to the given opcode.
fn get_addressing_mode(opcode: u8) -> AddressingMode {
    match opcode {
        0x00 => AddressingMode::Implied,
        0x01 => AddressingMode::IndirectIndexedX,
        0x02 => AddressingMode::Implied,
        0x03 => AddressingMode::IndirectIndexedX,
        0x04 => AddressingMode::ZeroPage,
        0x05 => AddressingMode::ZeroPage,
        0x06 => AddressingMode::ZeroPage,
        0x07 => AddressingMode::ZeroPage,
        0x08 => AddressingMode::Implied,
        0x09 => AddressingMode::Immediate,
        0x0A => AddressingMode::Accumulator,
        0x0B => todo!("Opcode 0x{opcode:02X} not implemented"),
        0x0C => AddressingMode::Absolute,
        0x0D => AddressingMode::Absolute,
        0x0E => AddressingMode::Absolute,
        0x0F => AddressingMode::Absolute,
        0x10 => AddressingMode::Relative,
        0x11 => AddressingMode::IndirectIndexedY,
        0x12 => AddressingMode::Implied,
        0x13 => AddressingMode::IndirectIndexedY,
        0x14 => AddressingMode::ZeroPageIndexedX,
        0x15 => AddressingMode::ZeroPageIndexedX,
        0x16 => AddressingMode::ZeroPageIndexedX,
        0x17 => AddressingMode::ZeroPageIndexedX,
        0x18 => AddressingMode::Implied,
        0x19 => AddressingMode::AbsoluteIndexedY,
        0x1A => AddressingMode::Implied,
        0x1B => AddressingMode::AbsoluteIndexedY,
        0x1C => AddressingMode::AbsoluteIndexedX,
        0x1D => AddressingMode::AbsoluteIndexedX,
        0x1E => AddressingMode::AbsoluteIndexedX,
        0x1F => AddressingMode::AbsoluteIndexedX,
        0x20 => AddressingMode::Absolute,
        0x21 => AddressingMode::IndirectIndexedX,
        0x22 => AddressingMode::Implied,
        0x23 => AddressingMode::IndirectIndexedX,
        0x24 => AddressingMode::ZeroPage,
        0x25 => AddressingMode::ZeroPage,
        0x26 => AddressingMode::ZeroPage,
        0x27 => AddressingMode::ZeroPage,
        0x28 => AddressingMode::Implied,
        0x29 => AddressingMode::Immediate,
        0x2A => AddressingMode::Accumulator,
        0x2B => todo!("Opcode 0x{opcode:02X} not implemented"),
        0x2C => AddressingMode::Absolute,
        0x2D => AddressingMode::Absolute,
        0x2E => AddressingMode::Absolute,
        0x2F => AddressingMode::Absolute,
        0x30 => AddressingMode::Relative,
        0x31 => AddressingMode::IndirectIndexedY,
        0x32 => AddressingMode::Implied,
        0x33 => AddressingMode::IndirectIndexedY,
        0x34 => AddressingMode::ZeroPageIndexedX,
        0x35 => AddressingMode::ZeroPageIndexedX,
        0x36 => AddressingMode::ZeroPageIndexedX,
        0x37 => AddressingMode::ZeroPageIndexedX,
        0x38 => AddressingMode::Implied,
        0x39 => AddressingMode::AbsoluteIndexedY,
        0x3A => AddressingMode::Implied,
        0x3B => AddressingMode::AbsoluteIndexedY,
        0x3C => AddressingMode::AbsoluteIndexedX,
        0x3D => AddressingMode::AbsoluteIndexedX,
        0x3E => AddressingMode::AbsoluteIndexedX,
        0x3F => AddressingMode::AbsoluteIndexedX,
        0x40 => AddressingMode::Implied,
        0x41 => AddressingMode::IndirectIndexedX,
        0x42 => AddressingMode::Implied,
        0x43 => AddressingMode::IndirectIndexedX,
        0x44 => AddressingMode::ZeroPage,
        0x45 => AddressingMode::ZeroPage,
        0x46 => AddressingMode::ZeroPage,
        0x47 => AddressingMode::ZeroPage,
        0x48 => AddressingMode::Implied,
        0x49 => AddressingMode::Immediate,
        0x4A => AddressingMode::Accumulator,
        0x4B => todo!("Opcode 0x{opcode:02X} not implemented"),
        0x4C => AddressingMode::Absolute,
        0x4D => AddressingMode::Absolute,
        0x4E => AddressingMode::Absolute,
        0x4F => AddressingMode::Absolute,
        0x50 => AddressingMode::Relative,
        0x51 => AddressingMode::IndirectIndexedY,
        0x52 => AddressingMode::Implied,
        0x53 => AddressingMode::IndirectIndexedY,
        0x54 => AddressingMode::ZeroPageIndexedX,
        0x55 => AddressingMode::ZeroPageIndexedX,
        0x56 => AddressingMode::ZeroPageIndexedX,
        0x57 => AddressingMode::ZeroPageIndexedX,
        0x58 => AddressingMode::Implied,
        0x59 => AddressingMode::AbsoluteIndexedY,
        0x5A => AddressingMode::Implied,
        0x5B => AddressingMode::AbsoluteIndexedY,
        0x5C => AddressingMode::AbsoluteIndexedX,
        0x5D => AddressingMode::AbsoluteIndexedX,
        0x5E => AddressingMode::AbsoluteIndexedX,
        0x5F => AddressingMode::AbsoluteIndexedX,
        0x60 => AddressingMode::Implied,
        0x61 => AddressingMode::IndirectIndexedX,
        0x62 => AddressingMode::Implied,
        0x63 => AddressingMode::IndirectIndexedX,
        0x64 => AddressingMode::ZeroPage,
        0x65 => AddressingMode::ZeroPage,
        0x66 => AddressingMode::ZeroPage,
        0x67 => AddressingMode::ZeroPage,
        0x68 => AddressingMode::Implied,
        0x69 => AddressingMode::Immediate,
        0x6A => AddressingMode::Accumulator,
        0x6B => todo!("Opcode 0x{opcode:02X} not implemented"),
        0x6C => AddressingMode::Indirect,
        0x6D => AddressingMode::Absolute,
        0x6E => AddressingMode::Absolute,
        0x6F => AddressingMode::Absolute,
        0x70 => AddressingMode::Relative,
        0x71 => AddressingMode::IndirectIndexedY,
        0x72 => AddressingMode::Implied,
        0x73 => AddressingMode::IndirectIndexedY,
        0x74 => AddressingMode::ZeroPageIndexedX,
        0x75 => AddressingMode::ZeroPageIndexedX,
        0x76 => AddressingMode::ZeroPageIndexedX,
        0x77 => AddressingMode::ZeroPageIndexedX,
        0x78 => AddressingMode::Implied,
        0x79 => AddressingMode::AbsoluteIndexedY,
        0x7A => AddressingMode::Implied,
        0x7B => AddressingMode::AbsoluteIndexedY,
        0x7C => AddressingMode::AbsoluteIndexedX,
        0x7D => AddressingMode::AbsoluteIndexedX,
        0x7E => AddressingMode::AbsoluteIndexedX,
        0x7F => AddressingMode::AbsoluteIndexedX,
        0x80 => AddressingMode::Immediate,
        0x81 => AddressingMode::IndirectIndexedX,
        0x82 => AddressingMode::Immediate,
        0x83 => AddressingMode::IndirectIndexedX,
        0x84 => AddressingMode::ZeroPage,
        0x85 => AddressingMode::ZeroPage,
        0x86 => AddressingMode::ZeroPage,
        0x87 => AddressingMode::ZeroPage,
        0x88 => AddressingMode::Implied,
        0x89 => AddressingMode::Immediate,
        0x8A => AddressingMode::Implied,
        0x8B => todo!("Opcode 0x{opcode:02X} not implemented"),
        0x8C => AddressingMode::Absolute,
        0x8D => AddressingMode::Absolute,
        0x8E => AddressingMode::Absolute,
        0x8F => AddressingMode::Absolute,
        0x90 => AddressingMode::Relative,
        0x91 => AddressingMode::IndirectIndexedY,
        0x92 => AddressingMode::Implied,
        0x93 => todo!("Opcode 0x{opcode:02X} not implemented"),
        0x94 => AddressingMode::ZeroPageIndexedX,
        0x95 => AddressingMode::ZeroPageIndexedX,
        0x96 => AddressingMode::ZeroPageIndexedY,
        0x97 => AddressingMode::ZeroPageIndexedY,
        0x98 => AddressingMode::Implied,
        0x99 => AddressingMode::AbsoluteIndexedY,
        0x9A => AddressingMode::Implied,
        0x9B => todo!("Opcode 0x{opcode:02X} not implemented"),
        0x9C => todo!("Opcode 0x{opcode:02X} not implemented"),
        0x9D => AddressingMode::AbsoluteIndexedX,
        0x9E => todo!("Opcode 0x{opcode:02X} not implemented"),
        0x9F => todo!("Opcode 0x{opcode:02X} not implemented"),
        0xA0 => AddressingMode::Immediate,
        0xA1 => AddressingMode::IndirectIndexedX,
        0xA2 => AddressingMode::Immediate,
        0xA3 => AddressingMode::IndirectIndexedX,
        0xA4 => AddressingMode::ZeroPage,
        0xA5 => AddressingMode::ZeroPage,
        0xA6 => AddressingMode::ZeroPage,
        0xA7 => AddressingMode::ZeroPage,
        0xA8 => AddressingMode::Implied,
        0xA9 => AddressingMode::Immediate,
        0xAA => AddressingMode::Implied,
        0xAB => todo!("Opcode 0x{opcode:02X} not implemented"),
        0xAC => AddressingMode::Absolute,
        0xAD => AddressingMode::Absolute,
        0xAE => AddressingMode::Absolute,
        0xAF => AddressingMode::Absolute,
        0xB0 => AddressingMode::Relative,
        0xB1 => AddressingMode::IndirectIndexedY,
        0xB2 => AddressingMode::Implied,
        0xB3 => AddressingMode::IndirectIndexedY,
        0xB4 => AddressingMode::ZeroPageIndexedX,
        0xB5 => AddressingMode::ZeroPageIndexedX,
        0xB6 => AddressingMode::ZeroPageIndexedY,
        0xB7 => AddressingMode::ZeroPageIndexedY,
        0xB8 => AddressingMode::Implied,
        0xB9 => AddressingMode::AbsoluteIndexedY,
        0xBA => AddressingMode::Implied,
        0xBB => todo!("Opcode 0x{opcode:02X} not implemented"),
        0xBC => AddressingMode::AbsoluteIndexedX,
        0xBD => AddressingMode::AbsoluteIndexedX,
        0xBE => AddressingMode::AbsoluteIndexedY,
        0xBF => AddressingMode::AbsoluteIndexedY,
        0xC0 => AddressingMode::Immediate,
        0xC1 => AddressingMode::IndirectIndexedX,
        0xC2 => AddressingMode::Immediate,
        0xC3 => AddressingMode::IndirectIndexedX,
        0xC4 => AddressingMode::ZeroPage,
        0xC5 => AddressingMode::ZeroPage,
        0xC6 => AddressingMode::ZeroPage,
        0xC7 => AddressingMode::ZeroPage,
        0xC8 => AddressingMode::Implied,
        0xC9 => AddressingMode::Immediate,
        0xCA => AddressingMode::Implied,
        0xCB => todo!("Opcode 0x{opcode:02X} not implemented"),
        0xCC => AddressingMode::Absolute,
        0xCD => AddressingMode::Absolute,
        0xCE => AddressingMode::Absolute,
        0xCF => AddressingMode::Absolute,
        0xD0 => AddressingMode::Relative,
        0xD1 => AddressingMode::IndirectIndexedY,
        0xD2 => AddressingMode::Implied,
        0xD3 => AddressingMode::IndirectIndexedY,
        0xD4 => AddressingMode::ZeroPageIndexedX,
        0xD5 => AddressingMode::ZeroPageIndexedX,
        0xD6 => AddressingMode::ZeroPageIndexedX,
        0xD7 => AddressingMode::ZeroPageIndexedX,
        0xD8 => AddressingMode::Implied,
        0xD9 => AddressingMode::AbsoluteIndexedY,
        0xDA => AddressingMode::Implied,
        0xDB => AddressingMode::AbsoluteIndexedY,
        0xDC => AddressingMode::AbsoluteIndexedX,
        0xDD => AddressingMode::AbsoluteIndexedX,
        0xDE => AddressingMode::AbsoluteIndexedX,
        0xDF => AddressingMode::AbsoluteIndexedX,
        0xE0 => AddressingMode::Immediate,
        0xE1 => AddressingMode::IndirectIndexedX,
        0xE2 => AddressingMode::Immediate,
        0xE3 => AddressingMode::IndirectIndexedX,
        0xE4 => AddressingMode::ZeroPage,
        0xE5 => AddressingMode::ZeroPage,
        0xE6 => AddressingMode::ZeroPage,
        0xE7 => AddressingMode::ZeroPage,
        0xE8 => AddressingMode::Implied,
        0xE9 => AddressingMode::Immediate,
        0xEA => AddressingMode::Implied,
        0xEB => AddressingMode::Immediate,
        0xEC => AddressingMode::Absolute,
        0xED => AddressingMode::Absolute,
        0xEE => AddressingMode::Absolute,
        0xEF => AddressingMode::Absolute,
        0xF0 => AddressingMode::Relative,
        0xF1 => AddressingMode::IndirectIndexedY,
        0xF2 => AddressingMode::Implied,
        0xF3 => AddressingMode::IndirectIndexedY,
        0xF4 => AddressingMode::ZeroPageIndexedX,
        0xF5 => AddressingMode::ZeroPageIndexedX,
        0xF6 => AddressingMode::ZeroPageIndexedX,
        0xF7 => AddressingMode::ZeroPageIndexedX,
        0xF8 => AddressingMode::Implied,
        0xF9 => AddressingMode::AbsoluteIndexedY,
        0xFA => AddressingMode::Implied,
        0xFB => AddressingMode::AbsoluteIndexedY,
        0xFC => AddressingMode::AbsoluteIndexedX,
        0xFD => AddressingMode::AbsoluteIndexedX,
        0xFE => AddressingMode::AbsoluteIndexedX,
        0xFF => AddressingMode::AbsoluteIndexedX,
    }
}
