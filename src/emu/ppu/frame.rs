use crate::emu::ppu::{BYTES_PER_PATTERN, OAM_SIZE, PPU, palettes::PALETTE_TABLE};
use sdl2::{pixels::Color, rect::Point};
use std::array;

pub const PATTERN_WIDTH: u16 = 8;
pub const PATTERN_HEIGHT: u16 = 8;

pub const PATTERN_PLANE_COUNT: u16 = 2;
pub const PATTERN_SIZE: u16 = PATTERN_HEIGHT * PATTERN_PLANE_COUNT;
pub const PATTERN_COLS_PER_FRAME: u16 = FRAME_WIDTH / PATTERN_WIDTH;

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

pub const ATTRIBUTE_AREA_HEIGHT: u16 = PATTERN_HEIGHT * PATTERN_ROWS_PER_ATTRIBUTE_AREA;
pub const ATTRIBUTE_AREA_WIDTH: u16 = PATTERN_WIDTH * PATTERN_COLS_PER_ATTRIBUTE_AREA;
pub const PATTERN_ROWS_PER_ATTRIBUTE_AREA: u16 = 4;
pub const PATTERN_COLS_PER_ATTRIBUTE_AREA: u16 = 4;
pub const ATTRIBUTE_AREA_COLS_PER_FRAME: u16 =
    PATTERN_COLS_PER_FRAME / PATTERN_COLS_PER_ATTRIBUTE_AREA;

impl PPU {
    pub fn render_frame(&mut self) {
        let mut frame = self.create_background();
        self.draw_sprites(&mut frame);
        self.frame = frame;
        self.frame_ready = true;
    }

    fn draw_sprites(&self, frame: &mut Frame) {
        let sprite_pattern_table_addr = self.registers.ppu_ctrl.get_sprite_pattern_table_addr();

        for i in 0..(OAM_SIZE / BYTES_PER_PATTERN) {
            let oam_addr = i * BYTES_PER_PATTERN;

            let y_pos = self.oam[oam_addr];
            let pattern_index = self.oam[oam_addr + 1];
            let attribute = self.oam[oam_addr + 2];
            let x_pos = self.oam[oam_addr + 3];

            let palette_index = attribute & 0b_0000_0011;
            let palette = self.get_sprite_palette(palette_index);

            let pattern_addr = sprite_pattern_table_addr + (pattern_index as u16 * PATTERN_SIZE);

            let flip_horz = (attribute & 0b_0100_0000) != 0;
            let flip_vert = (attribute & 0b_1000_0000) != 0;

            for row in 0..PATTERN_HEIGHT {
                for col in 0..PATTERN_WIDTH {
                    let x = x_pos as u16 + col;
                    let y = y_pos as u16 + row;

                    let pattern_row_addr =
                        pattern_addr + if flip_vert { PATTERN_HEIGHT - row } else { row };
                    let lo_bits = self.buses.read(pattern_row_addr);
                    let hi_bits = self.buses.read(pattern_row_addr + PATTERN_HEIGHT);

                    let mask = if flip_horz {
                        0b_0000_0001 << col
                    } else {
                        0b_1000_0000 >> col
                    };
                    let pixel = ((hi_bits & mask) != 0, (lo_bits & mask) != 0);

                    if FRAME_WIDTH <= x || FRAME_HEIGHT <= y {
                        continue;
                    }

                    let color = match pixel {
                        (false, false) => PALETTE_TABLE[palette.0 as usize],
                        (false, true) => PALETTE_TABLE[palette.1 as usize],
                        (true, false) => PALETTE_TABLE[palette.2 as usize],
                        (true, true) => PALETTE_TABLE[palette.3 as usize],
                    };

                    frame.set_pixel(
                        Point::new(x as i32, y as i32),
                        Color::RGB(color.0, color.1, color.2),
                    );
                }
            }
        }
    }

    fn create_background(&self) -> Frame {
        let background_pattern_table_addr =
            self.registers.ppu_ctrl.get_background_pattern_table_addr();

        Frame::new(array::from_fn(|y| {
            array::from_fn(|x| {
                let pattern_index = self.get_pattern_index(x as u16, y as u16);
                let pattern_row_index = y % PATTERN_HEIGHT as usize;
                let pattern_col_index = x % PATTERN_WIDTH as usize;

                let attribute_byte = self.get_attribute_byte(x as u16, y as u16);
                let palette_index = match (
                    ((pattern_row_index % PATTERN_ROWS_PER_ATTRIBUTE_AREA as usize) / 2),
                    ((pattern_col_index % PATTERN_COLS_PER_ATTRIBUTE_AREA as usize) / 2),
                ) {
                    (0, 0) => attribute_byte & 0b_0000_0011,
                    (0, 1) => (attribute_byte >> 2) & 0b_0000_0011,
                    (1, 0) => (attribute_byte >> 4) & 0b_0000_0011,
                    (1, 1) => (attribute_byte >> 6) & 0b_0000_0011,
                    idx => unreachable!("unreachable palette index: ({}, {})", idx.0, idx.1),
                };

                let palette = self.get_background_palette(palette_index);

                let pattern_addr =
                    background_pattern_table_addr + (pattern_index as u16 * PATTERN_SIZE);
                let pattern_row_addr = pattern_addr + pattern_row_index as u16;
                let lo_bits = self.buses.read(pattern_row_addr);
                let hi_bits = self.buses.read(pattern_row_addr + PATTERN_HEIGHT);

                let mask = 0b_1000_0000 >> pattern_col_index;
                let pixel = ((hi_bits & mask) != 0, (lo_bits & mask) != 0);

                match pixel {
                    (false, false) => PALETTE_TABLE[palette.0 as usize],
                    (false, true) => PALETTE_TABLE[palette.1 as usize],
                    (true, false) => PALETTE_TABLE[palette.2 as usize],
                    (true, true) => PALETTE_TABLE[palette.3 as usize],
                }
            })
        }))
    }
}
