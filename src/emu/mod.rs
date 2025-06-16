use crate::emu::instruction::get_instruction;

pub mod instruction;

const MEMORY_LOCATION_COUNT: usize = 65535;

pub trait Operation {
    fn execute_on(&self, state: State) -> State;
    fn get_size(&self) -> u8;
}

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

pub struct Registers {
    pub accumulator: u8,
    pub x_index: u8,
    pub y_index: u8,
    pub program_counter: u16,
    pub stack_pointer: u8,
    pub processor_status: u8,
}

pub struct State {
    pub registers: Registers,
    pub memory: [u8; MEMORY_LOCATION_COUNT],
    pub instruction_count: u64,
}

impl State {
    pub fn fetch_next_operation(state: State) -> impl Operation {
        let program_counter = state.registers.program_counter as usize;

        get_instruction((
            state.memory[program_counter],
            state.memory[program_counter + 1],
            state.memory[program_counter + 2],
        ))
    }
}
