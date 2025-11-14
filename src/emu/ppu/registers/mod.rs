use crate::emu::ppu::registers::{
    oam_addr::OamAddr, oam_data::OamData, oam_dma::OamDma, ppu_addr::PpuAddr, ppu_ctrl::PpuControl,
    ppu_data::PpuData, ppu_mask::PpuMask, ppu_scroll::PpuScroll, ppu_status::PpuStatus,
};

pub mod oam_addr;
pub mod oam_data;
pub mod oam_dma;
pub mod ppu_addr;
pub mod ppu_ctrl;
pub mod ppu_data;
pub mod ppu_mask;
pub mod ppu_scroll;
pub mod ppu_status;

/*
While Rendering:
- `v`: Scroll position.
- `t`: Starting coarse x scroll for the next scanline and y scroll for the screen.
- `x`: fine x scroll of the current scroll, used alongside `v`.

Outside of Rendering:
- `v`: Current VRAM address.
- `t`: Holds the scroll, or VRAM address before transferring to `v`.

Other:
- `w`: Write latch. Toggles on each write to PPUSCROLL or PPUADDR. Clears on
       reads to PPUSTATUS.
*/
#[derive(Default, Copy, Clone)]
pub struct InternalRegisters {
    pub v: (u8, u8),
    pub t: (u8, u8),
    pub x: u8,
    pub w: bool,
}

#[derive(Default, Copy, Clone)]
pub struct Registers {
    pub ppu_ctrl: PpuControl,        // Miscellaneous Settings (0x2000 W)
    pub ppu_mask: PpuMask,           // Rendering Settings (0x2001 W)
    pub ppu_status: PpuStatus,       // Rendering Events (0x2002 R)
    pub oam_addr: OamAddr,           // Sprite RAM Address (0x2003 W)
    pub oam_data: OamData,           // Spite RAM Data (0x2004 W)
    pub ppu_scroll: PpuScroll,       // X and Y Scroll (0x2005 Wx2)
    pub ppu_addr: PpuAddr,           // VRAM Address (0x2006 Wx2)
    pub ppu_data: PpuData,           // VRAM Data (0x2007 RW)
    pub oam_dma: OamDma,             // Spite DMA (0x4014 W)
    pub internal: InternalRegisters, // Internal Registers
}
