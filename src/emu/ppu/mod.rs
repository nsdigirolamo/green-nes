use crate::emu::{
    cartridge::Cartridge,
    ppu::{
        buses::Buses,
        frame::{Frame, render_frame},
        registers::Registers,
    },
};

pub mod buses;
pub mod frame;
pub mod nametable;
pub mod palettes;
pub mod patterns;
pub mod registers;

const OAM_SPRITE_SIZE: usize = 4;
const OAM_SPRITE_COUNT: u32 = 64;
const OAM_SIZE: usize = OAM_SPRITE_SIZE * OAM_SPRITE_COUNT as usize;

const PPU_CYCLES_PER_SCANLINE: u32 = 341;
const VBLANK_LINE_INDEX: u32 = 241;
const PRERENDER_LINE_INDEX: u32 = 261;

pub struct PPU {
    registers: Registers,
    buses: Buses,
    oam: [u8; OAM_SIZE],
    ppu_data_read_buffer: u8,
    nmi_pin: bool, // True = NMI pin is pulled low (and thus a NMI occurs).
    cycle_count: u32,
    scanline_index: u32,
    frame: Option<Frame>,
}

impl PPU {
    pub fn new(cart: Cartridge) -> Self {
        PPU {
            registers: Registers::default(),
            buses: Buses::new(cart),
            oam: [0; OAM_SIZE],
            ppu_data_read_buffer: 0,
            nmi_pin: false,
            cycle_count: 0,
            scanline_index: 0,
            frame: None,
        }
    }

    pub fn tick(&mut self) {
        // PPU clock runs three times faster than the CPU clock.
        self.cycle_count += 3;

        if PPU_CYCLES_PER_SCANLINE <= self.cycle_count {
            self.scanline_index += 1;
            self.cycle_count -= PPU_CYCLES_PER_SCANLINE;

            if self.scanline_index == VBLANK_LINE_INDEX {
                self.registers.ppu_status.set_vblank_flag(true);
                self.update_nmi();
                self.frame = Some(render_frame(self));
            } else if self.scanline_index == PRERENDER_LINE_INDEX {
                self.registers.ppu_status.set_vblank_flag(false);
                self.update_nmi();
            } else if PRERENDER_LINE_INDEX < self.scanline_index {
                self.scanline_index = 0;
            }
        }
    }

    pub fn take_frame(&mut self) -> Option<Frame> {
        self.frame.take()
    }

    /// Sets the appropriate state of the NMI pin depending on the state of
    /// the appropriate bits in the PPUSTATUS and PPUCTRL registers.
    fn update_nmi(&mut self) {
        self.nmi_pin = self.registers.ppu_status.get_vblank_flag()
            && self.registers.ppu_ctrl.is_vblank_nmi_enabled();
    }

    /// Returns the value of the NMI pin, where True means the pin is pulled low
    /// (and thus a NMI occurs).
    pub fn get_nmi(&self) -> bool {
        self.nmi_pin
    }

    /// Writes a byte to PPUCTRL.
    pub fn write_ppu_ctrl(&mut self, data: u8) {
        self.registers.ppu_ctrl.data = data;
        self.update_nmi();
    }

    /// Writes a byte to PPUMASK.
    pub fn write_ppu_mask(&mut self, data: u8) {
        self.registers.ppu_mask.data = data;
    }

    /// Returns the byte stored in the PPUSTATUS register. This has the
    /// side effect of clearing both the internal `w` register and the internal
    /// vertical blanking flag.
    pub fn read_ppu_status(&mut self) -> u8 {
        let data = self.registers.ppu_status.data;

        self.registers.internal.w = false;
        self.registers.ppu_status.set_vblank_flag(false);
        self.update_nmi();

        data
    }

    /// Returns the byte stored in the PPUSTATUS register without side effects.
    pub fn peek_ppu_status(&self) -> u8 {
        self.registers.ppu_status.data
    }

    /// Writes a byte to OAMADDR.
    pub fn write_oam_addr(&mut self, data: u8) {
        self.registers.oam_addr.data = data;
    }

