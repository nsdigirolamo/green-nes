pub const MAX_MEMORY_ADDRESS: u16 = 65535;
pub const MAX_STACK_ADDRESS: u16 = 0x00FF;

pub const MEMORY_LENGTH: usize = MAX_MEMORY_ADDRESS as usize + 1;
pub const PROGRAM_HEADER_LENGTH: usize = 16;

pub struct AddressResult {
    address: u16,
    page_crossed: bool,
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
    pub fn read_from_memory(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub fn write_to_memory(&mut self, address: u16, data: u8) {
        self.memory[address as usize] = data;
    }

    pub fn get_negative_flag(&self) -> bool {
        (self.registers.processor_status & 0b10000000) != 0
    }

    pub fn get_overflow_flag(&self) -> bool {
        (self.registers.processor_status & 0b01000000) != 0
    }

    pub fn get_break_command_flag(&self) -> bool {
        (self.registers.processor_status & 0b00010000) != 0
    }

    pub fn get_decimal_mode_flag(&self) -> bool {
        (self.registers.processor_status & 0b00001000) != 0
    }

    pub fn get_interrupt_disable_flag(&self) -> bool {
        (self.registers.processor_status & 0b00000100) != 0
    }

    pub fn get_zero_flag(&self) -> bool {
        (self.registers.processor_status & 0b00000010) != 0
    }

    pub fn get_carry_flag(&self) -> bool {
        (self.registers.processor_status & 0b00000001) != 0
    }

    pub fn set_negative_flag(&mut self, flag: bool) {
        let new_status = if flag {
            self.registers.processor_status | 0b10000000
        } else {
            self.registers.processor_status & 0b01111111
        };

        self.registers.processor_status = new_status;
    }

    pub fn set_overflow_flag(&mut self, flag: bool) {
        let new_status = if flag {
            self.registers.processor_status | 0b01000000
        } else {
            self.registers.processor_status & 0b10111111
        };

        self.registers.processor_status = new_status;
    }

    pub fn set_break_command_flag(&mut self, flag: bool) {
        let new_status = if flag {
            self.registers.processor_status | 0b00010000
        } else {
            self.registers.processor_status & 0b11101111
        };

        self.registers.processor_status = new_status;
    }

    pub fn set_decimal_mode_command_flag(&mut self, flag: bool) {
        let new_status = if flag {
            self.registers.processor_status | 0b00001000
        } else {
            self.registers.processor_status & 0b11110111
        };

        self.registers.processor_status = new_status;
    }

    pub fn set_interrupt_disable_command_flag(&mut self, flag: bool) {
        let new_status = if flag {
            self.registers.processor_status | 0b00000100
        } else {
            self.registers.processor_status & 0b11111011
        };

        self.registers.processor_status = new_status;
    }

    pub fn set_zero_flag(&mut self, flag: bool) {
        let new_status = if flag {
            self.registers.processor_status | 0b00000010
        } else {
            self.registers.processor_status & 0b11111101
        };

        self.registers.processor_status = new_status;
    }

    pub fn set_carry_flag(&mut self, flag: bool) {
        let new_status = if flag {
            self.registers.processor_status | 0b00000001
        } else {
            self.registers.processor_status & 0b11111110
        };

        self.registers.processor_status = new_status;
    }
}
