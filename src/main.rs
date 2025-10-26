use std::process;

use clap::{Parser, Subcommand, ValueEnum};

use crate::emu::{cartridge::ines::read_cartridge, nes::NES, run_emulator};

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
    let debug_level = cli.debug;

    match cli.command {
        Commands::Run { path } => {
            let cart = match read_cartridge(&path) {
                Ok(cart) => cart,
                Err(err) => {
                    eprintln!("Loading cartridge failed: {err}");
                    process::exit(1);
                }
            };

            let mut nes = NES::new(cart);

            let final_state = run_emulator(&mut nes, debug_level);
            let status02 = final_state.buses.peek(0x0002);
            let status03 = final_state.buses.peek(0x0003);
            println!("[0x02, 0x03]: [0x{status02:02X}, 0x{status03:02X}]");
        }
    }

    process::exit(0);
}
