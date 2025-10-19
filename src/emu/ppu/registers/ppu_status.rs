#[derive(Default)]
pub struct PpuStatus {
    pub data: u8,
}

impl PpuStatus {
    /// Gets the PPU open bus value.
    pub fn get_ppu_open_bus(&self) -> u8 {
        self.data & 0b_0001_1111
    }

    /// Gets the sprite overflow flag.
    pub fn get_sprite_overflow_flag(&self) -> bool {
        (self.data >> 5) & 0b_0000_0001 == 1
    }

    /// Gets the sprite zero hit flag.
    pub fn get_sprite_zero_hit_flag(&self) -> bool {
        (self.data >> 6) & 0b_0000_0001 == 1
    }

    /// Gets the vblank flag.
    pub fn get_vblank_flag(&self) -> bool {
        self.data >> 7 == 1
    }
}
