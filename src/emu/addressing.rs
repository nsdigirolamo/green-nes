use crate::{
    concat_u8, cycle,
    emu::state::{Cycle, Instruction, State, StateOperation},
};

/*
 * A lot of the below code contains reads/writes to memory that exist only to
 * ensure each instruction is "cycle-accurate." The writes to memory actually
 * occur, but the memory reads don't ever actually influence any other part of
 * the machine state. For example, these "redundant" reads don't load the memory
 * value into the Accumulator when the operation otherwise would.
 *
 * TODO: Do these "redundant" reads need to actually store the read value
 * anywhere? Or is it enough to simply read the memory location and make sure
 * the read is "visible" on the bus?
 */

pub enum AbsoluteAddressing {
    Jump,
    Read,
    ReadModifyWrite,
    Write,
}

impl Instruction for AbsoluteAddressing {
    fn get_cycles(&self, operation: StateOperation) -> Vec<Cycle> {
        match self {
            AbsoluteAddressing::Jump => {
                vec![cycle![fetch_low_effective_address_byte], cycle![operation]]
            }
            AbsoluteAddressing::Read => vec![
                cycle![fetch_low_effective_address_byte],
                cycle![fetch_high_effective_address_byte],
                cycle![read_from_effective_address, operation],
            ],
            AbsoluteAddressing::ReadModifyWrite => vec![
                cycle![fetch_low_effective_address_byte],
                cycle![fetch_high_effective_address_byte],
                cycle![read_from_effective_address],
                cycle![operation],
                cycle![write_to_effective_address],
            ],
            AbsoluteAddressing::Write => vec![
                cycle![fetch_low_effective_address_byte],
                cycle![fetch_high_effective_address_byte],
                cycle![operation, write_to_effective_address],
            ],
        }
    }
}

pub enum ZeroPageAddressing {
    Read,
    ReadModifyWrite,
    Write,
}

impl Instruction for ZeroPageAddressing {
    fn get_cycles(&self, operation: StateOperation) -> Vec<Cycle> {
        match self {
            ZeroPageAddressing::Read => {
                vec![
                    cycle![fetch_effective_zero_page_address],
                    cycle![read_from_effective_address, operation],
                ]
            }
            ZeroPageAddressing::ReadModifyWrite => vec![
                cycle![fetch_effective_zero_page_address],
                cycle![read_from_effective_address],
                cycle![operation],
                cycle![write_to_effective_address],
            ],
            ZeroPageAddressing::Write => {
                vec![
                    cycle![fetch_effective_zero_page_address],
                    cycle![operation, write_to_effective_address],
                ]
            }
        }
    }
}

pub enum ZeroPageXIndexedAddressing {
    Read,
    ReadModifyWrite,
    Write,
}

impl Instruction for ZeroPageXIndexedAddressing {
    fn get_cycles(&self, operation: StateOperation) -> Vec<Cycle> {
        match self {
            ZeroPageXIndexedAddressing::Read => vec![
                cycle![fetch_effective_zero_page_address],
                cycle![do_effective_zero_page_address_x_index],
                cycle![read_from_effective_address, operation],
            ],
            ZeroPageXIndexedAddressing::ReadModifyWrite => vec![
                cycle![fetch_effective_zero_page_address],
                cycle![do_effective_zero_page_address_x_index],
                cycle![read_from_effective_address],
                cycle![operation],
                cycle![write_to_effective_address],
            ],
            ZeroPageXIndexedAddressing::Write => vec![
                cycle![fetch_effective_zero_page_address],
                cycle![do_effective_zero_page_address_x_index],
                cycle![operation, write_to_effective_address],
            ],
        }
    }
}

pub enum ZeroPageYIndexedAddressing {
    Read,
    ReadModifyWrite,
    Write,
}

impl Instruction for ZeroPageYIndexedAddressing {
    fn get_cycles(&self, operation: StateOperation) -> Vec<Cycle> {
        match self {
            ZeroPageYIndexedAddressing::Read => vec![
                cycle![fetch_effective_zero_page_address],
                cycle![do_effective_zero_page_address_y_index],
                cycle![read_from_effective_address, operation],
            ],
            ZeroPageYIndexedAddressing::ReadModifyWrite => vec![
                cycle![fetch_effective_zero_page_address],
                cycle![do_effective_zero_page_address_y_index],
                cycle![read_from_effective_address],
                cycle![operation],
                cycle![write_to_effective_address],
            ],
            ZeroPageYIndexedAddressing::Write => vec![
                cycle![fetch_effective_zero_page_address],
                cycle![do_effective_zero_page_address_y_index],
                cycle![operation, write_to_effective_address],
            ],
        }
    }
}

