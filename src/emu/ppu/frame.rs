use std::{array, fmt};

use sdl2::{pixels::Color, rect::Point};

use crate::emu::ppu::{
    PPU,
    nametable::{NAMETABLE_SIZE, NAMETABLES_START_ADDR, Nametable},
    patterns::{
        PATTERN_HEIGHT_PIXELS, PATTERN_SIZE, PATTERN_WIDTH_PIXELS, PatternTable,
        get_pattern_from_nametable_entry,
    },
};
use std::fmt::Write;

pub const FRAME_WIDTH_PIXELS: u16 = 256;
pub const FRAME_HEIGHT_PIXELS: u16 = 240;

pub const PATTERN_ROWS_PER_FRAME: u16 = FRAME_HEIGHT_PIXELS / PATTERN_HEIGHT_PIXELS;
pub const PATTERN_COLS_PER_FRAME: u16 = FRAME_WIDTH_PIXELS / PATTERN_WIDTH_PIXELS;

#[derive(Clone)]
pub struct Frame {
    pixels: [[(u8, u8, u8); FRAME_WIDTH_PIXELS as usize]; FRAME_HEIGHT_PIXELS as usize],
}

impl Frame {
    pub const WIDTH_PIXELS: usize = FRAME_WIDTH_PIXELS as usize;
    pub const HEIGHT_PIXELS: usize = FRAME_HEIGHT_PIXELS as usize;
    pub const BYTES_PER_PIXEL: usize = 3;

    pub fn new(
        pixels: [[(u8, u8, u8); FRAME_WIDTH_PIXELS as usize]; FRAME_HEIGHT_PIXELS as usize],
    ) -> Self {
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

    /// Returns the pixel data for a frame as a flattened list of bytes.
    pub fn get_pixel_data(&self) -> Vec<u8> {
        let mut vec = Vec::new();

        for color in self.pixels.as_flattened().iter() {
            vec.push(color.0);
            vec.push(color.1);
            vec.push(color.2);
        }

        vec
    }
}

impl Default for Frame {
    fn default() -> Self {
        Frame::new([[(255, 0, 0); FRAME_WIDTH_PIXELS as usize]; FRAME_HEIGHT_PIXELS as usize])
    }
}

impl fmt::Debug for Frame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = "".to_string();

        for row in self.pixels.iter() {
            for pixel in row.iter() {
                write!(result, " ({},{},{})", pixel.0, pixel.1, pixel.2)?;
            }
            writeln!(result)?;
        }

        write!(f, "{result}")
    }
}

pub fn render_frame(ppu: &PPU) -> Frame {
    let pattern_table_addr = ppu.registers.ppu_ctrl.get_background_pattern_table_addr();

    let pixels = array::from_fn(|y| {
        array::from_fn(|x| {
            let stride = (y / PATTERN_HEIGHT_PIXELS as usize) * PATTERN_COLS_PER_FRAME as usize;
            let nametable_index = stride + (x / PATTERN_WIDTH_PIXELS as usize);
            let nametable_addr = NAMETABLES_START_ADDR + nametable_index as u16;

            let pattern_index = ppu.buses.read(nametable_addr);
            let pattern_addr = pattern_table_addr + (pattern_index as u16 * PATTERN_SIZE);

            let pattern_col_index = x % PATTERN_WIDTH_PIXELS as usize;
            let pattern_row_index = y % PATTERN_HEIGHT_PIXELS as usize;

            let pattern_row_addr = pattern_addr + pattern_row_index as u16;
            let lo_bits = ppu.buses.read(pattern_row_addr);
            let hi_bits = ppu.buses.read(pattern_row_addr + PATTERN_HEIGHT_PIXELS);

            let mask = 0b_1000_0000 >> pattern_col_index;
            let pixel = ((hi_bits & mask) != 0, (lo_bits & mask) != 0);

            match pixel {
                (false, false) => (32, 32, 32),
                (false, true) => (159, 159, 159),
                (true, false) => (96, 96, 96),
                (true, true) => (223, 223, 223),
            }
        })
    });

    Frame::new(pixels)
}

pub fn render_nametable(nametable: &Nametable) -> Frame {
    let mut frame = Frame::default();

    for (i, entry) in nametable.iter().enumerate() {
        if NAMETABLE_SIZE <= i as u16 {
            break;
        }

        let pattern = get_pattern_from_nametable_entry(*entry);

        let pattern_y_offset = PATTERN_HEIGHT_PIXELS * (i as u16 / PATTERN_COLS_PER_FRAME);
        let pattern_x_offset = PATTERN_WIDTH_PIXELS * (i as u16 % PATTERN_COLS_PER_FRAME);

        for (row, row_of_pixels) in pattern.data.iter().enumerate() {
            let y = pattern_y_offset + row as u16;

            for (col, pixel) in row_of_pixels.iter().enumerate() {
                let x = pattern_x_offset + col as u16;

                let color = match pixel {
                    (false, false) => Color::RGB(32, 32, 32),
                    (false, true) => Color::RGB(159, 159, 159),
                    (true, false) => Color::RGB(96, 96, 96),
                    (true, true) => Color::RGB(223, 223, 223),
                };

                frame.set_pixel(Point::new(x as i32, y as i32), color);
            }
        }
    }

    frame
}

pub fn render_pattern_table(pattern_table: &PatternTable) -> Frame {
    let mut frame =
        Frame::new([[(0, 0, 0); FRAME_WIDTH_PIXELS as usize]; FRAME_HEIGHT_PIXELS as usize]);

    const MARGIN_WIDTH_PIXELS: u16 = 3;
    const PATTERNS_PER_ROW: u16 = 16;

    for (pattern_index, pattern) in pattern_table.data.iter().enumerate() {
        let pattern_y_offset = MARGIN_WIDTH_PIXELS
            + ((PATTERN_HEIGHT_PIXELS + MARGIN_WIDTH_PIXELS)
                * (pattern_index as u16 / PATTERNS_PER_ROW));
        let pattern_x_offset = MARGIN_WIDTH_PIXELS
            + ((PATTERN_WIDTH_PIXELS + MARGIN_WIDTH_PIXELS)
                * (pattern_index as u16 % PATTERNS_PER_ROW));

        for (row, row_of_pixels) in pattern.data.iter().enumerate() {
            let y = pattern_y_offset + row as u16;

            for (col, pixel) in row_of_pixels.iter().enumerate() {
                let x = pattern_x_offset + col as u16;

                let color = match pixel {
                    (false, false) => Color::RGB(32, 32, 32),
                    (false, true) => Color::RGB(159, 159, 159),
                    (true, false) => Color::RGB(96, 96, 96),
                    (true, true) => Color::RGB(223, 223, 223),
                };

                frame.set_pixel(Point::new(x as i32, y as i32), color);
            }
        }
    }

    frame
}
