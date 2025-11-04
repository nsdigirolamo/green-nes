use std::time::Duration;

use sdl3::{event::Event, keyboard::Keycode, pixels::Color, rect::Point};

use crate::{
    DebugLevel,
    emu::{
        cartridge::{
            Cartridge,
            mappers::{COLS_PER_PATTERN_TABLE, PLANE_COUNT, TILE_WIDTH, TILES_PER_PATTERN_TABLE},
        },
        nes::NES,
    },
};

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

pub fn run_emulator(cart: Cartridge, debug_level: DebugLevel) -> NES {
    let mut nes = NES::new(cart);

    while !nes.cpu.is_halted {
        do_debug(&nes, debug_level);
        nes.cpu.tick(&mut nes.buses);
    }

    nes
}

pub fn display_pattern_tables(cart: Cartridge) {
    let pattern_tables = cart.mapper.borrow().dump_pattern_tables();

    // 8x8 pixel map where colors are determined by two bit values.
    type Tile = [(bool, bool); TILE_WIDTH * TILE_WIDTH];

    let mut patterns: Vec<[Tile; TILES_PER_PATTERN_TABLE]> = Vec::new();

    for pattern_table in pattern_tables {
        let mut new_patterns = [[(false, false); TILE_WIDTH * TILE_WIDTH]; TILES_PER_PATTERN_TABLE];

        for (tile, pixels) in new_patterns
            .iter_mut()
            .enumerate()
            .take(TILES_PER_PATTERN_TABLE)
        {
            let bytes_per_tile = TILE_WIDTH * PLANE_COUNT;
            let start_index = tile * bytes_per_tile;

            for row in 0..TILE_WIDTH {
                let low_bits = pattern_table[start_index + row];
                let high_bits = pattern_table[start_index + row + TILE_WIDTH];

                for col in 0..TILE_WIDTH {
                    let bit_mask = 0b_1000_0000 >> col;
                    let pattern: (bool, bool) =
                        ((high_bits & bit_mask) != 0, (low_bits & bit_mask) != 0);

                    let pixel_index = (row * TILE_WIDTH) + col;

                    pixels[pixel_index] = pattern;
                }
            }
        }

        patterns.push(new_patterns)
    }

    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Green NES", 500, 500)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    for (table_index, table) in patterns.iter().enumerate() {
        for (tile_index, tile) in table.iter().enumerate() {
            for (pixel_index, pixel) in tile.iter().enumerate() {
                let table_margins = Point::new(
                    2 + (table_index * ((COLS_PER_PATTERN_TABLE * 2) + 2)) as i32,
                    2,
                );

                let table_origin = Point::new(
                    (table_index * (TILE_WIDTH * COLS_PER_PATTERN_TABLE)) as i32,
                    0,
                ) + table_margins;

                let tile_offset = Point::new(
                    ((tile_index % COLS_PER_PATTERN_TABLE) * TILE_WIDTH) as i32,
                    ((tile_index / COLS_PER_PATTERN_TABLE) * TILE_WIDTH) as i32,
                );

                let pixel_offset = Point::new(
                    (pixel_index % TILE_WIDTH) as i32,
                    (pixel_index / TILE_WIDTH) as i32,
                );

                let margin_offset = Point::new(
                    ((tile_index % COLS_PER_PATTERN_TABLE) * 2) as i32,
                    ((tile_index / COLS_PER_PATTERN_TABLE) * 2) as i32,
                );

                let pixel_origin = table_origin + tile_offset + pixel_offset + margin_offset;

                match pixel {
                    (false, false) => canvas.set_draw_color(Color::RGB(0, 23, 155)),
                    (true, false) => canvas.set_draw_color(Color::RGB(160, 4, 226)),
                    (false, true) => canvas.set_draw_color(Color::RGB(252, 165, 15)),
                    (true, true) => canvas.set_draw_color(Color::WHITE),
                }

                canvas.draw_point(pixel_origin).unwrap();
            }
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
