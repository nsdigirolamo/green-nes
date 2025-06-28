pub const MAX_MEMORY_ADDRESS: u16 = 65535;
pub const MAX_STACK_ADDRESS: u16 = 0x00FF;
pub const PROGRAM_START_ADDRESS: u16 = 0xC000;

pub const MEMORY_LENGTH: usize = MAX_MEMORY_ADDRESS as usize + 1;

#[derive(Clone, Copy, Default, Debug)]
pub struct Registers {
    pub accumulator: u8,
    pub x_index: u8,
    pub y_index: u8,
    pub program_counter: u16,
    pub stack_pointer: u8,
    pub processor_status: u8,
}

/// Data used by the "micro-instructions" on a per-cycle basis.
#[derive(Clone, Copy, Default, Debug)]
pub struct CycleData {
    pub opcode: u8,             // Used as if it were the instruction register (IR)
    pub low_operand: u8,        // Low operand of the instruction in memory.
    pub high_operand: u8,       // High operand of the instruction in memory.
    pub effective_address: u16, // The memory address that the current instruction is working on.
    pub acting_data: u8,        // The data that the current instruction is working on.
}

#[derive(Clone, Copy, Debug)]
pub struct State {
    memory: [u8; MEMORY_LENGTH],
    pub registers: Registers,
    pub cycle_data: CycleData,
}

impl Default for State {
    fn default() -> Self {
        State {
            memory: [0u8; MEMORY_LENGTH],
            registers: Registers::default(),
            cycle_data: CycleData::default(),
        }
    }
}

// impl fmt::Display for State {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         let pc = self.registers.program_counter;
//         let opcode = self.read_from_memory(pc);
//         let operand_1 = self.read_from_memory(pc.wrapping_add(1));
//         let operand_2 = self.read_from_memory(pc.wrapping_add(2));

//         let next_instruction = fetch_next_operation(*self);
//         let next_instruction_size = next_instruction.get_size();

//         let address_string = format!("{:04X}", pc);

//         let instruction_bytes_string = if next_instruction_size == 1 {
//             format!("{:02X}", opcode)
//         } else if next_instruction_size == 2 {
//             format!("{:02X} {:02X}", opcode, operand_1)
//         } else {
//             format!("{:02X} {:02X} {:02X}", opcode, operand_1, operand_2)
//         };
//         let instruction_bytes_string = format!("{:8}", instruction_bytes_string);

//         let instruction_name = format!("{:?}", next_instruction);

//         write!(
//             f,
//             "{}  {}  {}",
//             address_string, instruction_bytes_string, instruction_name
//         )
//     }
// }

impl State {
    pub fn read_from_memory(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub fn write_to_memory(&mut self, address: u16, data: u8) {
        self.memory[address as usize] = data;
    }

    pub fn read_from_pc_address(&self) -> u8 {
        let address = self.registers.program_counter;

        self.read_from_memory(address)
    }

    pub fn get_negative_flag(&self) -> bool {
        (self.registers.processor_status & 0b10000000) != 0
    }

    pub fn set_negative_flag(&mut self, flag: bool) {
        let new_status = if flag {
            self.registers.processor_status | 0b10000000
        } else {
            self.registers.processor_status & 0b01111111
        };

        self.registers.processor_status = new_status;
    }

    pub fn get_overflow_flag(&self) -> bool {
        (self.registers.processor_status & 0b01000000) != 0
    }

    pub fn set_overflow_flag(&mut self, flag: bool) {
        let new_status = if flag {
            self.registers.processor_status | 0b01000000
        } else {
            self.registers.processor_status & 0b10111111
        };

        self.registers.processor_status = new_status;
    }

    pub fn get_break_command_flag(&self) -> bool {
        (self.registers.processor_status & 0b00010000) != 0
    }

    pub fn set_break_command_flag(&mut self, flag: bool) {
        let new_status = if flag {
            self.registers.processor_status | 0b00010000
        } else {
            self.registers.processor_status & 0b11101111
        };

        self.registers.processor_status = new_status;
    }

    pub fn get_decimal_mode_flag(&self) -> bool {
        (self.registers.processor_status & 0b00001000) != 0
    }

    pub fn set_decimal_mode_command_flag(&mut self, flag: bool) {
        let new_status = if flag {
            self.registers.processor_status | 0b00001000
        } else {
            self.registers.processor_status & 0b11110111
        };

        self.registers.processor_status = new_status;
    }

    pub fn get_interrupt_disable_flag(&self) -> bool {
        (self.registers.processor_status & 0b00000100) != 0
    }

    pub fn set_interrupt_disable_command_flag(&mut self, flag: bool) {
        let new_status = if flag {
            self.registers.processor_status | 0b00000100
        } else {
            self.registers.processor_status & 0b11111011
        };

        self.registers.processor_status = new_status;
    }

    pub fn get_zero_flag(&self) -> bool {
        (self.registers.processor_status & 0b00000010) != 0
    }

    pub fn set_zero_flag(&mut self, flag: bool) {
        let new_status = if flag {
            self.registers.processor_status | 0b00000010
        } else {
            self.registers.processor_status & 0b11111101
        };

        self.registers.processor_status = new_status;
    }

    pub fn get_carry_flag(&self) -> bool {
        (self.registers.processor_status & 0b00000001) != 0
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
