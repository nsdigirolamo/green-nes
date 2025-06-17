use std::env;

use crate::emu::State;

pub mod emu;

fn main() {
    let state: State = Default::default();

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide an argument.");
        return;
    }

    let command = args[1].as_str();

    match command {
        "run" => {
            if args.len() < 3 {
                println!("Usage: green-nes run <path-to-program-file>");
                return;
            }

            let path_to_program = args[2].as_str();
            let final_state = match state.run(path_to_program) {
                Ok(state) => state,
                Err(e) => panic!("{}", e),
            };

            println!(
                "Done. Completed program in {} cycles.",
                final_state.cycle_count
            );
        }
        _ => println!("Invalid command."),
    }
}
