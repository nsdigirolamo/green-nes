use crate::emu::{State, instruction::Operation};

#[derive(Debug)]
pub enum LoadAccumulator {
    Immediate { operand: u8 },
    ZeroPage { operand: u8 },
    ZeroPageX { operand: u8 },
    Absolute { operand: u16 },
    AbsoluteX { operand: u16 },
    AbsoluteY { operand: u16 },
    IndirectX { operand: u8 },
    IndirectY { operand: u8 },
}

impl Operation for LoadAccumulator {
    fn execute_on(&self, state: State) -> State {
        // match *self {
        //     Self::Immediate { operand } => {
        //         state.registers.accumulator = operand;
        //         state.cycle_count += 2;
        //     }
        //     Self::ZeroPage { operand } => {
        //         let address = concat_u8!(0x00, operand);
        //         let value = state.memory[address as usize];

        //         state.registers.accumulator = value;
        //         state.cycle_count += 3;
        //     }
        //     Self::ZeroPageX { operand } => {
        //         let address = concat_u8!(0x00, operand);
        //         let index_offset = state.registers.x_index as u16;
        //         let value = state.memory[(address + index_offset) as usize];

        //         state.registers.accumulator = value;
        //         state.cycle_count += 4;
        //     }
        //     Self::Absolute { operand } => {
        //         let address = operand;
        //         let value = state.memory[address as usize];

        //         state.registers.accumulator = value;
        //         state.cycle_count += 4;
        //     }
        //     Self::AbsoluteX { operand } => {
        //         let address = operand;
        //         let index_offset = state.registers.x_index as u16;
        //         let value = state.memory[(address + index_offset) as usize];

        //         state.registers.accumulator = value;

        //         let first_page = split_u16!(address).0;
        //         let second_page = split_u16!(address + index_offset).0;

        //         if first_page == second_page {
        //             state.cycle_count += 4;
        //         } else {
        //             state.cycle_count += 5;
        //         }
        //     }
        //     Self::AbsoluteY { operand } => {
        //         let address = operand;
        //         let index_offset = state.registers.y_index as u16;
        //         let value = state.memory[(address + index_offset) as usize];

        //         state.registers.accumulator = value;

        //         let first_page = split_u16!(address).0;
        //         let second_page = split_u16!(address + index_offset).0;

        //         if first_page == second_page {
        //             state.cycle_count += 4;
        //         } else {
        //             state.cycle_count += 5;
        //         }
        //     }
        //     Self::IndirectX { operand } => {
        //         let pointer_address = concat_u8!(0x00, operand);
        //         let address = concat_u8!(
        //             state.memory[(pointer_address + 1) as usize],
        //             state.memory[pointer_address as usize]
        //         );
        //         let index_offset = state.registers.x_index as u16;
        //         let value = state.memory[(address + index_offset) as usize];

        //         state.registers.accumulator = value;
        //         state.cycle_count += 6;
        //     }
        //     Self::IndirectY { operand } => {
        //         let pointer_address = concat_u8!(0x00, operand);
        //         let address = concat_u8!(
        //             state.memory[(pointer_address + 1) as usize],
        //             state.memory[pointer_address as usize]
        //         );
        //         let index_offset = state.registers.x_index as u16;
        //         let value = state.memory[(address + index_offset) as usize];

        //         state.registers.accumulator = value;

        //         let first_page = split_u16!(address).0;
        //         let second_page = split_u16!(address + index_offset).0;

        //         if first_page == second_page {
        //             state.cycle_count += 5;
        //         } else {
        //             state.cycle_count += 6;
        //         }
        //     }
        // }

        state
    }

    fn get_size(&self) -> u16 {
        match self {
            Self::Immediate { operand: _ } => 2,
            Self::ZeroPage { operand: _ } => 2,
            Self::ZeroPageX { operand: _ } => 2,
            Self::Absolute { operand: _ } => 3,
            Self::AbsoluteX { operand: _ } => 3,
            Self::AbsoluteY { operand: _ } => 3,
            Self::IndirectX { operand: _ } => 2,
            Self::IndirectY { operand: _ } => 2,
        }
    }
}
