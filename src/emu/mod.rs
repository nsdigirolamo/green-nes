use std::path::Path;

use crate::emu::{
    cycles::{FETCH_INSTRUCTION, get_cycles},
    error::{EmuError, LoadError},
    state::{PROGRAM_START_ADDRESS, State},
};

pub mod cycles;
pub mod error;
pub mod half_cycles;
pub mod instructions;
pub mod operations;
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
    state.half_cycle_count = 14;

    while !state.is_halted {
        match state.cycle_queue.pop_front() {
            Some([phase1, phase2]) => {
                println!("{state:?}");
                phase1(state);
                phase2(state);
            }
            None => {
                println!();
                println!("{state:?}");

                let [phase1, phase2] = FETCH_INSTRUCTION;
                phase1(state);
                phase2(state);

                let new_cycles = get_cycles(state.instruction_register);
                state.cycle_queue.extend(new_cycles.iter());
            }
        };

        state.half_cycle_count += 2;
    }

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

    let starting_addr = split_u16!(PROGRAM_START_ADDRESS);
    state.program_counter = starting_addr;

    let program = &program[PROGRAM_HEADER_LENGTH..];
    // let maximum_program_size = state::MEMORY_LENGTH - starting_addr as usize;

    // if maximum_program_size < program.len() {
    //     return Err(LoadError::ProgramTooLarge {
    //         maximum_size: maximum_program_size,
    //     });
    // }

    for (index, &data) in program.iter().enumerate() {
        let address = concat_u8!(starting_addr.0, starting_addr.1).wrapping_add(index as u16);
        state.write_to_memory(split_u16!(address), data);

        if address == 0xFFFF {
            break;
        }
    }

    Ok(state)
}
