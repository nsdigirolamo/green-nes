use std::collections::VecDeque;

use crate::emu::{
    Event,
    operation::{
        Operation,
        addressing::{read_at_effective_absolute_address, write_to_effective_absolute_address},
        instruction::{fetch_high_operand, fetch_low_operand},
    },
    state::State,
};

#[derive(Debug)]
pub enum ROR {
    Accumulator,
    ZeroPage,
    ZeroPageX,
    Absolute,
    AbsoluteX,
}

impl Operation for ROR {
    fn get_events(&self) -> VecDeque<Event> {
        match *self {
            ROR::Accumulator => panic!("ror accumulator not implemented"),
            ROR::ZeroPageX => panic!("ror zero page x not implemented"),
            ROR::ZeroPage => panic!("ror zero page not implemented"),
            ROR::Absolute => VecDeque::from([
                fetch_low_operand,
                fetch_high_operand,
                read_at_effective_absolute_address,
                ror,
                write_to_effective_absolute_address,
            ]),
            ROR::AbsoluteX => panic!("ror absolute x not implemented"),
        }
    }
}

fn ror(state: &mut State) {
    let address = state.cycle_data.effective_address;
    let data = state.cycle_data.acting_data;
    state.write_to_memory(address, data);

    let result = (data >> 1) & ((state.get_carry_flag() as u8) << 7);
    state.cycle_data.acting_data = result;

    state.set_carry_flag((data & 0b_0000_0001) != 0);
    state.set_zero_flag(result == 0);
    state.set_negative_flag((result & 0b_1000_0000) != 0);
}
