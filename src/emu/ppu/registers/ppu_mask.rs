#[derive(Default)]
pub struct PpuMask {
    pub data: u8,
}

impl PpuMask {
    /// Gets the greyscale status.
    pub fn is_greyscale(&self) -> bool {
        self.data & 0b_0000_0001 == 1
    }

    /// Gets the background status. If true then show the background in the
    /// leftmost 8 pixels of the screen.
    pub fn show_background(&self) -> bool {
        (self.data >> 1) & 0b_0000_0001 == 1
    }

    /// Gets the sprite status. If true then show sprites in the leftmost 8
    /// pixels of the screen.
    pub fn show_sprites(&self) -> bool {
        (self.data >> 2) & 0b_0000_0001 == 1
    }

    /// Gets the background rendering status. If true then background rendering
    /// is enabled.
    pub fn enable_background_rendering(&self) -> bool {
        (self.data >> 3) & 0b_0000_0001 == 1
    }

    /// Gets the sprite rendering status. If true then sprite rendering is
    /// enabled.
    pub fn is_sprite_rendering_enabled(&self) -> bool {
        (self.data >> 4) & 0b_0000_0001 == 1
    }

    /// Gets the emphasize red status. If true then emphasize red.
    pub fn emphasize_red(&self) -> bool {
        (self.data >> 5) & 0b_0000_0001 == 1
    }

    /// Gets the emphasize green status. If true then emphasize green.
    pub fn emphasize_green(&self) -> bool {
        (self.data >> 6) & 0b_0000_0001 == 1
    }

    /// Gets the emphasize blue status. If true then emphasize blue.
    pub fn emphasize_blue(&self) -> bool {
        self.data >> 7 == 1
    }
}
