use crate::{
    concat_u8,
    emu::ppu::{buses::Buses, registers::Registers},
    split_u16,
};

pub mod buses;
pub mod registers;

#[derive(Default)]
pub struct PPU {
    registers: Registers,
    buses: Buses,
    ppu_data_read_buffer: u8,
}

impl PPU {
    pub fn ppu_ctrl_write(&mut self, data: u8) {
        self.registers.ppu_ctrl.data = data;
    }

    pub fn ppu_addr_write(&mut self, data: u8) {
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

    pub fn ppu_data_read(&mut self, addr: u16) -> u8 {
        let data = self.ppu_data_read_buffer;
        self.ppu_data_read_buffer = self.buses.read(addr);

        let old_vram_addr = concat_u8!(self.registers.internal.v.0, self.registers.internal.v.1);
        let vram_addr_inc = self.registers.ppu_ctrl.get_vram_addr_incr() as u16;
        let new_vram_addr = old_vram_addr.wrapping_add(vram_addr_inc);
        self.registers.internal.v = split_u16!(new_vram_addr);

        data
    }

    pub fn ppu_data_write(&mut self, data: u8) {
        self.registers.ppu_data.data = data;
    }
}
