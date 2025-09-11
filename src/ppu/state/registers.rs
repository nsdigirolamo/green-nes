#[derive(Default)]
pub struct Registers {
    // External Registers
    pub ppu_ctrl: u8,
    pub ppu_mask: u8,
    pub ppu_status: u8,
    pub oam_addr: u8,
    pub oam_data: u8,
    pub ppu_scroll: u8,
    pub ppu_addr: u8,
    pub ppu_data: u8,
    pub oam_dma: u8,
    // Internal Registers
    pub v: u8,
    pub t: u8,
    pub x: u8,
    pub w: u8,
}
