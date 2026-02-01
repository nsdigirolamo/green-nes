use std::process;

use clap::{Parser, Subcommand, ValueEnum};

use crate::emu::{
    cartridge::{Cartridge, ines::read_cartridge},
    nes::NES,
};

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
        // An optional starting address, for programs that do not respect the
        // reset vector at $FFFC–$FFFD.
        #[arg(value_parser = clap::value_parser!(u16), default_value_t = 0xFFFF)]
        start_addr: u16,
    },
    /// Displays the contents of the pattern tables to the screen.
    Patterntables {
        /// Path to the NES program.
        path: String,
    },
    /// Displays the contents of the nametables to the screen.
    Nametables {
        /// Path to the NES program.
        path: String,
    },
}

#[derive(Clone, PartialEq, ValueEnum)]
pub enum DebugLevel {
    None,
    Low,
    High,
}

fn main() {
    let cli = Cli::parse();
    let debug_level = cli.debug;

    match cli.command {
        Commands::Run { path, start_addr } => {
            let cart = load_cart(&path);

            let start_addr = match start_addr {
                0xFFFF => None,
                _ => Some(start_addr),
            };

            let mut nes = NES::new(cart);
            nes.cpu.reset(&mut nes.buses, start_addr);
            nes.run(debug_level, false);
        }
        Commands::Patterntables { path } => {
            let cart = load_cart(&path);

            let nes = NES::new(cart.clone());
            nes.show_pattern_tables(cart);
        }
        Commands::Nametables { path } => {
            let cart = load_cart(&path);

            let nes = NES::new(cart.clone());
            nes.show_nametables(nes.buses.get_ppu());
        }
    }

    process::exit(0);
}

fn load_cart(path: &str) -> Cartridge {
    match read_cartridge(path) {
        Ok(cart) => cart,
        Err(err) => {
            eprintln!("Loading cartridge failed: {err}");
            process::exit(1);
        }
    }
}
