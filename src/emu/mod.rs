use std::path::Path;

use crate::emu::{
    error::{EmuError, LoadError},
    instructions::{fetch_opcode, get_operation},
    state::{Cycle, PROGRAM_START_ADDRESS, State},
};

pub mod addressing;
pub mod error;
pub mod instructions;
pub mod state;

#[macro_export]
macro_rules! concat_u8 {
    ($high:expr, $low:expr) => {
        (($high as u16) << 8) | ($low as u16)
    };
}

#[macro_export]
macro_rules! split_u16 {
    ($value:expr) => {
        (($value >> 8) as u8, $value as u8)
    };
}

#[macro_export]
macro_rules! did_signed_overflow {
    ($lhs:expr, $rhs:expr, $result:expr) => {
        (($lhs ^ $result) & ($rhs ^ $result) & 0x80) != 0
    };
}

pub const PROGRAM_HEADER_LENGTH: usize = 16;

pub fn run_emulator(state: &mut State) -> Result<&State, EmuError> {
    let mut is_halted = false;
    let mut cycle_count = 0u64;

    while !is_halted {
        match state.cycle_queue.pop_front() {
            Some(cycle) => {
                cycle.iter().for_each(|operation| operation(state));
            }
            None => {
                fetch_opcode(state);
                let new_cycles: Vec<Cycle> = get_operation(state.cycle_data.opcode);
                state.cycle_queue.extend(new_cycles.into_iter());
            }
        };

        cycle_count += 1;
        is_halted = cycle_count > 100; // @TODO: Determine when to halt
    }

    println!("Cycles completed: {cycle_count}");

    Ok(state)
}

pub fn load_program(mut state: State, path_to_program: &str) -> Result<State, LoadError> {
    // @TODO: Clean this function up. Should check to ensure the file being
    // loaded is a valid file format.

    let program =
        std::fs::read(Path::new(path_to_program)).map_err(|e| LoadError::FileOpenFailed {
            message: e.to_string(),
        })?;
    if program.len() < PROGRAM_HEADER_LENGTH {
        return Err(LoadError::MissingHeader);
    }

    let starting_addr: u16 = PROGRAM_START_ADDRESS;
    state.registers.program_counter = starting_addr;

    let program = &program[PROGRAM_HEADER_LENGTH..];
    // let maximum_program_size = state::MEMORY_LENGTH - starting_addr as usize;

    // if maximum_program_size < program.len() {
    //     return Err(LoadError::ProgramTooLarge {
    //         maximum_size: maximum_program_size,
    //     });
    // }

    for (index, &data) in program.iter().enumerate() {
        let address = starting_addr + index as u16;
        state.write_to_memory(address, data);

        if address == 0xFFFF {
            break;
        }
    }

    Ok(state)
}
