use crate::emu::ppu::{
    PPU,
    mappings::{
        NAMETABLE_SIZE, NAMETABLES_START_ADDR, PALETTE_RAM_BACKGROUND_START_ADDR, PALETTE_SIZE,
    },
    palettes::PALETTE_TABLE,
};
use sdl2::{pixels::Color, rect::Point};
use std::array;

const PATTERN_WIDTH: u16 = 8;
const PATTERN_HEIGHT: u16 = 8;

const PATTERN_PLANE_COUNT: u16 = 2;
const PATTERN_SIZE: u16 = PATTERN_HEIGHT * PATTERN_PLANE_COUNT;
const PATTERN_COLS_PER_FRAME: u16 = FRAME_WIDTH / PATTERN_WIDTH;

#[derive(Clone, Copy, Default)]
pub struct Pattern {
    pub data: [[(bool, bool); PATTERN_WIDTH as usize]; PATTERN_HEIGHT as usize],
}

const FRAME_WIDTH: u16 = 256;
const FRAME_HEIGHT: u16 = 240;
const FRAME_BYTES_PER_PIXEL: u16 = 3;

#[derive(Clone, Copy)]
pub struct Frame {
    pixels: [[(u8, u8, u8); FRAME_WIDTH as usize]; FRAME_HEIGHT as usize],
}

impl Frame {
    pub const WIDTH: usize = FRAME_WIDTH as usize;
    pub const HEIGHT: usize = FRAME_HEIGHT as usize;
    pub const BYTES_PER_PIXEL: usize = FRAME_BYTES_PER_PIXEL as usize;

    pub fn new(pixels: [[(u8, u8, u8); FRAME_WIDTH as usize]; FRAME_HEIGHT as usize]) -> Self {
        Frame { pixels }
    }

    /// Returns the color of the pixel at the given point.
    ///
    /// # Arguments
    ///
    /// * `location`: The (x,y) location of the pixel.
    ///
    /// # Examples
    ///
    /// ```Rust
    /// let frame = Frame::new();
    /// let color1 = frame.get_pixel(Point::new(0, 0)); // Returns the color of the top leftmost pixel in the frame.
    /// let color2 = frame.get_pixel(Point::new(Frame::WIDTH, Frame::HEIGHT)); // Returns the color of the bottom rightmost pixel in the frame.
    /// ```
    ///
    pub fn get_pixel(&self, location: Point) -> Color {
        let pixel = self.pixels[location.y as usize][location.x as usize];

        Color::RGB(pixel.0, pixel.1, pixel.2)
    }

    /// Sets the color of the pixel at the given point.
    ///
    /// # Arguments
    ///
    /// * `location`: The (x,y) location of the pixel.
    ///
    /// # Examples
    ///
    /// ```Rust
    /// let frame = Frame::new();
    /// let color1 = frame.set_pixel(Point::new(0, 0), Color::GREEN); // Sets the color of the top leftmost pixel to green.
    /// let color2 = frame.set_pixel(Point::new(Frame::WIDTH, Frame::HEIGHT), Color::GREEN); // Sets the color of the bottom rightmost pixel to green.
    /// ```
    ///
    pub fn set_pixel(&mut self, location: Point, color: Color) {
        let pixel = (color.r, color.g, color.b);
        self.pixels[location.y as usize][location.x as usize] = pixel;
    }

    /// Returns the pixel data for a frame as a flattened array of bytes.
    pub fn get_pixel_data(
        &self,
    ) -> [u8; FRAME_WIDTH as usize * FRAME_HEIGHT as usize * FRAME_BYTES_PER_PIXEL as usize] {
        let flattened = self.pixels.as_flattened();
        array::from_fn(|i| {
            let pixel = flattened[i / 3];
            match i % 3 {
                0 => pixel.0,
                1 => pixel.1,
                2 => pixel.2,
                _ => unreachable!("mod 3 is not greater than 2"),
            }
        })
    }
}

impl Default for Frame {
    fn default() -> Self {
        Frame::new([[(0, 0, 0); FRAME_WIDTH as usize]; FRAME_HEIGHT as usize])
    }
}

