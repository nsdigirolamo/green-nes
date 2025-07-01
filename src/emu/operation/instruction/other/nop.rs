use std::collections::VecDeque;

use crate::emu::{
    Event,
    operation::{
        Operation,
        instruction::{fetch_high_effective_address_byte, fetch_low_effective_address_byte},
    },
    state::State,
};

#[derive(Debug)]
pub enum NOP {
    Absolute,
}

impl Operation for NOP {
    fn get_events(&self) -> VecDeque<Event> {
        match *self {
            NOP::Absolute => VecDeque::from([
                fetch_low_effective_address_byte,
                fetch_high_effective_address_byte,
                nop,
            ]),
        }
    }
}

fn nop(_state: &mut State) {}
