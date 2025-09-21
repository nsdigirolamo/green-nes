use std::collections::VecDeque;

use crate::emu::cpu::state::Cycle;

#[derive(Default)]
pub struct Abstracts {
    pub cycle_queue: VecDeque<Cycle>,
    pub half_cycle_count: u64,
    pub is_halted: bool,
    pub crossed_page: bool,
}
