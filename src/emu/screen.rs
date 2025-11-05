use sdl2::{pixels::Color, rect::Point};

use crate::emu::cartridge::{
    Cartridge,
    mappers::{COLS_PER_PATTERN_TABLE, PLANE_COUNT, TILE_WIDTH, TILES_PER_PATTERN_TABLE},
};

const DEFAULT_SCREEN_WIDTH: usize = 283;
const DEFAULT_SCREEN_HEIGHT: usize = 242;

pub struct Screen {
    width: usize,
    height: usize,
    pixels: Vec<Vec<Color>>,
}

impl Screen {
    pub fn new(width: u32, height: u32, color: Color) -> Self {
        Screen {
            width: width as usize,
            height: height as usize,
            pixels: vec![vec![color; width as usize]; height as usize],
        }
    }

    pub fn get_width(&self) -> u32 {
        self.width as u32
    }

    pub fn get_height(&self) -> u32 {
        self.height as u32
    }

    pub fn get_pixel(&self, location: Point) -> Color {
        self.pixels[location.y as usize][location.x as usize]
    }

    pub fn set_pixel(&mut self, location: Point, color: Color) {
        self.pixels[location.y as usize][location.x as usize] = color
    }
}

impl Default for Screen {
    fn default() -> Self {
        Screen {
            width: DEFAULT_SCREEN_WIDTH,
            height: DEFAULT_SCREEN_HEIGHT,
            pixels: vec![vec![Color::RGB(0, 0, 0); DEFAULT_SCREEN_WIDTH]; DEFAULT_SCREEN_HEIGHT],
        }
    }
}

pub fn create_pattern_table_screen(cart: Cartridge) -> Screen {
    let byte_arrays = cart.mapper.borrow().dump_pattern_tables();

    type Tile = [(bool, bool); TILE_WIDTH * TILE_WIDTH];
    let mut pattern_tables: Vec<[Tile; TILES_PER_PATTERN_TABLE]> = Vec::new();

    for byte_array in byte_arrays {
        let mut pattern_table =
            [[(false, false); TILE_WIDTH * TILE_WIDTH]; TILES_PER_PATTERN_TABLE];

        for (tile, pixels) in pattern_table
            .iter_mut()
            .enumerate()
            .take(TILES_PER_PATTERN_TABLE)
        {
            let bytes_per_tile = TILE_WIDTH * PLANE_COUNT;
            let start_index = tile * bytes_per_tile;

            for row in 0..TILE_WIDTH {
                let low_bits = byte_array[start_index + row];
                let high_bits = byte_array[start_index + row + TILE_WIDTH];

                for col in 0..TILE_WIDTH {
                    let bit_mask = 0b_1000_0000 >> col;
                    let pattern: (bool, bool) =
                        ((high_bits & bit_mask) != 0, (low_bits & bit_mask) != 0);

                    let pixel_index = (row * TILE_WIDTH) + col;

                    pixels[pixel_index] = pattern;
                }
            }
        }

        pattern_tables.push(pattern_table)
    }

    let mut screen = Screen::new(
        (DEFAULT_SCREEN_WIDTH * 2) as u32,
        (DEFAULT_SCREEN_HEIGHT * 2) as u32,
        Color::RGB(153, 76, 0),
    );

    for (table_index, table) in pattern_tables.iter().enumerate() {
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

                let pixel_location = table_origin + tile_offset + pixel_offset + margin_offset;

                let color = match pixel {
                    (false, false) => Color::RGB(0, 0, 0),
                    (false, true) => Color::RGB(160, 160, 160),
                    (true, false) => Color::RGB(50, 50, 50),
                    (true, true) => Color::WHITE,
                };

                screen.set_pixel(pixel_location, color);
            }
        }
    }

    screen
}
