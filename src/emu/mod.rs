use crate::{DebugLevel, emu::nes::NES};

pub mod buses;
pub mod cartridge;
pub mod cpu;
pub mod error;
pub mod nes;
pub mod ppu;

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

pub fn run_emulator(nes: &mut NES, debug_level: DebugLevel) -> &NES {
    while !nes.cpu.is_halted {
        do_debug(nes, debug_level);
        nes.cpu.tick(&mut nes.buses);
    }

    nes
}

pub fn do_debug(nes: &NES, debug_level: DebugLevel) {
    match debug_level {
        DebugLevel::High => {
            println!("{nes}")
        }
        DebugLevel::Low => {
            println!("{nes:?}")
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        DebugLevel,
        emu::{
            cartridge::{Cartridge, ines::read_cartridge},
            nes::NES,
            run_emulator,
        },
    };

    #[test]
    /// [nestest](http://nickmass.com/images/nestest.nes) with final results
    /// stored in `0x02` and `0x03` in memory. See the
    /// [docs](https://www.qmtpro.com/~nes/misc/nestest.txt) for more info.
    fn nestest() {
        let cartridge: Cartridge = read_cartridge("tests/nestest.nes").unwrap();
        let mut nes = NES::new(cartridge);
        let final_state = run_emulator(&mut nes, DebugLevel::None);

        assert_eq!(final_state.buses.peek(0x0002), 0x00);
        assert_eq!(final_state.buses.peek(0x0003), 0x00);
    }
}
