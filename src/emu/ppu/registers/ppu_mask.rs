const GREYSCALE_MASK: u8 = 0b_0000_0001;
const SHOW_BACKGROUND_MASK: u8 = 0b_0000_0010;
const SHOW_SPRITES_MASK: u8 = 0b_0000_0100;
const ENABLE_BACKGROUND_RENDERING_MASK: u8 = 0b_0000_1000;
const ENABLE_SPRITE_RENDERING_MASK: u8 = 0b_0001_0000;
const EMPHASIZE_RED_MASK: u8 = 0b_0010_0000;
const EMPHASIZE_GREEN_MASK: u8 = 0b_0100_0000;
const EMPHASIZE_BLUE_MASK: u8 = 0b_1000_0000;

#[derive(Default, Copy, Clone)]
pub struct PpuMask {
    pub data: u8,
}

impl PpuMask {
    /// Returns the PPU color mode, where `true` is greyscale and `false` is
    /// normal color.
    pub fn is_greyscale(&self) -> bool {
        self.data & GREYSCALE_MASK != 0
    }

    /// Returns `true` to show background in the leftmost 8 pixels of the
    /// screen.
    pub fn show_background(&self) -> bool {
        self.data & SHOW_BACKGROUND_MASK != 0
    }

    /// Returns `true` to show sprites in the leftmost 8 pixels of the screen.
    pub fn show_sprites(&self) -> bool {
        self.data & SHOW_SPRITES_MASK != 0
    }

    /// Returns `true` if background rendering is enabled.
    pub fn is_background_rendering_enabled(&self) -> bool {
        self.data & ENABLE_BACKGROUND_RENDERING_MASK != 0
    }

    /// Returns `true` if sprite rendering is enabled
    pub fn is_sprite_rendering_enabled(&self) -> bool {
        self.data & ENABLE_SPRITE_RENDERING_MASK != 0
    }

    /// Returns `true` for red emphasis.
    pub fn emphasize_red(&self) -> bool {
        self.data & EMPHASIZE_RED_MASK != 0
    }

    /// Returns `true` for green emphasis.
    pub fn emphasize_green(&self) -> bool {
        self.data & EMPHASIZE_GREEN_MASK != 0
    }

    /// Returns `true` for blue emphasis.
    pub fn emphasize_blue(&self) -> bool {
        self.data & EMPHASIZE_BLUE_MASK != 0
    }
}
