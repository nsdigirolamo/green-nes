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
    pub a: u8,   // Acumulator
    pub x: u8,   // X Index
    pub y: u8,   // Y Index
    pub pc: u16, // Program Counter
    pub s: u8,   // Stack Pointer
    pub p: u8,   // Processor Status Register
}

pub struct State {
    pub registers: Registers,
    pub memory: [u8; MEMORY_LOCATION_COUNT],
}

impl State {
    pub fn fetch_next_operation(state: State) -> impl Operation {
        let program_counter = state.registers.pc as usize;

        get_instruction((
            state.memory[program_counter],
            state.memory[program_counter + 1],
            state.memory[program_counter + 2],
        ))
    }
}