pub enum AbsoluteXIndexedAddressing {
    Read,
    ReadModifyWrite,
    Write,
}

impl Instruction for AbsoluteXIndexedAddressing {
    fn get_cycles(&self, operation: StateOperation) -> Vec<Cycle> {
        match self {
            AbsoluteXIndexedAddressing::Read => vec![
                cycle![fetch_low_effective_address_byte],
                cycle![fetch_high_effective_address_byte_absolute_x_indexed],
                /*
                 * The below cycle assumes the `operation` parameter fixes the
                 * high address byte and queues another cycle if necessary.
                 * Ideally this would be the responsibility of this function
                 * instead.
                 *
                 * TODO: Determine how to handle high address byte fix and cycle
                 * re-queue from outside of the `operation` parameter
                 */
                cycle![read_from_effective_address, operation],
            ],
            AbsoluteXIndexedAddressing::ReadModifyWrite => vec![
                cycle![fetch_low_effective_address_byte],
                cycle![fetch_high_effective_address_byte_absolute_x_indexed],
                cycle![
                    read_from_effective_address,
                    fix_high_effective_address_byte_absolute_indexed
                ],
                cycle![read_from_effective_address],
                cycle![write_to_effective_address, operation],
                cycle![write_to_effective_address],
            ],
            AbsoluteXIndexedAddressing::Write => vec![
                cycle![fetch_low_effective_address_byte],
                cycle![fetch_high_effective_address_byte_absolute_x_indexed],
                cycle![
                    read_from_effective_address,
                    fix_high_effective_address_byte_absolute_indexed
                ],
                cycle![operation, write_to_effective_address],
            ],
        }
    }
}

pub enum AbsoluteYIndexedAddressing {
    Read,
    ReadModifyWrite,
    Write,
}

impl Instruction for AbsoluteYIndexedAddressing {
    fn get_cycles(&self, operation: StateOperation) -> Vec<Cycle> {
        match self {
            AbsoluteYIndexedAddressing::Read => vec![
                cycle![fetch_low_effective_address_byte],
                cycle![fetch_high_effective_address_byte_absolute_y_indexed],
                /*
                 * The below cycle assumes the `operation` parameter fixes the
                 * high address byte and queues another cycle if necessary.
                 * Ideally this would be the responsibility of this function
                 * instead.
                 *
                 * TODO: Determine how to handle high address byte fix and cycle
                 * re-queue from outside of the `operation` parameter
                 */
                cycle![read_from_effective_address, operation],
            ],
            AbsoluteYIndexedAddressing::ReadModifyWrite => vec![
                cycle![fetch_low_effective_address_byte],
                cycle![fetch_high_effective_address_byte_absolute_y_indexed],
                cycle![
                    read_from_effective_address,
                    fix_high_effective_address_byte_absolute_indexed
                ],
                cycle![read_from_effective_address],
                cycle![write_to_effective_address, operation],
                cycle![write_to_effective_address],
            ],
            AbsoluteYIndexedAddressing::Write => vec![
                cycle![fetch_low_effective_address_byte],
                cycle![fetch_high_effective_address_byte_absolute_y_indexed],
                cycle![
                    read_from_effective_address,
                    fix_high_effective_address_byte_absolute_indexed
                ],
                cycle![operation, write_to_effective_address],
            ],
        }
    }
}

/**
Reads the contents of the current effective address;
*/
pub fn read_from_effective_address(state: &mut State) {
    let address = concat_u8!(
        state.cycle_data.effective_address.0,
        state.cycle_data.effective_address.1
    );
    let data = state.read_from_memory(address);
    state.cycle_data.acting_data = data;
}

/**
Write the state's `acting_data` to the `effective_address` in its memory.
*/
pub fn write_to_effective_address(state: &mut State) {
    let address = concat_u8!(
        state.cycle_data.effective_address.0,
        state.cycle_data.effective_address.1
    );
    let data = state.cycle_data.acting_data;
    state.write_to_memory(address, data);
}

