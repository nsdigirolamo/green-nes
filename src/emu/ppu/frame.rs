use std::fmt;

use sdl2::{pixels::Color, rect::Point};

use crate::emu::ppu::PPU;
use std::fmt::Write;

pub const FRAME_WIDTH: usize = 256;
pub const FRAME_HEIGHT: usize = 240;

// const TILE_ROWS_PER_FRAME: u32 = (FRAME_HEIGHT / TILE_WIDTH) as u32;
// const TILE_COLS_PER_FRAME: u32 = (FRAME_WIDTH / TILE_WIDTH) as u32;

pub struct Frame {
    pixels: [[(u8, u8, u8); FRAME_WIDTH]; FRAME_HEIGHT],
}

impl Frame {
    pub const WIDTH: usize = FRAME_WIDTH;
    pub const HEIGHT: usize = FRAME_HEIGHT;
    pub const BYTES_PER_PIXEL: usize = 3;

    pub fn new(pixels: [[(u8, u8, u8); FRAME_WIDTH]; FRAME_HEIGHT]) -> Self {
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
        Frame::new([[(255, 0, 0); FRAME_WIDTH]; FRAME_HEIGHT])
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
    let color = match ppu.scanline_index % 3 {
        0 => (255, 0, 0),
        1 => (0, 255, 0),
        2 => (0, 0, 255),
        _ => (255, 255, 255),
    };

    Frame::new([[color; Frame::WIDTH]; Frame::HEIGHT])

    // let mut frame = Frame::default();

    // // Get the address for the background pattern table.
    // let background_pattern_table_addr = ppu.registers.ppu_ctrl.get_background_pattern_table_addr();
    // // 16 bytes per tile.
    // let bytes_per_tile = TILE_WIDTH * PLANE_COUNT;

    // for tile_index in 0..NAMETABLE_SIZE {
    //     // Get the pattern index from the nametable.
    //     // For now, we're only concerned with the contents of the first nametable.
    //     let pattern_index = ppu.buses.read(NAMETABLE_START_ADDR + tile_index);
    //     // Get start address of pattern data.
    //     let start_addr =
    //         background_pattern_table_addr + (pattern_index as u16 * bytes_per_tile as u16);

    //     for tile_row in 0..TILE_WIDTH {
    //         let low_bits = ppu.buses.read(start_addr + tile_row as u16);
    //         let high_bits = ppu
    //             .buses
    //             .read(start_addr + TILE_WIDTH as u16 + tile_row as u16);

    //         for tile_col in 0..TILE_WIDTH {
    //             let x = ((tile_index % TILE_COLS_PER_FRAME as u16) * TILE_WIDTH as u16)
    //                 + tile_col as u16;
    //             let y = ((tile_index / TILE_COLS_PER_FRAME as u16) * TILE_WIDTH as u16)
    //                 + tile_row as u16;

    //             let location = Point::new(x as i32, y as i32);

    //             let bit_mask = 0b_1000_0000 >> tile_col;
    //             let pixel_pattern: (bool, bool) =
    //                 ((high_bits & bit_mask) == 1, (low_bits & bit_mask) == 1);

    //             let color = match pixel_pattern {
    //                 (false, false) => Color::BLACK,
    //                 (false, true) => Color::RGB(160, 160, 160),
    //                 (true, false) => Color::RGB(50, 50, 50),
    //                 (true, true) => Color::WHITE,
    //             };

    //             frame.set_pixel(location, color);
    //         }
    //     }
    // }

    // frame
}
