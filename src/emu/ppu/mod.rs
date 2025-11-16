use crate::{
    concat_u8,
    emu::{
        cartridge::Cartridge,
        ppu::{buses::Buses, registers::Registers},
    },
    split_u16,
};

pub mod buses;
pub mod nametable;
pub mod palettes;
pub mod registers;

#[derive(Clone)]
pub struct PPU {
    registers: Registers,
    buses: Buses,
    ppu_data_read_buffer: u8,
    is_vblank: bool,
    nmi: bool,
}

impl PPU {
    pub fn new(cart: Cartridge) -> Self {
        PPU {
            registers: Registers::default(),
            buses: Buses::new(cart),
            ppu_data_read_buffer: 0,
            is_vblank: false,
            nmi: false,
        }
    }

    pub fn get_registers(&self) -> Registers {
        self.registers
    }

    pub fn get_buses(&self) -> Buses {
        self.buses.clone()
    }

    pub fn get_data_read_buffer(&self) -> u8 {
        self.ppu_data_read_buffer
    }

    pub fn is_vblank(&self) -> bool {
        self.is_vblank
    }

    pub fn get_nmi(&self) -> bool {
        self.nmi
    }

    pub fn read_ppu_ctrl(&self) -> u8 {
        panic!("invalid read: PPUCTRL is write-only")
    }

    pub fn write_ppu_ctrl(&mut self, data: u8) {
        self.registers.ppu_ctrl.data = data;
    }

    pub fn read_ppu_mask(&self) -> u8 {
        panic!("invalid read: PPUMASK is write-only")
    }

    pub fn write_ppu_mask(&mut self, data: u8) {
        self.registers.ppu_mask.data = data;
    }

    pub fn read_ppu_status(&mut self) -> u8 {
        // Clear the vertical blanking flag and the internal w register.
        self.is_vblank = false;
        self.registers.internal.w = false;

        self.registers.ppu_status.data
    }

    pub fn write_ppu_status(&mut self, _: u8) {
        panic!("invalid write: PPUSTATUS is read-only")
    }

    pub fn read_oam_addr(&self) -> u8 {
        panic!("invalid read: OAMADDR is write-only")
    }

    pub fn write_oam_addr(&mut self, data: u8) {
        self.registers.oam_addr.data = data;
    }

    pub fn read_oam_data(&self) -> u8 {
        self.registers.oam_data.data
    }

    pub fn write_oam_data(&mut self, data: u8) {
        self.registers.oam_data.data = data;
    }

    pub fn read_ppu_scroll(&self) -> u8 {
        panic!("invalid read: PPUSCROLL is write-only")
    }

    pub fn write_ppu_scroll(&mut self, data: u8) {
        self.registers.ppu_scroll.data = data;
    }

    pub fn read_ppu_addr(&self) -> u8 {
        panic!("invalid read: PPUADDR is write-only")
    }

    pub fn write_ppu_addr(&mut self, data: u8) {
        let is_first_write = !self.registers.internal.w;

        if is_first_write {
            // PPU address space is 14-bit. Clear the two most significant bits.
            self.registers.internal.t.0 = data & 0b_0011_1111;
        } else {
            self.registers.internal.t.1 = data;
            self.registers.internal.v = self.registers.internal.t;
        }

        self.registers.internal.w = !self.registers.internal.w
    }

    pub fn read_ppu_data(&mut self) -> u8 {
        // First, read from the data buffer.
        let data = self.ppu_data_read_buffer;

        // Next, read from the internal v register to get current VRAM address.
        let v = self.registers.internal.v;
        let addr = concat_u8!(v.0, v.1);
        self.ppu_data_read_buffer = self.buses.read(addr);

        // Finally, increment the address by the value specified in PPUCTRL
        // and store the new value back into the internal v register.
        let addr_incr = self.registers.ppu_ctrl.get_vram_addr_incr() as u16;
        let new_addr = addr.wrapping_add(addr_incr);
        self.registers.internal.v = split_u16!(new_addr);

        data
    }

    pub fn write_ppu_data(&mut self, data: u8) {
        self.registers.ppu_data.data = data;
    }

    pub fn read_oam_dma(&self) -> u8 {
        panic!("invalid red: OAMDMA is write-only")
    }

    pub fn write_oam_dma(&mut self, data: u8) {
        self.registers.oam_dma.data = data;
    }
}
