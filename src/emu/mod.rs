use std::{
    collections::VecDeque,
    fmt::{self},
    path::Path,
};

use crate::emu::{
    error::{EmuError, LoadError},
    operation::{fetch_opcode, get_operation},
    state::{PROGRAM_START_ADDRESS, State},
};

pub mod error;
pub mod operation;
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

type Event = fn(&mut State);

pub trait Operation: fmt::Debug {
    fn get_events(&self) -> VecDeque<Event>;
}

pub const PROGRAM_HEADER_LENGTH: usize = 16;

pub fn run_emulator(state: &mut State) -> Result<State, EmuError> {
    let mut is_halted = false;
    let mut cycle_count = 0u64;

    let mut event_queue: VecDeque<Event> = VecDeque::new();

    while !is_halted {
        match event_queue.pop_front() {
            Some(event) => event(state),
            None => {
                fetch_opcode(state);
                let opcode = state.cycle_data.opcode;
                let operation = get_operation(opcode);
                let mut new_events = operation.get_events();

                event_queue.extend(new_events.drain(..));
            }
        }

        cycle_count += 1;
        is_halted = cycle_count > 100; // @TODO: Determine how to halt (haha halting problem)
    }

    println!("Cycles completed: {}", cycle_count);

    Ok(*state)
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
