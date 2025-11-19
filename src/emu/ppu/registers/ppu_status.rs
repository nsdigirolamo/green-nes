const PPU_OPEN_BUS_MASK: u8 = 0b_0001_1111;
const SPRITE_OVERFLOW_FLAG_MASK: u8 = 0b_0010_0000;
const SPRITE_ZERO_HIT_FLAG_MASK: u8 = 0b_0100_0000;
const VBLANK_FLAG_MASK: u8 = 0b_1000_0000;

#[derive(Default, Copy, Clone)]
pub struct PpuStatus {
    pub data: u8,
}

impl PpuStatus {
    /// Returns the PPU's open bus.
    pub fn get_ppu_open_bus(&self) -> u8 {
        self.data & PPU_OPEN_BUS_MASK
    }

    /// Returns `true` depending on sprite evaluation from the PPU.
    pub fn get_sprite_overflow_flag(&self) -> bool {
        self.data & SPRITE_OVERFLOW_FLAG_MASK != 0
    }

    /// Returns `true` if the first sprite in OAM collides with the background.
    pub fn get_sprite_zero_hit_flag(&self) -> bool {
        self.data & SPRITE_ZERO_HIT_FLAG_MASK != 0
    }

    /// Returns `true` after the start of vertical blanking.
    pub fn get_vblank_flag(&self) -> bool {
        self.data & VBLANK_FLAG_MASK != 0
    }

    /// Sets the vertical blanking flag to the given value
    pub fn set_vblank_flag(&mut self, val: bool) {
        self.data = match val {
            true => self.data | VBLANK_FLAG_MASK,
            false => self.data & (VBLANK_FLAG_MASK ^ 0b_1111_1111),
        }
    }
}