    /// Returns a byte from Object Attribute Memory (OAM) addressed by the value
    /// in OAMADDR.
    pub fn read_oam_data(&self) -> u8 {
        let addr = self.registers.oam_addr.data;

        self.oam[addr as usize]
    }

    /// Writes a byte to Object Attribute Memory (OAM) addressed by the value in
    /// OAMADDR. This has the side effect of incrementing OAMADDR.
    pub fn write_oam_data(&mut self, data: u8) {
        let addr = self.registers.oam_addr.data;

        self.oam[addr as usize] = data;
        self.registers.oam_addr.data += 1;
    }

    /// Writes a byte to the scroll position. This has the side effect of
    /// updating the internal `t`, `x`, and `w` registers.
    pub fn write_ppu_scroll(&mut self, data: u8) {
        let is_first_write = !self.registers.internal.w;

        if is_first_write {
            let coarse_x_scroll = (data >> 3) as u16;
            let fine_x_scroll = data & 0b_0000_0111;

            self.registers.internal.t |= coarse_x_scroll;
            self.registers.internal.x |= fine_x_scroll;
            self.registers.internal.w = true;
        } else {
            let coarse_y_scroll = ((data >> 3) as u16) << 5;
            let fine_y_scroll = ((data & 0b_0000_0111) as u16) << 12;

            self.registers.internal.t |= coarse_y_scroll;
            self.registers.internal.t |= fine_y_scroll;
            self.registers.internal.w = false;
        }
    }

    /// Writes a byte to the PPU address. This has the side effect of updating
    /// the internal `t`, `w`, and `v` registers.
    pub fn write_ppu_addr(&mut self, data: u8) {
        let is_first_write = !self.registers.internal.w;

        if is_first_write {
            let high_byte = ((data & 0b_0011_1111) as u16) << 8;

            self.registers.internal.t |= high_byte;
            self.registers.internal.t &= 0b_0011_1111_1111_1111;
            self.registers.internal.w = true;
        } else {
            let low_byte = data as u16;

            self.registers.internal.t |= low_byte;
            self.registers.internal.w = false;

            // TODO: This should only happen 1 to 1.5 dots after the write.
            // https://www.nesdev.org/wiki/PPU_programmer_reference#PPUADDR
            // https://www.nesdev.org/wiki/PPU_scrolling#PPU_internal_registers
            self.registers.internal.v = self.registers.internal.t
        }
    }

    /// Returns a byte from the PPUDATA read buffer. The buffer will be filled
    /// with the next byte from the PPU's 14-bit address space. This also has
    /// the side effect of updating the internal `v` register.
    pub fn read_ppu_data(&mut self) -> u8 {
        // First, read from the data buffer.
        let data = self.ppu_data_read_buffer;

        // Next, read from the internal v register to get current VRAM address.
        let addr = self.registers.internal.v;
        self.ppu_data_read_buffer = self.buses.read(addr);

        // Finally, increment the address by the value specified in PPUCTRL
        // and store the new value back into the internal v register.
        let addr_incr = self.registers.ppu_ctrl.get_vram_addr_incr();
        let new_addr = addr.wrapping_add(addr_incr);
        self.registers.internal.v = new_addr;

        data
    }

    /// Writes a byte to the PPU's 14-bit address space.
    pub fn write_ppu_data(&mut self, data: u8) {
        let addr = self.registers.internal.v;
        self.buses.write(addr, data);
    }

    /// Returns a byte from the PPUDATA read buffer without side effects.
    pub fn peek_ppu_data(&self) -> u8 {
        self.ppu_data_read_buffer
    }

    /// Writes a page of memory to the Object Attribute Memory (OAM).
    pub fn write_oam_dma(&mut self, _data: u8) {
        // TODO: This is kind of complicated and uses a buffer of bytes.
        // https://www.nesdev.org/wiki/PPU_programmer_reference#OAMDMA
        todo!("write to OAMDMA is not implemented")
    }
}
