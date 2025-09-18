use crate::{
    DebugLevel,
    cpu::{
        cycles::{FETCH_INSTRUCTION, get_cycles},
        state::{PROGRAM_START_ADDRESS, State},
    },
    emu::error::Error as EmuError,
};

pub mod error;
pub mod ines;

#[macro_export]
macro_rules! concat_u8 {
    ($high:expr, $low:expr) => {
        (($high as u16) << 8) | ($low as u16)
    };
}

#[macro_export]
macro_rules! split_u16 {
    ($value:expr) => {
        (($value >> 8) as u8, $value as u8)
    };
}

#[macro_export]
macro_rules! did_signed_overflow {
    ($lhs:expr, $rhs:expr, $result:expr) => {
        (($lhs ^ $result) & ($rhs ^ $result) & 0x80) != 0
    };
}

pub const PROGRAM_HEADER_LENGTH: usize = 16;

pub fn run_emulator(mut state: State, debug_level: DebugLevel) -> Result<State, EmuError> {
    state.abstracts.half_cycle_count = 14;
    state.registers.pc = split_u16!(PROGRAM_START_ADDRESS);
    state.registers.sp = 0xFD;
    state.registers.psr = 0x24;

    while !state.abstracts.is_halted {
        match state.abstracts.cycle_queue.pop_front() {
            Some([phase1, phase2]) => {
                // @TODO: Uncomment once output is fixed.
                // @TODO: Look into: Do these if statement debug messages impact performance?
                if debug_level == DebugLevel::High {
                    println!("{state}");
                }

                phase1(&mut state);
                phase2(&mut state);
            }
            None => {
                // @TODO: Uncomment once output is fixed.
                // @TODO: Look into: Do these if statement debug messages impact performance?
                if debug_level == DebugLevel::Low {
                    println!("{state:?}");
                } else if debug_level == DebugLevel::High {
                    println!();
                    println!("{state}");
                }

                let [phase1, phase2] = FETCH_INSTRUCTION;
                phase1(&mut state);
                phase2(&mut state);

                let new_cycles = get_cycles(state.registers.ir);
                state.abstracts.cycle_queue.extend(new_cycles.iter());
            }
        };

        state.abstracts.half_cycle_count += 2;
    }

    Ok(state)
}

#[cfg(test)]
mod tests {
    use crate::{
        DebugLevel,
        cpu::state::State,
        emu::{
            ines::{Cartridge, read_cartridge},
            run_emulator,
        },
    };

    #[test]
    /// [nestest](http://nickmass.com/images/nestest.nes) with final results
    /// stored in `0x02` and `0x03` in memory. See the
    /// [docs](https://www.qmtpro.com/~nes/misc/nestest.txt) for more info.
    fn nestest() {
        let cartridge: Cartridge = read_cartridge("tests/nestest.nes").unwrap();
        let state: State = State::new(cartridge);

        let run_result = run_emulator(state, DebugLevel::None);
        let mut final_state = run_result.unwrap();

        assert_eq!(final_state.mem_read((0x00, 0x02)), 0x00);
        assert_eq!(final_state.mem_read((0x00, 0x03)), 0x00);
    }
}
