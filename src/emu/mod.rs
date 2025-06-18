#![allow(dead_code)]
#![allow(unused_variables)]

use std::{
    fmt::{self},
    path::Path,
};

use crate::{
    concat_u8,
    emu::{
        error::{EmuError, LoadError},
        instruction::get_instruction,
    },
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
    fn absolute_get_memory(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    fn zero_page_get_memory(&self, low_order_address_byte: u8) -> u8 {
        let address = concat_u8!(0x00, low_order_address_byte);

        self.absolute_get_memory(address)
    }

    fn indirect_absolute_get_memory(&self, indirect_address: u16) -> u8 {
        let low_order_address_byte = self.absolute_get_memory(indirect_address);
        let high_order_address_byte = self.absolute_get_memory(indirect_address.wrapping_add(1));
        let address = concat_u8!(high_order_address_byte, low_order_address_byte);

        self.absolute_get_memory(address)
    }

    fn absolute_x_indexed_get_memory(&self, address: u16) -> u8 {
        let x_index_offset = self.registers.x_index as u16;

        self.absolute_get_memory(address.wrapping_add(x_index_offset))
    }

    fn absolute_y_indexed_get_memory(&self, address: u16) -> u8 {
        let y_index_offset = self.registers.y_index as u16;

        self.absolute_get_memory(address.wrapping_add(y_index_offset))
    }

    fn zero_page_x_indexed_get_memory(&self, low_order_address_byte: u8) -> u8 {
        let x_index_offset = self.registers.x_index as u16;
        let address = concat_u8!(0x00, low_order_address_byte);

        self.absolute_get_memory(address.wrapping_add(x_index_offset))
    }

    fn zero_page_y_indexed_get_memory(&self, low_order_address_byte: u8) -> u8 {
        let y_index_offset = self.registers.y_index as u16;
        let address = concat_u8!(0x00, low_order_address_byte);

        self.absolute_get_memory(address.wrapping_add(y_index_offset))
    }

    fn indirect_x_indexed_get_memory(&self, low_order_address_byte: u8) -> u8 {
        let indirect_address = concat_u8!(0x00, low_order_address_byte);
        let low_order_address_byte = self.absolute_get_memory(indirect_address);
        let high_order_address_byte = self.absolute_get_memory(indirect_address.wrapping_add(1));

        let address = concat_u8!(high_order_address_byte, low_order_address_byte);
        let x_index_offset = self.registers.x_index as u16;

        self.absolute_get_memory(address.wrapping_add(x_index_offset))
    }

    fn indirect_y_indexed_get_memory(&self, low_order_address_byte: u8) -> u8 {
        let indirect_address = concat_u8!(0x00, low_order_address_byte);
        let low_order_address_byte = self.absolute_get_memory(indirect_address);
        let high_order_address_byte = self.absolute_get_memory(indirect_address.wrapping_add(1));

        let address = concat_u8!(high_order_address_byte, low_order_address_byte);
        let y_index_offset = self.registers.y_index as u16;

        self.absolute_get_memory(address.wrapping_add(y_index_offset))
    }

    fn get_negative_flag(&self) -> bool {
        (self.registers.processor_status & 0b10000000) != 0
    }

    fn get_overflow_flag(&self) -> bool {
        (self.registers.processor_status & 0b01000000) != 0
    }

    fn get_break_command_flag(&self) -> bool {
        (self.registers.processor_status & 0b00010000) != 0
    }

    fn get_decimal_mode_flag(&self) -> bool {
        (self.registers.processor_status & 0b00001000) != 0
    }

    fn get_interrupt_disable_flag(&self) -> bool {
        (self.registers.processor_status & 0b00000100) != 0
    }

    fn get_zero_flag(&self) -> bool {
        (self.registers.processor_status & 0b00000010) != 0
    }

    fn get_carry_flag(&self) -> bool {
        (self.registers.processor_status & 0b00000001) != 0
    }
}

pub fn set_negative_flag(mut state: State) -> State {
    let new_status = state.registers.processor_status | 0b10000000;
    state.registers.processor_status = new_status;

    state
}

pub fn set_overflow_flag(mut state: State) -> State {
    let new_status = state.registers.processor_status | 0b10000000;
    state.registers.processor_status = new_status;

    state
}

pub fn set_break_command_flag(mut state: State) -> State {
    let new_status = state.registers.processor_status & 0b00010000;
    state.registers.processor_status = new_status;

    state
}

pub fn set_decimal_mode_command_flag(mut state: State) -> State {
    let new_status = state.registers.processor_status & 0b00001000;
    state.registers.processor_status = new_status;

    state
}

pub fn set_interrupt_disable_command_flag(mut state: State) -> State {
    let new_status = state.registers.processor_status & 0b00000100;
    state.registers.processor_status = new_status;

    state
}

pub fn set_zero_flag(mut state: State) -> State {
    let new_status = state.registers.processor_status & 0b00000010;
    state.registers.processor_status = new_status;

    state
}

pub fn set_carry_flag(mut state: State) -> State {
    let new_status = state.registers.processor_status & 0b00000001;
    state.registers.processor_status = new_status;

    state
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
        state.absolute_get_memory(pc),
        state.absolute_get_memory(pc.wrapping_add(1)),
        state.absolute_get_memory(pc.wrapping_add(2)),
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
