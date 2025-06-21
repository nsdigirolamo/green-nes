use crate::{
    concat_u8,
    emu::{State, instruction::Operation},
    split_u16,
};

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
    fn execute_on(&self, mut state: State) -> State {
        match *self {
            Self::Immediate { operand } => {
                state.registers.accumulator = operand;
                state.cycle_count += 2;
            }
            Self::ZeroPage { operand } => {
                let address = concat_u8!(0x00, operand);

                let data = state.read_from_memory(address);
                state.registers.accumulator = data;
                state.cycle_count += 3;
            }
            Self::ZeroPageX { operand } => {
                let x_index_offset = state.registers.x_index as u16;
                let zero_page_address = concat_u8!(0x00, operand);
                let address = zero_page_address.wrapping_add(x_index_offset);

                let data = state.read_from_memory(address);
                state.registers.accumulator = data;
                state.cycle_count += 4;
            }
            Self::Absolute { operand } => {
                let address = operand;

                let data = state.read_from_memory(address);
                state.registers.accumulator = data;
                state.cycle_count += 4;
            }
            Self::AbsoluteX { operand } => {
                let x_index_offset = state.registers.x_index as u16;
                let address = operand.wrapping_add(x_index_offset);

                let data = state.read_from_memory(address);
                state.registers.accumulator = data;

                let first_page = split_u16!(operand).0;
                let second_page = split_u16!(address).0;

                if first_page == second_page {
                    state.cycle_count += 4;
                } else {
                    state.cycle_count += 5;
                }
            }
            Self::AbsoluteY { operand } => {
                let y_index_offset = state.registers.y_index as u16;
                let address = operand.wrapping_add(y_index_offset);

                let data = state.read_from_memory(address);
                state.registers.accumulator = data;

                let first_page = split_u16!(operand).0;
                let second_page = split_u16!(address).0;

                if first_page == second_page {
                    state.cycle_count += 4;
                } else {
                    state.cycle_count += 5;
                }
            }
            Self::IndirectX { operand } => {
                let indirect_address = concat_u8!(0x00, operand);
                let low_order_address_byte = state.read_from_memory(indirect_address);
                let high_order_address_byte =
                    state.read_from_memory(indirect_address.wrapping_add(1));

                let x_index_offset = state.registers.x_index as u16;
                let address = concat_u8!(high_order_address_byte, low_order_address_byte)
                    .wrapping_add(x_index_offset);

                let data = state.read_from_memory(address);
                state.registers.accumulator = data;

                state.cycle_count += 6;
            }
            Self::IndirectY { operand } => {
                let indirect_address = concat_u8!(0x00, operand);
                let low_order_address_byte = state.read_from_memory(indirect_address);
                let high_order_address_byte =
                    state.read_from_memory(indirect_address.wrapping_add(1));

                let y_index_offset = state.registers.y_index as u16;
                let nonoffset_address = concat_u8!(high_order_address_byte, low_order_address_byte);
                let address = nonoffset_address.wrapping_add(y_index_offset);

                let data = state.read_from_memory(address);
                state.registers.accumulator = data;

                let first_page = split_u16!(nonoffset_address).0;
                let second_page = split_u16!(address).0;

                if first_page == second_page {
                    state.cycle_count += 5
                } else {
                    state.cycle_count += 6
                }
            }
        }

        state.set_negative_flag(state.registers.accumulator >> 7 != 0);
        state.set_zero_flag(state.registers.accumulator == 0);

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
