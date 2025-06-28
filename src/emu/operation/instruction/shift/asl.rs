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
pub enum ASL {
    Accumulator,
    ZeroPage,
    ZeroPageX,
    Absolute,
    AbsoluteX,
}

impl Operation for ASL {
    fn get_events(&self) -> VecDeque<Event> {
        match *self {
            ASL::Accumulator => panic!("asl accumulator not implemented"),
            ASL::ZeroPageX => panic!("asl zero page x not implemented"),
            ASL::ZeroPage => panic!("asl zero page not implemented"),
            ASL::Absolute => VecDeque::from([
                fetch_low_operand,
                fetch_high_operand,
                read_at_effective_absolute_address,
                asl,
                write_to_effective_absolute_address,
            ]),
            ASL::AbsoluteX => panic!("asl absolute x not implemented"),
        }
    }
}

fn asl(state: &mut State) {
    let address = state.cycle_data.effective_address;
    let data = state.cycle_data.acting_data;
    state.write_to_memory(address, data);

    let result = data << 1;
    state.cycle_data.acting_data = result;

    state.set_carry_flag((data & 0b_1000_0000) != 0);
    state.set_zero_flag(result == 0);
    state.set_negative_flag((result & 0b_1000_0000) != 0);
}