const ATTRIBUTE_AREA_HEIGHT: u16 = PATTERN_HEIGHT * PATTERN_ROWS_PER_ATTRIBUTE_AREA;
const ATTRIBUTE_AREA_WIDTH: u16 = PATTERN_WIDTH * PATTERN_COLS_PER_ATTRIBUTE_AREA;
const PATTERN_ROWS_PER_ATTRIBUTE_AREA: u16 = 4;
const PATTERN_COLS_PER_ATTRIBUTE_AREA: u16 = 4;
const ATTRIBUTE_AREA_COLS_PER_FRAME: u16 = PATTERN_COLS_PER_FRAME / PATTERN_COLS_PER_ATTRIBUTE_AREA;

impl PPU {
    pub fn render_frame(&mut self) {
        let pattern_table_addr = self.registers.ppu_ctrl.get_background_pattern_table_addr();
        let nametable_addr = NAMETABLES_START_ADDR;

        let pixels = array::from_fn(|y| {
            array::from_fn(|x| {
                let nametable_stride =
                    (y / PATTERN_HEIGHT as usize) * PATTERN_COLS_PER_FRAME as usize;
                let nametable_index = nametable_stride + (x / PATTERN_WIDTH as usize);
                let pattern_index = self.buses.read(nametable_addr + nametable_index as u16);

                let pattern_row_index = y % PATTERN_HEIGHT as usize;
                let pattern_col_index = x % PATTERN_WIDTH as usize;

                let attribute_stride =
                    (y / ATTRIBUTE_AREA_HEIGHT as usize) * ATTRIBUTE_AREA_COLS_PER_FRAME as usize;
                let attribute_index = attribute_stride + (x / ATTRIBUTE_AREA_WIDTH as usize);
                let attribute = self
                    .buses
                    .read(nametable_addr + NAMETABLE_SIZE + attribute_index as u16);

                let palette_index = match (
                    ((pattern_row_index % PATTERN_ROWS_PER_ATTRIBUTE_AREA as usize) / 2),
                    ((pattern_col_index % PATTERN_COLS_PER_ATTRIBUTE_AREA as usize) / 2),
                ) {
                    (0, 0) => attribute & 0b_0000_0011,
                    (0, 1) => (attribute >> 2) & 0b_0000_0011,
                    (1, 0) => (attribute >> 4) & 0b_0000_0011,
                    (1, 1) => (attribute >> 6) & 0b_0000_0011,
                    idx => unreachable!("unreachable palette index: ({}, {})", idx.0, idx.1),
                };

                let palette0 = self.buses.read(
                    PALETTE_RAM_BACKGROUND_START_ADDR + (palette_index as u16 * PALETTE_SIZE),
                );
                let palette1 = self.buses.read(
                    PALETTE_RAM_BACKGROUND_START_ADDR + (palette_index as u16 * PALETTE_SIZE) + 1,
                );
                let palette2 = self.buses.read(
                    PALETTE_RAM_BACKGROUND_START_ADDR + (palette_index as u16 * PALETTE_SIZE) + 2,
                );
                let palette3 = self.buses.read(
                    PALETTE_RAM_BACKGROUND_START_ADDR + (palette_index as u16 * PALETTE_SIZE) + 3,
                );

                let pattern_addr = pattern_table_addr + (pattern_index as u16 * PATTERN_SIZE);
                let pattern_row_addr = pattern_addr + pattern_row_index as u16;
                let lo_bits = self.buses.read(pattern_row_addr);
                let hi_bits = self.buses.read(pattern_row_addr + PATTERN_HEIGHT);

                let mask = 0b_1000_0000 >> pattern_col_index;
                let pixel = ((hi_bits & mask) != 0, (lo_bits & mask) != 0);

                match pixel {
                    (false, false) => PALETTE_TABLE[palette0 as usize],
                    (false, true) => PALETTE_TABLE[palette1 as usize],
                    (true, false) => PALETTE_TABLE[palette2 as usize],
                    (true, true) => PALETTE_TABLE[palette3 as usize],
                }
            })
        });

        self.frame = Frame::new(pixels);
        self.frame_ready = true;
    }
}
