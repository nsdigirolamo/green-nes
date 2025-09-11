use std::process;

use clap::{Parser, Subcommand, ValueEnum};

use crate::{
    cpu::state::State,
    emu::{load_program, run_emulator},
};

pub mod cpu;
pub mod emu;
pub mod ppu;

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
            let state = match load_program(state, path.as_str()) {
                Ok(state) => state,
                Err(err) => {
                    eprintln!("Loading program failed: {err}");
                    process::exit(1);
                }
            };

            let mut final_state = match run_emulator(state, debug_level) {
                Ok(state) => state,
                Err(err) => {
                    eprintln!("Running program failed: {err}");
                    process::exit(1);
                }
            };

            // @TODO: Uncomment once output is fixed.
            // let cycle_count = final_state.abstracts.half_cycle_count / 2;
            // println!("Completed {cycle_count} cycles. Final State:\n{final_state:?}");

            let status02 = final_state.buses.read((0x00, 0x02));
            let status03 = final_state.buses.read((0x00, 0x03));
            println!("[0x02, 0x03]: [0x{status02:02X}, 0x{status03:02X}]");
        }
    }

    process::exit(0);
}
