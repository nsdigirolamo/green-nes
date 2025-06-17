use std::{
    fmt::{self},
    path::Path,
};

use crate::emu::instruction::get_instruction;

pub mod instruction;

const MEMORY_LOCATION_COUNT: usize = 65535;

#[derive(Debug, Clone)]
pub enum EmuError {
    LoadError { e: LoadError },
}

impl fmt::Display for EmuError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::LoadError { e } => write!(f, "{}", e),
        }
    }
}

#[derive(Debug, Clone)]
pub enum LoadError {
    ProgramTooLarge { maximum_size: usize },
    FileOpenFailed { message: String },
    MissingHeader,
}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ProgramTooLarge { maximum_size } => write!(
                f,
                "program is too large (exceeds maximum size of {} bytes)",
                maximum_size
            ),
            Self::FileOpenFailed { message } => {
                write!(f, "failed to open program file: {}", message)
            }
            Self::MissingHeader => {
                write!(f, "the program header is missing")
            }
        }
    }
}

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
    pub memory: [u8; MEMORY_LOCATION_COUNT],
    pub cycle_count: u64,
    pub is_halted: bool,
}

impl Default for State {
    fn default() -> Self {
        State {
            registers: Registers::default(),
            memory: [0u8; MEMORY_LOCATION_COUNT],
            cycle_count: 0,
            is_halted: true,
        }
    }
}

pub trait Operation: fmt::Debug {
    fn execute_on(&self, state: State) -> State;
    fn get_size(&self) -> u16;
}

impl State {
    pub fn run(self, path_to_program: &str) -> Result<State, EmuError> {
        let mut state = self;
        state = load_program(state, path_to_program).map_err(|e| EmuError::LoadError { e })?;
        //println!("{:?}", state);

        state.is_halted = false;

        while !state.is_halted {
            let next_operation = state.fetch_next_operation();
            println!("{:?}", next_operation);
            state = next_operation.execute_on(state);
            state.registers.program_counter += next_operation.get_size();
        }

        Ok(state)
    }

    pub fn fetch_next_operation(self) -> impl Operation {
        let program_counter = self.registers.program_counter as usize;

        get_instruction((
            self.memory[program_counter],
            self.memory[program_counter + 1],
            self.memory[program_counter + 2],
        ))
    }
}

fn load_program(mut state: State, path_to_program: &str) -> Result<State, LoadError> {
    let starting_addr: u16 = 0x0100;

    let program =
        std::fs::read(Path::new(path_to_program)).map_err(|e| LoadError::FileOpenFailed {
            message: e.to_string(),
        })?;
    if program.len() < 16 {
        return Err(LoadError::MissingHeader);
    }

    let program = &program[16..];

    let address_size = state.memory.len() - starting_addr as usize;
    if address_size < program.len() {
        return Err(LoadError::ProgramTooLarge {
            maximum_size: address_size,
        });
    }

    for (index, &data) in program.iter().enumerate() {
        let address = starting_addr + index as u16;
        state.memory[address as usize] = data;
    }

    state.registers.program_counter = starting_addr;

    Ok(state)
}
