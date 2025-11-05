use std::time::Duration;

use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Point};

use crate::{
    DebugLevel,
    emu::{cartridge::Cartridge, nes::NES, screen::create_pattern_table_screen},
};

pub mod buses;
pub mod cartridge;
pub mod cpu;
pub mod error;
pub mod nes;
pub mod ppu;
pub mod screen;

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

pub fn run_emulator(cart: Cartridge, debug_level: DebugLevel) -> NES {
    let mut nes = NES::new(cart);

    while !nes.cpu.is_halted {
        do_debug(&nes, debug_level);
        nes.cpu.tick(&mut nes.buses);
    }

    nes
}

pub fn display_pattern_tables(cart: Cartridge) {
    let screen = create_pattern_table_screen(cart);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Green NES", screen.get_width() * 2, screen.get_height() * 2)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas
        .set_logical_size(screen.get_width(), screen.get_height())
        .unwrap();

    canvas.set_draw_color(Color::BLACK);
    canvas.clear();

    for row in 0..screen.get_height() as i32 {
        for col in 0..screen.get_width() as i32 {
            let color = screen.get_pixel(Point::new(col, row));
            canvas.set_draw_color(color);
            canvas.draw_point(Point::new(col, row)).unwrap();
        }
    }

    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn do_debug(nes: &NES, debug_level: DebugLevel) {
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
            run_emulator,
        },
    };

    #[test]
    /// [nestest](http://nickmass.com/images/nestest.nes) with final results
    /// stored in `0x02` and `0x03` in memory. See the
    /// [docs](https://www.qmtpro.com/~nes/misc/nestest.txt) for more info.
    fn nestest() {
        let cart: Cartridge = read_cartridge("tests/nestest.nes").unwrap();
        let final_state = run_emulator(cart, DebugLevel::None);

        assert_eq!(final_state.buses.peek(0x0002), 0x00);
        assert_eq!(final_state.buses.peek(0x0003), 0x00);
    }
}
