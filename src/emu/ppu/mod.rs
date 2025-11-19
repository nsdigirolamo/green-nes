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

const OAM_SPRITE_SIZE: usize = 4;
const OAM_SPRITE_COUNT: u32 = 64;
const OAM_SIZE: usize = OAM_SPRITE_SIZE * OAM_SPRITE_COUNT as usize;

#[derive(Clone)]
pub struct PPU {
    registers: Registers,
    buses: Buses,
    oam: [u8; OAM_SIZE],
    ppu_data_read_buffer: u8,
    nmi: bool,
}

impl PPU {
    pub fn new(cart: Cartridge) -> Self {
        PPU {
            registers: Registers::default(),
            buses: Buses::new(cart),
            oam: [0; OAM_SIZE],
            ppu_data_read_buffer: 0,
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
        let data = self.registers.ppu_status.data;

        self.registers.internal.w = false;
        self.registers.ppu_status.set_vblank_flag(false);

        data
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
        let addr = self.registers.oam_addr.data;

        self.oam[addr as usize]
    }

    pub fn write_oam_data(&mut self, data: u8) {
        self.registers.oam_data.data = data;
        self.registers.oam_addr.data += 1;
    }

    pub fn read_ppu_scroll(&self) -> u8 {
        panic!("invalid read: PPUSCROLL is write-only")
    }

    pub fn write_ppu_scroll(&mut self, data: u8) {
        let is_first_write = !self.registers.internal.w;

        if is_first_write {
            self.registers.internal.t.0 = data;
        } else {
            self.registers.internal.t.1 = data;
            self.registers.internal.v = self.registers.internal.t;
        }

        self.registers.internal.w = !self.registers.internal.w;
    }

    pub fn read_ppu_addr(&self) -> u8 {
        panic!("invalid read: PPUADDR is write-only")
    }

    pub fn write_ppu_addr(&mut self, data: u8) {
        let is_first_write = !self.registers.internal.w;

        if is_first_write {
            self.registers.internal.t.0 = data;
        } else {
            self.registers.internal.t.1 = data;
            self.registers.internal.v = self.registers.internal.t;
        }

        self.registers.internal.w = !self.registers.internal.w;
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
        let addr_incr = self.registers.ppu_ctrl.get_vram_addr_incr();
        let new_addr = addr.wrapping_add(addr_incr);
        self.registers.internal.v = split_u16!(new_addr);

        data
    }

    pub fn write_ppu_data(&mut self, _data: u8) {
        todo!("write to PPUDATA is not implemented")
    }

    pub fn read_oam_dma(&self) -> u8 {
        panic!("invalid red: OAMDMA is write-only")
    }

    pub fn write_oam_dma(&mut self, _data: u8) {
        todo!("write to OAMDMA is not implemented")
    }
}
