use sdl3::pixels::Color;

const SCREEN_WIDTH: usize = 283;
const SCREEN_HEIGHT: usize = 242;

#[derive(Clone, Copy)]
pub struct Screen {
    pub width: usize,
    pub height: usize,
    pixels: [[Color; SCREEN_WIDTH]; SCREEN_HEIGHT],
}

impl Screen {
    pub fn draw_pixel(&mut self, location: (usize, usize), pixel: Color) {
        self.pixels[location.0][location.1] = pixel
    }
}

impl Default for Screen {
    fn default() -> Self {
        Screen {
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            pixels: [[Color::RED; SCREEN_WIDTH]; SCREEN_HEIGHT],
        }
    }
}
