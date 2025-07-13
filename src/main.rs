use std::process;

use clap::{Parser, Subcommand, ValueEnum};

use crate::emu::{load_program, run_emulator, state::State};

pub mod emu;

#[derive(clap::Parser)]
struct Cli {
    /// Debug level
    #[arg(long, short = 'd', value_enum, default_value_t = DebugLevel::None)]
    debug: DebugLevel,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Runs a NES program in the emulator.
    Run {
        /// Path to the NES program to execute.
        path: String,
    },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum DebugLevel {
    None,
    Low,
    High,
}

fn main() {
    let cli = Cli::parse();

    let state: State = Default::default();
    let debug_level = cli.debug;

    match cli.command {
        Commands::Run { path } => {
            let mut state = match load_program(state, path.as_str()) {
                Ok(state) => state,
                Err(err) => {
                    eprintln!("Loading program failed: {err}");
                    process::exit(1);
                }
            };

            let final_state = match run_emulator(&mut state, debug_level) {
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
        }
    }

    process::exit(0);
}
