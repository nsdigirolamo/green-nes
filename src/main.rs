use std::{env, process};

use crate::emu::{load_program, run_emulator, state::State};

pub mod emu;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.get(1).map(|s| s.as_str()) {
        Some("run") => {
            let state: State = Default::default();
            let path_to_program = match args.get(2) {
                Some(path) => path,
                None => {
                    eprintln!("Missing path to program file");
                    process::exit(1);
                }
            };

            let mut state = match load_program(state, path_to_program) {
                Ok(state) => state,
                Err(err) => {
                    eprintln!("Loading program failed: {err}");
                    process::exit(1);
                }
            };

            let final_state = match run_emulator(&mut state) {
                Ok(state) => state,
                Err(err) => {
                    eprintln!("Running program failed: {err}");
                    process::exit(1);
                }
            };

            let cycle_count = final_state.half_cycle_count / 2;
            println!("Completed {cycle_count} cycles. Final State:\n{final_state:?}");

            let status02 = state.read_from_memory((0x00, 0x02));
            let status03 = state.read_from_memory((0x00, 0x03));
            println!("[0x02, 0x03]: [0x{status02:02X}, 0x{status03:02X}]");

            process::exit(0);
        }
        _ => {
            println!("Invalid command.");
            process::exit(1);
        }
    }
}
