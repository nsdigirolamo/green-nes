#![allow(dead_code)]
#![allow(unused_variables)]

use std::{
    fmt::{self},
    path::Path,
};

use crate::emu::{
    error::{EmuError, LoadError},
    instruction::get_instruction,
    state::{PROGRAM_HEADER_LENGTH, State},
};

pub mod error;
pub mod instruction;
pub mod state;

pub trait Operation: fmt::Debug {
    fn execute_on(&self, state: State) -> State;
    fn get_size(&self) -> u16;
}

pub fn run_emulator(state: State) -> Result<State, EmuError> {
    let mut current_state = state;
    current_state.is_halted = false;

    while !current_state.is_halted {
        let next_operation = fetch_next_operation(current_state);
        let mut next_state = next_operation.execute_on(current_state);

        let next_program_counter = current_state
            .registers
            .program_counter
            .wrapping_add(next_operation.get_size());
        next_state.registers.program_counter = next_program_counter;

        current_state = next_state;

        println!("{:?}", next_operation);
    }

    Ok(current_state)
}

pub fn fetch_next_operation(state: State) -> impl Operation {
    let pc = state.registers.program_counter;

    get_instruction((
        state.read_from_memory(pc),
        state.read_from_memory(pc.wrapping_add(1)),
        state.read_from_memory(pc.wrapping_add(2)),
    ))
}

pub fn load_program(mut state: State, path_to_program: &str) -> Result<State, LoadError> {
    let program =
        std::fs::read(Path::new(path_to_program)).map_err(|e| LoadError::FileOpenFailed {
            message: e.to_string(),
        })?;
    if program.len() < state::PROGRAM_HEADER_LENGTH {
        return Err(LoadError::MissingHeader);
    }

    let starting_addr: u16 = state::MAX_STACK_ADDRESS + 1;
    state.registers.program_counter = starting_addr;

    let program = &program[PROGRAM_HEADER_LENGTH..];
    let maximum_program_size = state::MEMORY_LENGTH - starting_addr as usize;

    if maximum_program_size < program.len() {
        return Err(LoadError::ProgramTooLarge {
            maximum_size: maximum_program_size,
        });
    }

    for (index, &data) in program.iter().enumerate() {
        let address = starting_addr + index as u16;
        state.write_to_memory(address, data)
    }

    Ok(state)
}
