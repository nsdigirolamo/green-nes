const BASE_NAMETABLE_ADDRESS_MASK: u8 = 0b_0000_0011;
const VRAM_ADDRESS_INCREMENT_MASK: u8 = 0b_0000_0100;
const SPRITE_PATTERN_TABLE_ADDRESS_MASK: u8 = 0b_0000_1000;
const BACKGROUND_PATTERN_TABLE_ADDRESS_MASK: u8 = 0b_0001_0000;
const SPRITE_SIZE_MASK: u8 = 0b_0010_0000;
const PPU_SELECT_MASK: u8 = 0b_0100_0000;
const VBLANK_NMI_ENABLE_MASK: u8 = 0b_1000_0000;

#[derive(Default, Copy, Clone)]
pub struct PpuControl {
    pub data: u8,
}

impl PpuControl {
    /// Returns the base nametable address.
    pub fn get_base_nametable_addr(&self) -> u16 {
        match self.data & BASE_NAMETABLE_ADDRESS_MASK {
            0 => 0x2000,
            1 => 0x2400,
            2 => 0x2800,
            3 => 0x2C00,
            _ => unreachable!("ppu control base nametable address should always be less than 3"),
        }
    }

    /// Returns the VRAM address increment amount per read/write of `PPUDATA`.
    pub fn get_vram_addr_incr(&self) -> u16 {
        match self.data & VRAM_ADDRESS_INCREMENT_MASK != 0 {
            true => 32,
            false => 1,
        }
    }

    /// Returns the sprite pattern table address for 8x8 sprites (ignored in
    /// 8x16 mode).
    pub fn get_sprite_pattern_table_addr(&self) -> u16 {
        match self.data & SPRITE_PATTERN_TABLE_ADDRESS_MASK != 0 {
            true => 0x1000,
            false => 0x0000,
        }
    }

    /// Returns the background pattern table address.
    pub fn get_background_pattern_table_addr(&self) -> u16 {
        match self.data & BACKGROUND_PATTERN_TABLE_ADDRESS_MASK != 0 {
            true => 0x1000,
            false => 0x0000,
        }
    }

    /// Returns the sprite size, where `false` is 8x8 and `true` is 8x16.
    pub fn is_sprite_size_large(&self) -> bool {
        self.data & SPRITE_SIZE_MASK != 0
    }

    /// Returns the PPU EXT pin selection, where `false` is backdrop and `true`
    /// is output color.
    pub fn is_ext_pin_output_color(&self) -> bool {
        self.data & PPU_SELECT_MASK != 0
    }

    /// Returns `true` if the vertical blanking non-maskable interrupt is
    /// enabled.
    pub fn is_vblank_nmi_enabled(&self) -> bool {
        self.data & VBLANK_NMI_ENABLE_MASK != 0
    }
}
