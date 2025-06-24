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
                Err(e) => {
                    eprintln!("Loading program failed: {}", e);
                    process::exit(1);
                }
            };

            let final_state = match run_emulator(&mut state) {
                Ok(state) => state,
                Err(e) => {
                    eprintln!("Running program failed: {}", e);
                    process::exit(1);
                }
            };

            println!("{:?}", final_state);

            process::exit(0);
        }
        _ => {
            println!("Invalid command.");
            process::exit(1);
        }
    }
}