/**
Reads the contents of the program counter, loads it into the high byte of the
effective address, and then increments the program counter.
*/
fn fetch_high_effective_address_byte(state: &mut State) {
    let high_address_byte = state.read_from_pc_address();
    state.cycle_data.effective_address.0 = high_address_byte;
    state.increment_pc_address();
}

/**
Reads the contents of the program counter, loads it into the low byte of the
effective address, and then increments the program counter.
*/
fn fetch_low_effective_address_byte(state: &mut State) {
    let low_address_byte = state.read_from_pc_address();
    state.cycle_data.effective_address.1 = low_address_byte;
    state.increment_pc_address();
}

/**
Reads the contents of the program counter, loads it into the effective address
as a location in page zero, and then increments the program counter.
*/
fn fetch_effective_zero_page_address(state: &mut State) {
    let low_address_byte = state.read_from_pc_address();
    state.cycle_data.effective_address = (0x00, low_address_byte);
}

/**
Adds the contents of the X Index register to the zero page effective address.
*/
fn do_effective_zero_page_address_x_index(state: &mut State) {
    let address = concat_u8!(
        state.cycle_data.effective_address.0,
        state.cycle_data.effective_address.1
    );

    // TODO: Determine if the data read from memory needs to be stored in the
    // location being operated on. Example: Does `LDA ($01, X)` need to load the
    // contents of 0x0001 into the accumulator here? Or does the accumulator
    // only need to be loaded during the final cycle?

    // This memory read is required for cycle accuracy.
    let _ = state.read_from_memory(address);
    // Add X Index to low byte. High byte is always 0x00.
    let low_address_byte = state.cycle_data.effective_address.1;
    let offset = state.registers.x_index;
    let low_address_byte = low_address_byte.wrapping_add(offset);
    state.cycle_data.effective_address = (0x00, low_address_byte);
}

/**
Adds the contents of the Y Index register to the zero page effective address.
*/
fn do_effective_zero_page_address_y_index(state: &mut State) {
    let address = concat_u8!(
        state.cycle_data.effective_address.0,
        state.cycle_data.effective_address.1
    );

    // TODO: Determine if the data read from memory needs to be stored in the
    // location being operated on. Example: Does `LDX ($01, Y)` need to load the
    // contents of 0x0001 into the x index register here? Or does the x index
    // only need to be loaded during the final cycle?

    // This memory read is required for cycle accuracy.
    let _ = state.read_from_memory(address);
    // Add Y Index to low byte. High byte is always 0x00.
    let low_address_byte = state.cycle_data.effective_address.1;
    let offset = state.registers.y_index;
    let low_address_byte = low_address_byte.wrapping_add(offset);
    state.cycle_data.effective_address = (0x00, low_address_byte);
}

/**
Fetches the high effective address byte and then adds the X Index register to
the low effective address byte.
*/
fn fetch_high_effective_address_byte_absolute_x_indexed(state: &mut State) {
    // Fetch high byte of address
    fetch_high_effective_address_byte(state);

    // Add index register to low address byte
    let low_address_byte = state.cycle_data.effective_address.1;
    let offset = state.registers.x_index;
    let (low_address_byte, overflow) = low_address_byte.overflowing_add(offset);

    state.cycle_data.effective_address.1 = low_address_byte;
    state.cycle_data.crossed_page = overflow;
}

/**
Fetches the high effective address byte and then adds the Y Index register to
the low effective address byte.
*/
fn fetch_high_effective_address_byte_absolute_y_indexed(state: &mut State) {
    // Fetch high byte of address
    fetch_high_effective_address_byte(state);

    // Add index register to low address byte
    let low_address_byte = state.cycle_data.effective_address.1;
    let offset = state.registers.y_index;
    let (low_address_byte, overflow) = low_address_byte.overflowing_add(offset);

    state.cycle_data.effective_address.1 = low_address_byte;
    state.cycle_data.crossed_page = overflow;
}

/**
Fixes the high effective address byte for absolute indexed addressing modes.
*/
pub fn fix_high_effective_address_byte_absolute_indexed(state: &mut State) {
    let high_address_byte = state.cycle_data.effective_address.0;
    // TODO: If this wraps, do we need to adjust the low address byte?
    state.cycle_data.effective_address.0 = high_address_byte.wrapping_add(1);
}
