use std::{
    fmt::{self},
    path::Path,
};

use crate::emu::{
    error::{EmuError, LoadError},
    instruction::get_instruction,
};

pub mod error;
pub mod instruction;

const MAX_MEMORY_ADDRESS: u16 = 65535;
const MAX_STACK_ADDRESS: u16 = 0x00FF;

const MEMORY_LENGTH: usize = MAX_MEMORY_ADDRESS as usize + 1;
const PROGRAM_HEADER_LENGTH: usize = 16;

#[derive(Clone, Copy, Default, Debug)]
pub struct Registers {
    pub accumulator: u8,
    pub x_index: u8,
    pub y_index: u8,
    pub program_counter: u16,
    pub stack_pointer: u8,
    pub processor_status: u8,
}

#[derive(Clone, Copy, Debug)]
pub struct State {
    pub registers: Registers,
    memory: [u8; MEMORY_LENGTH],
    pub cycle_count: u64,
    pub is_halted: bool,
}

impl Default for State {
    fn default() -> Self {
        State {
            registers: Registers::default(),
            memory: [0u8; MEMORY_LENGTH],
            cycle_count: 0,
            is_halted: true,
        }
    }
}

impl State {
    fn get_memory(self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    // fn set_memory(mut self, address: u16, data: u8) {
    //     self.memory[address as usize] = data;
    // }
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

pub trait Operation: fmt::Debug {
    fn execute_on(&self, state: State) -> State;
    fn get_size(&self) -> u16;
}

pub fn fetch_next_operation(state: State) -> impl Operation {
    let pc = state.registers.program_counter;

    get_instruction((
        state.get_memory(pc),
        state.get_memory(pc.wrapping_add(1)),
        state.get_memory(pc.wrapping_add(2)),
    ))
}

pub fn load_program(mut state: State, path_to_program: &str) -> Result<State, LoadError> {
    let program =
        std::fs::read(Path::new(path_to_program)).map_err(|e| LoadError::FileOpenFailed {
            message: e.to_string(),
        })?;
    if program.len() < PROGRAM_HEADER_LENGTH {
        return Err(LoadError::MissingHeader);
    }

    let starting_addr: u16 = MAX_STACK_ADDRESS + 1;
    state.registers.program_counter = starting_addr;

    let program = &program[PROGRAM_HEADER_LENGTH..];
    let maximum_program_size = MEMORY_LENGTH - starting_addr as usize;

    if maximum_program_size < program.len() {
        return Err(LoadError::ProgramTooLarge {
            maximum_size: maximum_program_size,
        });
    }

    for (index, &data) in program.iter().enumerate() {
        let address = starting_addr as usize + index;
        state.memory[address] = data;
    }

    Ok(state)
}
