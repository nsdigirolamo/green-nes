#[derive(Default)]
pub struct Registers {
    // External Registers
    pub ppu_ctrl: u8,   // Miscellaneous Settings
    pub ppu_mask: u8,   // Rendering Settings
    pub ppu_status: u8, // Render Events
    pub oam_addr: u8,   // Sprite RAM Address
    pub oam_data: u8,   // Sprite RAM Data
    pub ppu_scroll: u8, // X and Y Scroll
    pub ppu_addr: u8,   // VRAM Address
    pub ppu_data: u8,   // VRAM Data
    pub oam_dma: u8,    // Sprite DMA
    // Internal Registers
    pub v: u8,
    pub t: u8,
    pub x: u8,
    pub w: u8,
}
