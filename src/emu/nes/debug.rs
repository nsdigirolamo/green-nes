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
}

pub fn get_debug_text(nes: NES) -> String {
    let (pch, pcl) = nes.cpu.get_registers().pc;
    let pc = concat_u8!(pch, pcl);
    let opcode = nes.buses.peek(pc);

    todo!("debug text not implemented: 0x{opcode:02X}")
}

pub fn get_instruction_text(nes: NES, label: &str, addressing_mode: AddressingMode) -> String {
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
/// create_zero_page_indexed_y_debug_text("ADC", 0x80, 0x02, 0x69) // => "ADC (#80 = &0080, Y) = &0082 -> #69"
/// ```
///
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
