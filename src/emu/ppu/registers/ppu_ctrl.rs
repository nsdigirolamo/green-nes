#[derive(Default, Copy, Clone)]
pub struct PpuControl {
    pub data: u8,
}

impl PpuControl {
    /// Gets the base nametable address.
    pub fn get_base_nametable_addr(&self) -> u16 {
        match self.data & 0b_0000_0011 {
            0 => 0x2000,
            1 => 0x2400,
            2 => 0x2800,
            3 => 0x2C00,
            _ => unreachable!("ppu control base nametable address should always be less than 3"),
        }
    }

    /// Gets the VRAM address increment value.
    pub fn get_vram_addr_incr(&self) -> u8 {
        match (self.data >> 2) & 0b_0000_0001 == 1 {
            true => 32,
            false => 1,
        }
    }

    /// Gets the sprite pattern table address.
    pub fn get_sprite_pattern_table_addr(&self) -> u16 {
        match (self.data >> 3) & 0b_0000_0001 == 1 {
            true => 0x1000,
            false => 0x0000,
        }
    }

    /// Gets the background pattern table address.
    pub fn get_background_pattern_table_addr(&self) -> u16 {
        match (self.data >> 4) & 0b_0000_0001 == 1 {
            true => 0x1000,
            false => 0x0000,
        }
    }

    /// Gets the sprite size. If true sprites are 8x16 and if false sprites are
    /// 8x8.
    pub fn is_sprite_size_large(&self) -> bool {
        (self.data >> 5) & 0b_0000_0001 == 1
    }

    /// Gets the PPU EXT pin selection. If true then output color on EXT pins
    /// and if false then read backdrop from EXT pins.
    pub fn is_ext_pin_output_color(&self) -> bool {
        (self.data >> 6) & 0b_0000_0001 == 1
    }

    /// Gets the vblank NMI status. If true then the NMI is enabled and if false
    /// NMI is disabled.
    pub fn is_vblank_nmi_enabled(&self) -> bool {
        self.data >> 7 == 1
    }
}
