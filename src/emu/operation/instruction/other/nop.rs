use std::collections::VecDeque;

use crate::emu::{
    Event,
    operation::{
        Operation,
        instruction::{fetch_high_operand, fetch_low_operand},
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
            NOP::Absolute => VecDeque::from([fetch_low_operand, fetch_high_operand, nop]),
        }
    }
}

fn nop(_state: &mut State) {}
