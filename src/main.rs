use std::process;

use clap::{Parser, Subcommand, ValueEnum};

use crate::emu::{cartridge::ines::read_cartridge, nes::NES};

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
        /// Path to the NES program.
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
            nes.cpu.reset(&mut nes.buses);
            nes.run(debug_level);
        }
    }

    process::exit(0);
}
